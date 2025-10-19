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
    Application, BasicType as AstBasicType, BinaryOp, Expr, FunctionDef, IfExpr, Literal,
    LiteralValue, MatchExpr, Pattern, Program, Type, Variable,
};

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        CodeGen {
            context,
            module,
            builder,
        }
    }

    pub fn compile_program(&mut self, program: &Program) -> Result<(), String> {
        for function in &program.func_defs {
            self.compile_function(function)?;
        }
        Ok(())
    }

    fn compile_function(&mut self, function: &FunctionDef) -> Result<FunctionValue<'ctx>, String> {
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

        let body_value = self.compile_expr(&function.body, fn_value)?;

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
            Expr::Application(app) => {
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
            _ => Err(format!("Unsupported literal: {:?}", lit)),
        }
    }

    fn compile_variable(
        &self,
        name: &str,
        function: FunctionValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        for (i, param) in function.get_param_iter().enumerate() {
            if let Ok(param_name) = function.get_nth_param(i as u32).unwrap().get_name().to_str() {
                if param_name == name {
                    return Ok(param);
                }
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

        let scrutinee_value = self.compile_expr(&match_expr.scrutinee, function)?.into_int_value();

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

                    let pattern_value = self.context.i64_type().const_int(*n as u64, true);
                    let cond = self
                        .builder
                        .build_int_compare(IntPredicate::EQ, scrutinee_value, pattern_value, "cond")
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
                _ => panic!("Unsupported basic type: {}", name),
            },
            _ => panic!("Unsupported type: {:?}", ty),
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
