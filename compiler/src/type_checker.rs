use std::collections::HashMap;
use crate::ast::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeError {
    pub message: String,
    pub location: Option<String>,
}

impl TypeError {
    pub fn new(message: impl Into<String>) -> Self {
        TypeError {
            message: message.into(),
            location: None,
        }
    }
    
    pub fn with_location(message: impl Into<String>, location: impl Into<String>) -> Self {
        TypeError {
            message: message.into(),
            location: Some(location.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeCheckResult {
    pub success: bool,
    pub errors: Vec<TypeError>,
}

impl TypeCheckResult {
    pub fn success() -> Self {
        TypeCheckResult {
            success: true,
            errors: vec![],
        }
    }
    
    pub fn failure(errors: Vec<TypeError>) -> Self {
        TypeCheckResult {
            success: false,
            errors,
        }
    }
}

pub struct TypeChecker {
    program: Program,
    type_env: HashMap<String, Type>,
    function_types: HashMap<String, FunctionType>,
    custom_types: HashMap<String, TypeDef>,
    errors: Vec<TypeError>,
}

impl TypeChecker {
    pub fn new(program: Program) -> Self {
        let mut checker = TypeChecker {
            program,
            type_env: HashMap::new(),
            function_types: HashMap::new(),
            custom_types: HashMap::new(),
            errors: Vec::new(),
        };
        
        checker.initialize_builtins();
        checker.collect_type_definitions();
        checker.collect_function_signatures();
        
        checker
    }
    
    fn initialize_builtins(&mut self) {
        // Helper to create a polymorphic type variable (we'll use "Unknown" to mean "any type")
        // In a real implementation, we'd need proper type variable support
        
        // List operations - curried form
        // list_get: List<T> -> Int -> T -> T
        // First application: List<T> -> (Int -> (T -> T))
        let list_get_type = FunctionType {
            param_type: Box::new(Type::List(ListType {
                element_type: Box::new(Type::Basic(BasicType { name: "Unknown".to_string() })),
            })),
            return_type: Box::new(Type::Function(FunctionType {
                param_type: Box::new(Type::Basic(BasicType { name: "Int".to_string() })),
                return_type: Box::new(Type::Function(FunctionType {
                    param_type: Box::new(Type::Basic(BasicType { name: "Unknown".to_string() })),
                    return_type: Box::new(Type::Basic(BasicType { name: "Unknown".to_string() })),
                    effect: None,
                })),
                effect: None,
            })),
            effect: None,
        };
        self.function_types.insert("list_get".to_string(), list_get_type);

        // list_set: List<T> -> Int -> T -> List<T>
        let list_set_type = FunctionType {
            param_type: Box::new(Type::List(ListType {
                element_type: Box::new(Type::Basic(BasicType { name: "Unknown".to_string() })),
            })),
            return_type: Box::new(Type::Function(FunctionType {
                param_type: Box::new(Type::Basic(BasicType { name: "Int".to_string() })),
                return_type: Box::new(Type::Function(FunctionType {
                    param_type: Box::new(Type::Basic(BasicType { name: "Unknown".to_string() })),
                    return_type: Box::new(Type::List(ListType {
                        element_type: Box::new(Type::Basic(BasicType { name: "Unknown".to_string() })),
                    })),
                    effect: None,
                })),
                effect: None,
            })),
            effect: None,
        };
        self.function_types.insert("list_set".to_string(), list_set_type);

        // list_push: List<T> -> T -> List<T>
        let list_push_type = FunctionType {
            param_type: Box::new(Type::List(ListType {
                element_type: Box::new(Type::Basic(BasicType { name: "Unknown".to_string() })),
            })),
            return_type: Box::new(Type::Function(FunctionType {
                param_type: Box::new(Type::Basic(BasicType { name: "Unknown".to_string() })),
                return_type: Box::new(Type::List(ListType {
                    element_type: Box::new(Type::Basic(BasicType { name: "Unknown".to_string() })),
                })),
                effect: None,
            })),
            effect: None,
        };
        self.function_types.insert("list_push".to_string(), list_push_type);

        // Type conversions
        // int_to_float: Int -> Float64
        let int_to_float_type = FunctionType {
            param_type: Box::new(Type::Basic(BasicType { name: "Int".to_string() })),
            return_type: Box::new(Type::Basic(BasicType { name: "Float64".to_string() })),
            effect: None,
        };
        self.function_types.insert("int_to_float".to_string(), int_to_float_type);

        // float_to_int: Float64 -> Int
        let float_to_int_type = FunctionType {
            param_type: Box::new(Type::Basic(BasicType { name: "Float64".to_string() })),
            return_type: Box::new(Type::Basic(BasicType { name: "Int".to_string() })),
            effect: None,
        };
        self.function_types.insert("float_to_int".to_string(), float_to_int_type);
    }
    
    fn collect_type_definitions(&mut self) {
        for type_def in &self.program.type_defs {
            self.custom_types.insert(type_def.name.clone(), type_def.clone());
        }
    }
    
    fn collect_function_signatures(&mut self) {
        for func_def in &self.program.func_defs {
            let func_type = if func_def.params.is_empty() {
                FunctionType {
                    param_type: Box::new(Type::Basic(BasicType {
                        name: "Unit".to_string(),
                    })),
                    return_type: Box::new(func_def.return_type.clone()),
                    effect: None,
                }
            } else if func_def.params.len() == 1 {
                // Single parameter - simple function type
                FunctionType {
                    param_type: Box::new(func_def.params[0].1.clone()),
                    return_type: Box::new(func_def.return_type.clone()),
                    effect: None,
                }
            } else {
                // Multiple parameters - build curried function type
                // params = [(x, Int), (y, String), (z, Bool)]
                // result type = Int -> (String -> (Bool -> ReturnType))
                let mut result_type = func_def.return_type.clone();
                
                // Iterate params in reverse to build nested function types
                // Skip the first parameter - it will be the outermost param_type
                for (_, param_type) in func_def.params.iter().skip(1).rev() {
                    result_type = Type::Function(FunctionType {
                        param_type: Box::new(param_type.clone()),
                        return_type: Box::new(result_type),
                        effect: None,
                    });
                }
                
                // Now wrap with the first parameter
                FunctionType {
                    param_type: Box::new(func_def.params[0].1.clone()),
                    return_type: Box::new(result_type),
                    effect: None,
                }
            };
            
            self.function_types.insert(func_def.name.clone(), func_type);
        }
    }
    
    pub fn check(mut self) -> TypeCheckResult {
        self.errors.clear();
        
        for func_def in self.program.func_defs.clone() {
            self.check_function(&func_def);
        }
        
        if self.errors.is_empty() {
            TypeCheckResult::success()
        } else {
            TypeCheckResult::failure(self.errors)
        }
    }
    
    fn check_function(&mut self, func_def: &FunctionDef) {
        let mut local_env = self.type_env.clone();
        
        for (param_name, param_type) in &func_def.params {
            local_env.insert(param_name.clone(), param_type.clone());
        }
        
        let old_env = std::mem::replace(&mut self.type_env, local_env);
        
        let body_type = self.infer_type(&func_def.body);
        
        if !self.types_compatible(&body_type, &func_def.return_type) {
            self.errors.push(TypeError::with_location(
                format!(
                    "Function '{}' body type {} does not match declared return type {}",
                    func_def.name,
                    self.type_to_string(&body_type),
                    self.type_to_string(&func_def.return_type)
                ),
                func_def.name.clone(),
            ));
        }
        
        self.type_env = old_env;
    }
    
    fn infer_type(&mut self, expr: &Expr) -> Type {
        match expr {
            Expr::Literal(lit) => self.literal_type(lit),
            
            Expr::Variable(var) => {
                if let Some(t) = self.type_env.get(&var.name) {
                    t.clone()
                } else if let Some(func_type) = self.function_types.get(&var.name) {
                    Type::Function(func_type.clone())
                } else {
                    self.errors.push(TypeError::with_location(
                        format!("Undefined variable '{}'", var.name),
                        var.name.clone(),
                    ));
                    Type::Basic(BasicType { name: "Unknown".to_string() })
                }
            }
            
            Expr::BinaryOp(binop) => {
                let left_type = self.infer_type(&binop.left);
                let right_type = self.infer_type(&binop.right);
                
                match binop.op.as_str() {
                    "+" | "-" | "*" | "/" => {
                        if self.is_numeric_type(&left_type) && self.is_numeric_type(&right_type) {
                            left_type
                        } else {
                            self.errors.push(TypeError::new(format!(
                                "Binary operator '{}' requires numeric types, got {} and {}",
                                binop.op,
                                self.type_to_string(&left_type),
                                self.type_to_string(&right_type)
                            )));
                            Type::Basic(BasicType { name: "Unknown".to_string() })
                        }
                    }
                    "==" | "!=" | "<" | ">" | "<=" | ">=" | "and" | "or" | "=>" => {
                        Type::Basic(BasicType { name: "Bool".to_string() })
                    }
                    _ => Type::Basic(BasicType { name: "Unknown".to_string() }),
                }
            }
            
            Expr::UnaryOp(unop) => {
                let operand_type = self.infer_type(&unop.operand);
                
                match unop.op.as_str() {
                    "-" => {
                        if self.is_numeric_type(&operand_type) {
                            operand_type
                        } else {
                            self.errors.push(TypeError::new(format!(
                                "Unary operator '-' requires numeric type, got {}",
                                self.type_to_string(&operand_type)
                            )));
                            Type::Basic(BasicType { name: "Unknown".to_string() })
                        }
                    }
                    "not" => Type::Basic(BasicType { name: "Bool".to_string() }),
                    _ => Type::Basic(BasicType { name: "Unknown".to_string() }),
                }
            }
            
            Expr::If(if_expr) => {
                let cond_type = self.infer_type(&if_expr.condition);
                
                let bool_type = Type::Basic(BasicType { name: "Bool".to_string() });
                if !self.types_compatible(&cond_type, &bool_type) {
                    self.errors.push(TypeError::new(format!(
                        "If condition must be Bool, got {}",
                        self.type_to_string(&cond_type)
                    )));
                }
                
                let then_type = self.infer_type(&if_expr.then_branch);
                let else_type = self.infer_type(&if_expr.else_branch);
                
                if !self.types_compatible(&then_type, &else_type) {
                    self.errors.push(TypeError::new(format!(
                        "If branches have incompatible types: {} and {}",
                        self.type_to_string(&then_type),
                        self.type_to_string(&else_type)
                    )));
                }
                
                then_type
            }
            
            Expr::Let(let_expr) => {
                let value_type = self.infer_type(&let_expr.value);
                
                let old_env = self.type_env.clone();
                self.type_env.insert(let_expr.var_name.clone(), value_type);
                
                let body_type = self.infer_type(&let_expr.body);
                
                self.type_env = old_env;
                
                body_type
            }
            
            Expr::Match(match_expr) => {
                let _scrutinee_type = self.infer_type(&match_expr.scrutinee);
                
                if match_expr.arms.is_empty() {
                    self.errors.push(TypeError::new("Match expression must have at least one arm"));
                    return Type::Basic(BasicType { name: "Unknown".to_string() });
                }
                
                let mut first_arm_type: Option<Type> = None;
                
                for (_pattern, body) in &match_expr.arms {
                    let arm_type = self.infer_type(body);
                    
                    match &first_arm_type {
                        None => first_arm_type = Some(arm_type),
                        Some(first_type) => {
                            if !self.types_compatible(&arm_type, first_type) {
                                self.errors.push(TypeError::new(format!(
                                    "Match arms have incompatible types: {} and {}",
                                    self.type_to_string(first_type),
                                    self.type_to_string(&arm_type)
                                )));
                            }
                        }
                    }
                }
                
                first_arm_type.unwrap_or(Type::Basic(BasicType { name: "Unknown".to_string() }))
            }
            
            Expr::Application(app) => {
                let func_type = self.infer_type(&app.func);
                let arg_type = self.infer_type(&app.arg);
                
                if let Type::Function(ft) = func_type {
                    if !self.types_compatible(&arg_type, &ft.param_type) {
                        self.errors.push(TypeError::new(format!(
                            "Function argument type mismatch: expected {}, got {}",
                            self.type_to_string(&ft.param_type),
                            self.type_to_string(&arg_type)
                        )));
                    }
                    *ft.return_type
                } else {
                    self.errors.push(TypeError::new(format!(
                        "Cannot apply non-function type: {}",
                        self.type_to_string(&func_type)
                    )));
                    Type::Basic(BasicType { name: "Unknown".to_string() })
                }
            }
            
            Expr::Constructor(ctor) => {
                Type::Basic(BasicType { name: ctor.name.clone() })
            }
            
            Expr::Tuple(tuple) => {
                let element_types: Vec<Type> = tuple.elements.iter()
                    .map(|e| self.infer_type(e))
                    .collect();
                Type::Tuple(TupleType { element_types })
            }
            
            Expr::Record(record) => {
                let fields: Vec<(String, Type)> = record.fields.iter()
                    .map(|(name, expr)| (name.clone(), self.infer_type(expr)))
                    .collect();
                Type::Record(RecordType { fields })
            }
            
            Expr::FieldAccess(field_access) => {
                let record_type = self.infer_type(&field_access.record);
                let resolved_type = self.resolve_type(&record_type);
                
                if let Type::Record(rec_type) = resolved_type {
                    for (field_name, field_type) in &rec_type.fields {
                        if field_name == &field_access.field {
                            return field_type.clone();
                        }
                    }
                    self.errors.push(TypeError::new(format!(
                        "Field '{}' not found in record type",
                        field_access.field
                    )));
                    Type::Basic(BasicType { name: "Unknown".to_string() })
                } else {
                    self.errors.push(TypeError::new(format!(
                        "Cannot access field on non-record type: {}",
                        self.type_to_string(&record_type)
                    )));
                    Type::Basic(BasicType { name: "Unknown".to_string() })
                }
            }
            
            Expr::Lambda(_) => {
                Type::Basic(BasicType { name: "Function".to_string() })
            }
        }
    }
    
    fn literal_type(&self, literal: &Literal) -> Type {
        let type_name = match &literal.type_name.as_str() {
            &"Int" => "Int",
            &"Nat" => "Nat",
            &"Float64" => "Float64",
            &"Bool" => "Bool",
            &"String" => "String",
            &"Unit" => "Unit",
            _ => "Unknown",
        };
        
        Type::Basic(BasicType { name: type_name.to_string() })
    }
    
    fn resolve_type(&self, t: &Type) -> Type {
        match t {
            Type::Basic(basic) => {
                // Try to resolve custom type names to their definitions
                if let Some(type_def) = self.custom_types.get(&basic.name) {
                    match &type_def.definition {
                        TypeDefKind::Record(rec_type) => {
                            Type::Record(rec_type.clone())
                        }
                        TypeDefKind::Variant(_variants) => {
                            // Variants are kept as Basic types with the variant name
                            // The actual variant checking happens during pattern matching
                            t.clone()
                        }
                        TypeDefKind::Alias(aliased_type) => {
                            // Recursively resolve aliases
                            self.resolve_type(aliased_type)
                        }
                    }
                } else {
                    t.clone()
                }
            }
            _ => t.clone(),
        }
    }
    
    fn is_numeric_type(&self, t: &Type) -> bool {
        if let Type::Basic(basic) = t {
            matches!(basic.name.as_str(), "Int" | "Nat" | "Float64")
        } else {
            false
        }
    }
    
    fn types_compatible(&self, t1: &Type, t2: &Type) -> bool {
        match (t1, t2) {
            (Type::Basic(b1), Type::Basic(b2)) => {
                if b1.name == "Unknown" || b2.name == "Unknown" {
                    return true;
                }
                if (b1.name == "Nat" && b2.name == "Int") || (b1.name == "Int" && b2.name == "Nat") {
                    return true;
                }
                b1.name == b2.name
            }
            
            (Type::Option(o1), Type::Option(o2)) => {
                self.types_compatible(&o1.inner, &o2.inner)
            }
            
            (Type::Result(r1), Type::Result(r2)) => {
                self.types_compatible(&r1.ok_type, &r2.ok_type)
                    && self.types_compatible(&r1.err_type, &r2.err_type)
            }
            
            (Type::List(l1), Type::List(l2)) => {
                self.types_compatible(&l1.element_type, &l2.element_type)
            }
            
            (Type::Tuple(tu1), Type::Tuple(tu2)) => {
                if tu1.element_types.len() != tu2.element_types.len() {
                    return false;
                }
                tu1.element_types.iter()
                    .zip(&tu2.element_types)
                    .all(|(e1, e2)| self.types_compatible(e1, e2))
            }
            
            (Type::Function(f1), Type::Function(f2)) => {
                self.types_compatible(&f1.param_type, &f2.param_type)
                    && self.types_compatible(&f1.return_type, &f2.return_type)
            }
            
            _ => false,
        }
    }
    
    fn type_to_string(&self, t: &Type) -> String {
        match t {
            Type::Basic(basic) => basic.name.clone(),
            
            Type::Option(opt) => {
                format!("Option<{}>", self.type_to_string(&opt.inner))
            }
            
            Type::Result(res) => {
                format!("Result<{}, {}>", 
                    self.type_to_string(&res.ok_type),
                    self.type_to_string(&res.err_type))
            }
            
            Type::List(list) => {
                format!("List<{}>", self.type_to_string(&list.element_type))
            }
            
            Type::Tuple(tuple) => {
                let types: Vec<String> = tuple.element_types.iter()
                    .map(|t| self.type_to_string(t))
                    .collect();
                format!("({})", types.join(", "))
            }
            
            Type::Record(record) => {
                let fields: Vec<String> = record.fields.iter()
                    .map(|(name, t)| format!("{}: {}", name, self.type_to_string(t)))
                    .collect();
                format!("{{{}}}", fields.join(", "))
            }
            
            Type::Function(func) => {
                format!("{} -> {}", 
                    self.type_to_string(&func.param_type),
                    self.type_to_string(&func.return_type))
            }
            
            Type::Pointer(ptr) => {
                format!("Ptr<{}>", self.type_to_string(&ptr.pointee_type))
            }
        }
    }
}

pub fn check_types(program: Program) -> TypeCheckResult {
    let checker = TypeChecker::new(program);
    checker.check()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir_parser::parse_ir;
    
    #[test]
    fn test_simple_function() {
        let ir = r#"
func add (x: Int, y: Int) -> Int :
  x + y
"#;
        let program = parse_ir(ir).unwrap();
        let result = check_types(program);
        assert!(result.success, "Type check failed: {:?}", result.errors);
    }
    
    #[test]
    fn test_factorial() {
        let ir = r#"
func factorial (n: Nat) -> Nat :
  match n with
  | 0 -> 1
  | n -> n * factorial (n - 1)
"#;
        let program = parse_ir(ir).unwrap();
        let result = check_types(program);
        assert!(result.success, "Type check failed: {:?}", result.errors);
    }
    
    #[test]
    fn test_match_expression() {
        let ir = r#"
func is_zero (x: Int) -> Bool :
  match x with
  | 0 -> true
  | _ -> false
"#;
        let program = parse_ir(ir).unwrap();
        let result = check_types(program);
        assert!(result.success, "Type check failed: {:?}", result.errors);
    }
    
    #[test]
    fn test_type_mismatch() {
        let ir = r#"
func bad () -> Int :
  true
"#;
        let program = parse_ir(ir).unwrap();
        let result = check_types(program);
        assert!(!result.success, "Should fail type check");
        assert!(!result.errors.is_empty());
    }
    
    #[test]
    fn test_record_return_type() {
        let ir = r#"
type Player = { health: Int }

func make_player() -> Player:
  { health: 100 }

func test_it() -> Int:
  make_player().health
"#;
        let program = parse_ir(ir).unwrap();
        let result = check_types(program);
        assert!(result.success, "Type check failed: {:?}", result.errors);
    }
}
