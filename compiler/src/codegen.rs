use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};
use inkwell::types::{BasicMetadataTypeEnum, BasicType as LLVMBasicType, BasicTypeEnum};
use inkwell::values::{BasicValueEnum, FunctionValue};
use inkwell::OptimizationLevel;
use inkwell::IntPredicate;
use std::path::Path;

use crate::ast::{
    Application, BasicType as AstBasicType, BinaryOp, Constructor, Expr, FieldAccess, FunctionDef,
    IfExpr, LetExpr, Literal, LiteralValue, MatchExpr, Pattern, Program, RecordExpr, RecordType,
    Type, TypeDefKind, Variable,
};

use std::collections::HashMap;
use bumpalo::Bump;

pub struct CodeGen<'ctx, 'arena> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    arena: &'arena Bump,
    type_defs: HashMap<String, RecordType>,
    variant_defs: HashMap<String, Vec<(String, Vec<Type>)>>,
    local_vars: HashMap<String, BasicValueEnum<'ctx>>,
    var_types: HashMap<String, Type>,
    current_function_return_type: Option<Type>,
}

impl<'ctx, 'arena> CodeGen<'ctx, 'arena> {
    pub fn new(context: &'ctx Context, module_name: &str, arena: &'arena Bump) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        CodeGen {
            context,
            module,
            builder,
            arena,
            type_defs: HashMap::new(),
            variant_defs: HashMap::new(),
            local_vars: HashMap::new(),
            var_types: HashMap::new(),
            current_function_return_type: None,
        }
    }
    
    fn alloc_temp<T>(&self, value: T) -> &'arena T {
        self.arena.alloc(value)
    }

    pub fn compile_program(&mut self, program: &Program) -> Result<(), String> {
        for type_def in &program.type_defs {
            match &type_def.definition {
                TypeDefKind::Record(record_type) => {
                    self.type_defs.insert(type_def.name.clone(), record_type.clone());
                }
                TypeDefKind::Variant(variants) => {
                    self.variant_defs.insert(type_def.name.clone(), variants.clone());
                }
                _ => {}
            }
        }
        
        for function in &program.func_defs {
            self.compile_function(function)?;
        }
        Ok(())
    }

    fn compile_function(&mut self, function: &FunctionDef) -> Result<FunctionValue<'ctx>, String> {
        self.var_types.clear();
        self.local_vars.clear();
        
        for (param_name, param_type) in &function.params {
            self.var_types.insert(param_name.clone(), param_type.clone());
        }
        
        let param_types: Vec<BasicMetadataTypeEnum> = function
            .params
            .iter()
            .map(|(_, ty)| self.compile_type(ty).into())
            .collect();

        let return_type = self.compile_type(&function.return_type);

        let fn_type = return_type.fn_type(&param_types, false);
        let fn_value = self.module.add_function(&function.name, fn_type, None);

        for (i, (param_name, _)) in function.params.iter().enumerate() {
            fn_value.get_nth_param(i as u32).unwrap().set_name(param_name);
        }

        let entry_bb = self.context.append_basic_block(fn_value, "entry");
        self.builder.position_at_end(entry_bb);

        self.current_function_return_type = Some(function.return_type.clone());
        let body_value = self.compile_expr(&function.body, fn_value)?;
        self.current_function_return_type = None;

        self.builder.build_return(Some(&body_value)).unwrap();

        if fn_value.verify(true) {
            Ok(fn_value)
        } else {
            Err(format!("Function verification failed: {}", function.name))
        }
    }

    fn compile_expr(
        &mut self,
        expr: &Expr,
        function: FunctionValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        match expr {
            Expr::Literal(lit) => self.compile_literal(lit),
            Expr::Variable(var) => self.compile_variable(&var.name, function),
            Expr::BinaryOp(binop) => self.compile_binary_op(binop, function),
            Expr::If(if_expr) => self.compile_if(if_expr, function),
            Expr::Match(match_expr) => self.compile_match(match_expr, function),
            Expr::Let(let_expr) => self.compile_let(let_expr, function),
            Expr::FieldAccess(field_access) => self.compile_field_access(field_access, function),
            Expr::Record(record_expr) => self.compile_record(record_expr, function),
            Expr::Constructor(constructor) => self.compile_constructor(constructor, function),
            Expr::Application(app) => {
                // Check if this is a builtin function or constructor
                if let Expr::Variable(var) = &*app.func {
                    match var.name.as_str() {
                        "String_length" | "String.length" => {
                            // String.length: String -> Nat
                            // String is { i8*, i64 }, extract field 1 (length)
                            let string_val = self.compile_expr(&app.arg, function)?;
                            let string_struct = string_val.into_struct_value();
                            let length = self.builder
                                .build_extract_value(string_struct, 1, "length")
                                .unwrap();
                            return Ok(length);
                        }
                        "Some" => {
                            // Some(x) -> { i32 1, T x }
                            let value = self.compile_expr(&app.arg, function)?;
                            let i32_type = self.context.i32_type();
                            let tag = i32_type.const_int(1, false);
                            
                            let option_type = self.context.struct_type(
                                &[i32_type.into(), value.get_type()],
                                false
                            );
                            
                            let mut option_val = option_type.get_undef();
                            option_val = self.builder.build_insert_value(option_val, tag, 0, "tag").unwrap().into_struct_value();
                            option_val = self.builder.build_insert_value(option_val, value, 1, "value").unwrap().into_struct_value();
                            
                            return Ok(option_val.into());
                        }
                        "Ok" => {
                            // Ok(x) -> { i32 1, T x }
                            let value = self.compile_expr(&app.arg, function)?;
                            let i32_type = self.context.i32_type();
                            let tag = i32_type.const_int(1, false);
                            
                            let result_type = self.context.struct_type(
                                &[i32_type.into(), value.get_type()],
                                false
                            );
                            
                            let mut result_val = result_type.get_undef();
                            result_val = self.builder.build_insert_value(result_val, tag, 0, "tag").unwrap().into_struct_value();
                            result_val = self.builder.build_insert_value(result_val, value, 1, "value").unwrap().into_struct_value();
                            
                            return Ok(result_val.into());
                        }
                        "Err" => {
                            // Err(e) -> { i32 0, E e }
                            let value = self.compile_expr(&app.arg, function)?;
                            let i32_type = self.context.i32_type();
                            let tag = i32_type.const_int(0, false);
                            
                            let result_type = self.context.struct_type(
                                &[i32_type.into(), value.get_type()],
                                false
                            );
                            
                            let mut result_val = result_type.get_undef();
                            result_val = self.builder.build_insert_value(result_val, tag, 0, "tag").unwrap().into_struct_value();
                            result_val = self.builder.build_insert_value(result_val, value, 1, "value").unwrap().into_struct_value();
                            
                            return Ok(result_val.into());
                        }
                        _ => {
                            // Check if it's a builtin before falling through
                        }
                    }
                }
                
                // Regular function call
                // Collect all args from nested Applications
                let (func_name, args) = self.flatten_application(app)?;
                
                let arg_values: Vec<BasicValueEnum> = args
                    .iter()
                    .map(|arg_expr| self.compile_expr(arg_expr, function))
                    .collect::<Result<Vec<_>, _>>()?;
                
                let callee = self
                    .module
                    .get_function(&func_name)
                    .ok_or_else(|| format!("Function '{}' not found", func_name))?;

                let arg_metadata: Vec<_> = arg_values.iter().map(|v| (*v).into()).collect();

                let call_site = self
                    .builder
                    .build_call(callee, &arg_metadata, "call")
                    .unwrap();

                call_site
                    .try_as_basic_value()
                    .left()
                    .ok_or_else(|| format!("Function '{}' returned void", func_name))
            }
            _ => Err(format!("Unsupported expression: {:?}", expr)),
        }
    }

    fn compile_literal(&self, lit: &Literal) -> Result<BasicValueEnum<'ctx>, String> {
        match &lit.value {
            LiteralValue::Int(n) => {
                let i64_type = self.context.i64_type();
                Ok(i64_type.const_int(*n as u64, true).into())
            }
            LiteralValue::Bool(b) => {
                let i1_type = self.context.bool_type();
                Ok(i1_type.const_int(*b as u64, false).into())
            }
            LiteralValue::Float(f) => {
                let f64_type = self.context.f64_type();
                Ok(f64_type.const_float(*f).into())
            }
            LiteralValue::String(s) => {
                // Create a global string constant
                let global_string = self.builder.build_global_string_ptr(s, "str").unwrap();
                let i8_ptr = global_string.as_pointer_value();
                let length = self.context.i64_type().const_int(s.len() as u64, false);
                
                // Build String struct { i8*, i64 }
                let i8_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
                let i64_type = self.context.i64_type();
                let string_type = self.context.struct_type(&[i8_ptr_type.into(), i64_type.into()], false);
                
                let string_val = string_type.const_named_struct(&[i8_ptr.into(), length.into()]);
                Ok(string_val.into())
            }
            LiteralValue::Unit => {
                // Unit is represented as i8 0
                let i8_type = self.context.i8_type();
                Ok(i8_type.const_int(0, false).into())
            }
        }
    }

    fn compile_variable(
        &self,
        name: &str,
        function: FunctionValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        if let Some(&value) = self.local_vars.get(name) {
            return Ok(value);
        }
        
        for (i, param) in function.get_param_iter().enumerate() {
            if let Ok(param_name) = function.get_nth_param(i as u32).unwrap().get_name().to_str() {
                if param_name == name {
                    return Ok(param);
                }
            }
        }
        
        // Check if it's a variant constructor
        for (_variant_name, constructors) in &self.variant_defs {
            for (idx, (ctor_name, ctor_args)) in constructors.iter().enumerate() {
                if ctor_name == name && ctor_args.is_empty() {
                    // Simple enum constructor (no arguments)
                    // Represent as i32 tag
                    let i32_type = self.context.i32_type();
                    let tag_value = i32_type.const_int(idx as u64, false);
                    return Ok(tag_value.into());
                }
            }
        }
        
        // Check if it's None (Option type)
        if name == "None" {
            if let Some(Type::Option(option_type)) = &self.current_function_return_type {
                // None -> { i32 0, T undef }
                let i32_type = self.context.i32_type();
                let tag = i32_type.const_int(0, false);
                let inner_type = self.compile_type(&option_type.inner);
                
                let option_struct_type = self.context.struct_type(
                    &[i32_type.into(), inner_type],
                    false
                );
                
                let mut option_val = option_struct_type.get_undef();
                option_val = self.builder.build_insert_value(option_val, tag, 0, "tag").unwrap().into_struct_value();
                
                return Ok(option_val.into());
            }
        }

        Err(format!("Variable '{}' not found", name))
    }

    fn compile_binary_op(
        &mut self,
        binop: &BinaryOp,
        function: FunctionValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let lhs = self.compile_expr(&binop.left, function)?.into_int_value();
        let rhs = self.compile_expr(&binop.right, function)?.into_int_value();

        let result = match binop.op.as_str() {
            "+" => self.builder.build_int_add(lhs, rhs, "add").unwrap(),
            "-" => self.builder.build_int_sub(lhs, rhs, "sub").unwrap(),
            "*" => self.builder.build_int_mul(lhs, rhs, "mul").unwrap(),
            "/" => self.builder.build_int_signed_div(lhs, rhs, "div").unwrap(),
            "==" => self
                .builder
                .build_int_compare(IntPredicate::EQ, lhs, rhs, "eq")
                .unwrap(),
            "!=" => self
                .builder
                .build_int_compare(IntPredicate::NE, lhs, rhs, "ne")
                .unwrap(),
            "<" => self
                .builder
                .build_int_compare(IntPredicate::SLT, lhs, rhs, "lt")
                .unwrap(),
            "<=" => self
                .builder
                .build_int_compare(IntPredicate::SLE, lhs, rhs, "le")
                .unwrap(),
            ">" => self
                .builder
                .build_int_compare(IntPredicate::SGT, lhs, rhs, "gt")
                .unwrap(),
            ">=" => self
                .builder
                .build_int_compare(IntPredicate::SGE, lhs, rhs, "ge")
                .unwrap(),
            _ => return Err(format!("Unsupported binary operator: {}", binop.op)),
        };

        Ok(result.into())
    }

    fn compile_if(
        &mut self,
        if_expr: &IfExpr,
        function: FunctionValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let cond_value = self.compile_expr(&if_expr.condition, function)?.into_int_value();

        let then_bb = self.context.append_basic_block(function, "then");
        let else_bb = self.context.append_basic_block(function, "else");
        let merge_bb = self.context.append_basic_block(function, "merge");

        self.builder
            .build_conditional_branch(cond_value, then_bb, else_bb)
            .unwrap();

        self.builder.position_at_end(then_bb);
        let then_value = self.compile_expr(&if_expr.then_branch, function)?;
        self.builder.build_unconditional_branch(merge_bb).unwrap();
        let then_bb_end = self.builder.get_insert_block().unwrap();

        self.builder.position_at_end(else_bb);
        let else_value = self.compile_expr(&if_expr.else_branch, function)?;
        self.builder.build_unconditional_branch(merge_bb).unwrap();
        let else_bb_end = self.builder.get_insert_block().unwrap();

        self.builder.position_at_end(merge_bb);
        let phi = self
            .builder
            .build_phi(then_value.get_type(), "result")
            .unwrap();
        phi.add_incoming(&[(&then_value, then_bb_end), (&else_value, else_bb_end)]);

        Ok(phi.as_basic_value())
    }

    fn compile_match(
        &mut self,
        match_expr: &MatchExpr,
        function: FunctionValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let arms = &match_expr.arms;
        
        if arms.is_empty() {
            return Err("Match expression must have at least one arm".to_string());
        }

        let scrutinee_value = self.compile_expr(&match_expr.scrutinee, function)?;

        if arms.len() == 1 {
            return self.compile_expr(&arms[0].1, function);
        }

        let (first_pattern, first_expr) = &arms[0];
        let rest_arms = &arms[1..];

        match first_pattern {
            Pattern::Literal(lit) => {
                if let LiteralValue::Int(n) = &lit.value {
                    let match_bb = self.context.append_basic_block(function, "match_case");
                    let next_bb = self.context.append_basic_block(function, "match_next");

                    let scrutinee_int = scrutinee_value.into_int_value();
                    let pattern_value = self.context.i64_type().const_int(*n as u64, true);
                    let cond = self
                        .builder
                        .build_int_compare(IntPredicate::EQ, scrutinee_int, pattern_value, "cond")
                        .unwrap();

                    self.builder
                        .build_conditional_branch(cond, match_bb, next_bb)
                        .unwrap();

                    self.builder.position_at_end(match_bb);
                    let match_value = self.compile_expr(first_expr, function)?;
                    let merge_bb = self.context.append_basic_block(function, "match_merge");
                    self.builder.build_unconditional_branch(merge_bb).unwrap();
                    let match_bb_end = self.builder.get_insert_block().unwrap();

                    self.builder.position_at_end(next_bb);
                    let rest_match = MatchExpr {
                        scrutinee: match_expr.scrutinee.clone(),
                        arms: rest_arms.to_vec(),
                    };
                    let next_value = self.compile_match(&rest_match, function)?;
                    self.builder.build_unconditional_branch(merge_bb).unwrap();
                    let next_bb_end = self.builder.get_insert_block().unwrap();

                    self.builder.position_at_end(merge_bb);
                    let phi = self
                        .builder
                        .build_phi(match_value.get_type(), "match_result")
                        .unwrap();
                    phi.add_incoming(&[(&match_value, match_bb_end), (&next_value, next_bb_end)]);

                    Ok(phi.as_basic_value())
                } else {
                    Err(format!("Unsupported pattern literal: {:?}", lit))
                }
            }
            Pattern::Variable(_) => {
                self.compile_expr(first_expr, function)
            }
            Pattern::Constructor(ctor_pattern) => {
                // Handle Option/Result constructor patterns
                match ctor_pattern.name.as_str() {
                    "Some" | "Ok" => {
                        // Extract tag from { i32 tag, T value }
                        let scrutinee_struct = scrutinee_value.into_struct_value();
                        let tag = self.builder.build_extract_value(scrutinee_struct, 0, "tag")
                            .unwrap().into_int_value();
                        
                        // Check if tag == 1 (Some/Ok)
                        let tag_one = self.context.i32_type().const_int(1, false);
                        let is_some = self.builder.build_int_compare(
                            IntPredicate::EQ, tag, tag_one, "is_some"
                        ).unwrap();
                        
                        let match_bb = self.context.append_basic_block(function, "match_some");
                        let next_bb = self.context.append_basic_block(function, "match_next");
                        
                        self.builder.build_conditional_branch(is_some, match_bb, next_bb).unwrap();
                        
                        // Some/Ok branch
                        self.builder.position_at_end(match_bb);
                        
                        // Extract value and bind to pattern variable
                        if let Some(Pattern::Variable(var_pattern)) = ctor_pattern.args.first() {
                            let value = self.builder.build_extract_value(scrutinee_struct, 1, "value")
                                .unwrap();
                            
                            let old_var = self.local_vars.insert(var_pattern.name.clone(), value);
                            let match_value = self.compile_expr(first_expr, function)?;
                            
                            // Restore old variable binding
                            if let Some(old) = old_var {
                                self.local_vars.insert(var_pattern.name.clone(), old);
                            } else {
                                self.local_vars.remove(&var_pattern.name);
                            }
                            
                            let merge_bb = self.context.append_basic_block(function, "match_merge");
                            self.builder.build_unconditional_branch(merge_bb).unwrap();
                            let match_bb_end = self.builder.get_insert_block().unwrap();
                            
                            // None/Err branch
                            self.builder.position_at_end(next_bb);
                            let rest_match = MatchExpr {
                                scrutinee: match_expr.scrutinee.clone(),
                                arms: rest_arms.to_vec(),
                            };
                            let next_value = self.compile_match(&rest_match, function)?;
                            self.builder.build_unconditional_branch(merge_bb).unwrap();
                            let next_bb_end = self.builder.get_insert_block().unwrap();
                            
                            self.builder.position_at_end(merge_bb);
                            let phi = self.builder.build_phi(match_value.get_type(), "match_result").unwrap();
                            phi.add_incoming(&[(&match_value, match_bb_end), (&next_value, next_bb_end)]);
                            
                            Ok(phi.as_basic_value())
                        } else {
                            Err("Some/Ok pattern must have exactly one variable argument".to_string())
                        }
                    }
                    "None" | "Err" => {
                        // Extract tag from { i32 tag, T value }
                        let scrutinee_struct = scrutinee_value.into_struct_value();
                        let tag = self.builder.build_extract_value(scrutinee_struct, 0, "tag")
                            .unwrap().into_int_value();
                        
                        let expected_tag = if ctor_pattern.name == "None" { 0 } else { 0 };
                        let tag_value = self.context.i32_type().const_int(expected_tag, false);
                        let is_match = self.builder.build_int_compare(
                            IntPredicate::EQ, tag, tag_value, "is_none"
                        ).unwrap();
                        
                        let match_bb = self.context.append_basic_block(function, "match_none");
                        let next_bb = self.context.append_basic_block(function, "match_next");
                        
                        self.builder.build_conditional_branch(is_match, match_bb, next_bb).unwrap();
                        
                        self.builder.position_at_end(match_bb);
                        let match_value = self.compile_expr(first_expr, function)?;
                        let merge_bb = self.context.append_basic_block(function, "match_merge");
                        self.builder.build_unconditional_branch(merge_bb).unwrap();
                        let match_bb_end = self.builder.get_insert_block().unwrap();
                        
                        self.builder.position_at_end(next_bb);
                        let rest_match = MatchExpr {
                            scrutinee: match_expr.scrutinee.clone(),
                            arms: rest_arms.to_vec(),
                        };
                        let next_value = self.compile_match(&rest_match, function)?;
                        self.builder.build_unconditional_branch(merge_bb).unwrap();
                        let next_bb_end = self.builder.get_insert_block().unwrap();
                        
                        self.builder.position_at_end(merge_bb);
                        let phi = self.builder.build_phi(match_value.get_type(), "match_result").unwrap();
                        phi.add_incoming(&[(&match_value, match_bb_end), (&next_value, next_bb_end)]);
                        
                        Ok(phi.as_basic_value())
                    }
                    _ => Err(format!("Unsupported constructor pattern: {}", ctor_pattern.name))
                }
            }
            _ => Err(format!("Unsupported pattern: {:?}", first_pattern)),
        }
    }

    // Helper to flatten curried function applications
    // factorial(n - 1) is represented as Application(Application(factorial, n), -1)
    // We need to extract the function name and all arguments
    fn flatten_application(&self, app: &Application) -> Result<(String, Vec<Expr>), String> {
        let mut args = vec![];
        let mut current = app;
        
        // Collect the innermost argument first
        args.push((*current.arg).clone());
        
        // Walk up the application chain
        let func_name = loop {
            match &*current.func {
                Expr::Variable(var) => {
                    // Found the function name
                    break var.name.clone();
                }
                Expr::Application(inner_app) => {
                    // Another application - collect its argument
                    args.push((*inner_app.arg).clone());
                    current = inner_app;
                }
                _ => {
                    return Err(format!("Expected function or application, got {:?}", current.func));
                }
            }
        };
        
        // Reverse args since we collected them inside-out
        args.reverse();
        
        Ok((func_name, args))
    }

    fn compile_type(&self, ty: &Type) -> BasicTypeEnum<'ctx> {
        match ty {
            Type::Basic(AstBasicType { name }) => match name.as_str() {
                "Int" | "Nat" => self.context.i64_type().into(),
                "Bool" => self.context.bool_type().into(),
                "Float64" => self.context.f64_type().into(),
                "Unit" => self.context.i8_type().into(),
                "String" => {
                    // String = { i8*, i64 } (data pointer + length)
                    let i8_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
                    let i64_type = self.context.i64_type();
                    self.context.struct_type(&[i8_ptr_type.into(), i64_type.into()], false).into()
                }
                type_name => {
                    if let Some(record_type) = self.type_defs.get(type_name) {
                        let field_types: Vec<BasicTypeEnum> = record_type
                            .fields
                            .iter()
                            .map(|(_, field_ty)| self.compile_type(field_ty))
                            .collect();
                        self.context.struct_type(&field_types, false).into()
                    } else if self.variant_defs.contains_key(type_name) {
                        // Variant types are represented as i32 tag (for simple enums)
                        self.context.i32_type().into()
                    } else {
                        panic!("Unsupported basic type: {}", name)
                    }
                }
            },
            Type::List(list_type) => {
                // List<T> = { T*, i64 } (element pointer + length)
                let element_type = self.compile_type(&list_type.element_type);
                let element_ptr_type = element_type.ptr_type(inkwell::AddressSpace::default());
                let i64_type = self.context.i64_type();
                self.context.struct_type(&[element_ptr_type.into(), i64_type.into()], false).into()
            }
            Type::Option(option_type) => {
                // Option<T> = { i32 tag, T value }
                // tag: 0 = None, 1 = Some
                let i32_type = self.context.i32_type();
                let inner_type = self.compile_type(&option_type.inner);
                self.context.struct_type(&[i32_type.into(), inner_type], false).into()
            }
            Type::Result(result_type) => {
                // Result<T, E> = { i32 tag, union { T ok, E err } }
                // tag: 0 = Err, 1 = Ok
                // For now, use the larger of the two types
                let i32_type = self.context.i32_type();
                let ok_type = self.compile_type(&result_type.ok_type);
                let err_type = self.compile_type(&result_type.err_type);
                
                // Use the larger type for the union
                let ok_size = ok_type.size_of().unwrap();
                let err_size = err_type.size_of().unwrap();
                let value_type = if ok_size.get_zero_extended_constant().unwrap() >= err_size.get_zero_extended_constant().unwrap() {
                    ok_type
                } else {
                    err_type
                };
                
                self.context.struct_type(&[i32_type.into(), value_type], false).into()
            }
            _ => panic!("Unsupported type: {:?}", ty),
        }
    }
    
    fn compile_let(
        &mut self,
        let_expr: &LetExpr,
        function: FunctionValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let value = self.compile_expr(&let_expr.value, function)?;
        
        let value_type = self.infer_expr_type(&let_expr.value)?;
        
        let old_value = self.local_vars.insert(let_expr.var_name.clone(), value);
        let old_type = self.var_types.insert(let_expr.var_name.clone(), value_type);
        
        let body_result = self.compile_expr(&let_expr.body, function)?;
        
        if let Some(old) = old_value {
            self.local_vars.insert(let_expr.var_name.clone(), old);
        } else {
            self.local_vars.remove(&let_expr.var_name);
        }
        
        if let Some(old_ty) = old_type {
            self.var_types.insert(let_expr.var_name.clone(), old_ty);
        } else {
            self.var_types.remove(&let_expr.var_name);
        }
        
        Ok(body_result)
    }
    
    fn infer_expr_type(&self, expr: &Expr) -> Result<Type, String> {
        match expr {
            Expr::Literal(lit) => match &lit.value {
                LiteralValue::Int(_) => Ok(Type::Basic(AstBasicType { name: "Int".to_string() })),
                LiteralValue::Bool(_) => Ok(Type::Basic(AstBasicType { name: "Bool".to_string() })),
                LiteralValue::Float(_) => Ok(Type::Basic(AstBasicType { name: "Float64".to_string() })),
                LiteralValue::String(_) => Ok(Type::Basic(AstBasicType { name: "String".to_string() })),
                LiteralValue::Unit => Ok(Type::Basic(AstBasicType { name: "Unit".to_string() })),
            },
            Expr::Variable(var) => self.var_types.get(&var.name)
                .cloned()
                .ok_or_else(|| format!("Cannot find type for variable '{}'", var.name)),
            Expr::BinaryOp(binop) => self.infer_expr_type(&binop.left),
            Expr::FieldAccess(field_access) => {
                let record_type = self.infer_expr_type(&field_access.record)?;
                let type_name = if let Type::Basic(AstBasicType { name }) = record_type {
                    name
                } else {
                    return Err("Expected basic type for record".to_string());
                };
                
                let record_def = self.type_defs.get(&type_name)
                    .ok_or_else(|| format!("Record type '{}' not found", type_name))?;
                
                record_def.fields.iter()
                    .find(|(name, _)| name == &field_access.field)
                    .map(|(_, ty)| ty.clone())
                    .ok_or_else(|| format!("Field '{}' not found", field_access.field))
            },
            _ => Err(format!("Cannot infer type for expression: {:?}", expr)),
        }
    }
    
    fn compile_field_access(
        &mut self,
        field_access: &FieldAccess,
        function: FunctionValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let record_value = self.compile_expr(&field_access.record, function)?;
        
        let record_type = if let Expr::Variable(var) = &*field_access.record {
            self.var_types.get(&var.name).ok_or_else(|| {
                format!("Cannot find type for variable '{}'", var.name)
            })?
        } else {
            return Err("Field access only supported on variables for now".to_string());
        };
        
        let type_name = if let Type::Basic(AstBasicType { name }) = record_type {
            name
        } else {
            return Err(format!("Expected basic type for record, got {:?}", record_type));
        };
        
        let record_def = self.type_defs.get(type_name).ok_or_else(|| {
            format!("Record type '{}' not found", type_name)
        })?;
        
        let field_index = record_def
            .fields
            .iter()
            .position(|(name, _)| name == &field_access.field)
            .ok_or_else(|| {
                format!("Field '{}' not found in record '{}'", field_access.field, type_name)
            })?;
        
        let struct_value = record_value.into_struct_value();
        let field_value = self
            .builder
            .build_extract_value(struct_value, field_index as u32, &field_access.field)
            .unwrap();
        
        Ok(field_value)
    }
    
    fn compile_record(
        &mut self,
        record_expr: &RecordExpr,
        function: FunctionValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let field_values: Vec<BasicValueEnum> = record_expr
            .fields
            .iter()
            .map(|(_, expr)| self.compile_expr(expr, function))
            .collect::<Result<Vec<_>, _>>()?;
        
        let struct_type = if let Some(first_field) = record_expr.fields.first() {
            let first_value_type = field_values[0].get_type();
            let field_types: Vec<_> = field_values.iter().map(|v| v.get_type()).collect();
            self.context.struct_type(&field_types, false)
        } else {
            return Err("Empty record construction not supported".to_string());
        };
        
        let mut struct_val = struct_type.get_undef();
        for (i, field_value) in field_values.iter().enumerate() {
            struct_val = self
                .builder
                .build_insert_value(struct_val, *field_value, i as u32, "field")
                .unwrap()
                .into_struct_value();
        }
        
        Ok(struct_val.into())
    }
    
    fn compile_constructor(
        &mut self,
        constructor: &Constructor,
        function: FunctionValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        // Handle List constructor
        if constructor.name == "List" {
            let element_values: Vec<BasicValueEnum> = constructor
                .args
                .iter()
                .map(|arg| self.compile_expr(arg, function))
                .collect::<Result<Vec<_>, _>>()?;
            
            let length = element_values.len() as u64;
            
            if element_values.is_empty() {
                // Empty list: { null, 0 }
                let i64_type = self.context.i64_type();
                let i64_ptr_type = i64_type.ptr_type(inkwell::AddressSpace::default());
                let list_type = self.context.struct_type(
                    &[i64_ptr_type.into(), i64_type.into()],
                    false
                );
                
                let null_ptr = i64_ptr_type.const_null();
                let zero_len = i64_type.const_int(0, false);
                let list_val = list_type.const_named_struct(&[null_ptr.into(), zero_len.into()]);
                
                Ok(list_val.into())
            } else {
                // Non-empty list: create global array and return { ptr, length }
                let i64_type = self.context.i64_type();
                
                let const_values: Vec<_> = element_values.iter()
                    .map(|v| match v {
                        BasicValueEnum::IntValue(iv) => *iv,
                        _ => panic!("Expected int value for now"),
                    })
                    .collect();
                
                let array_val = i64_type.const_array(&const_values);
                let global_array = self.module.add_global(array_val.get_type(), None, "list_data");
                global_array.set_initializer(&array_val);
                global_array.set_constant(true);
                
                let array_ptr = global_array.as_pointer_value();
                let length_val = i64_type.const_int(length, false);
                
                let i64_ptr_type = i64_type.ptr_type(inkwell::AddressSpace::default());
                let list_type = self.context.struct_type(
                    &[i64_ptr_type.into(), i64_type.into()],
                    false
                );
                
                let list_val = list_type.const_named_struct(&[array_ptr.into(), length_val.into()]);
                Ok(list_val.into())
            }
        } else {
            Err(format!("Unknown constructor: {}", constructor.name))
        }
    }

    pub fn get_module(&self) -> &Module<'ctx> {
        &self.module
    }

    pub fn print_to_string(&self) -> String {
        self.module.print_to_string().to_string()
    }

    /// Write LLVM IR to file (.ll)
    pub fn write_ir_to_file(&self, path: &Path) -> Result<(), String> {
        self.module
            .print_to_file(path)
            .map_err(|e| format!("Failed to write LLVM IR: {}", e))
    }

    /// Write bitcode to file (.bc)
    pub fn write_bitcode_to_file(&self, path: &Path) -> Result<(), String> {
        self.module.write_bitcode_to_path(path);
        Ok(())
    }

    /// Write object file (.o)
    pub fn write_object_file(&self, path: &Path) -> Result<(), String> {
        Target::initialize_native(&InitializationConfig::default())
            .map_err(|e| format!("Failed to initialize native target: {}", e))?;

        let target_triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&target_triple)
            .map_err(|e| format!("Failed to create target: {}", e))?;

        let target_machine = target
            .create_target_machine(
                &target_triple,
                "generic",
                "",
                OptimizationLevel::Default,
                RelocMode::PIC,
                CodeModel::Default,
            )
            .ok_or_else(|| "Failed to create target machine".to_string())?;

        target_machine
            .write_to_file(&self.module, FileType::Object, path)
            .map_err(|e| format!("Failed to write object file: {}", e))
    }
}
