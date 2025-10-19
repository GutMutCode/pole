// AST Types for Pole IR
// Mirrors src/pole/runtime/ir_ast.py

use serde::{Deserialize, Serialize};

// ============================================================================
// Types
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    Basic(BasicType),
    Option(OptionType),
    Result(ResultType),
    List(ListType),
    Tuple(TupleType),
    Record(RecordType),
    Function(FunctionType),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicType {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptionType {
    pub inner: Box<Type>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResultType {
    pub ok_type: Box<Type>,
    pub err_type: Box<Type>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListType {
    pub element_type: Box<Type>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TupleType {
    pub element_types: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecordType {
    pub fields: Vec<(String, Type)>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionType {
    pub param_type: Box<Type>,
    pub return_type: Box<Type>,
    pub effect: Option<String>,
}

// ============================================================================
// Expressions
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    Literal(Literal),
    Variable(Variable),
    Lambda(Lambda),
    Application(Application),
    Let(LetExpr),
    If(IfExpr),
    Match(MatchExpr),
    Constructor(Constructor),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    Tuple(TupleExpr),
    Record(RecordExpr),
    FieldAccess(FieldAccess),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Literal {
    pub value: LiteralValue,
    pub type_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LiteralValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Unit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lambda {
    pub params: Vec<String>,
    pub body: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Application {
    pub func: Box<Expr>,
    pub arg: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LetExpr {
    pub var_name: String,
    pub value: Box<Expr>,
    pub body: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfExpr {
    pub condition: Box<Expr>,
    pub then_branch: Box<Expr>,
    pub else_branch: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchExpr {
    pub scrutinee: Box<Expr>,
    pub arms: Vec<(Pattern, Expr)>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Constructor {
    pub name: String,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryOp {
    pub op: String,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnaryOp {
    pub op: String,
    pub operand: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TupleExpr {
    pub elements: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecordExpr {
    pub fields: Vec<(String, Expr)>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldAccess {
    pub record: Box<Expr>,
    pub field: String,
}

// ============================================================================
// Patterns
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Pattern {
    Literal(LiteralPattern),
    Variable(VariablePattern),
    Constructor(ConstructorPattern),
    Tuple(TuplePattern),
    Record(RecordPattern),
    Wildcard(WildcardPattern),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiteralPattern {
    pub value: LiteralValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariablePattern {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstructorPattern {
    pub name: String,
    pub args: Vec<Pattern>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TuplePattern {
    pub elements: Vec<Pattern>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecordPattern {
    pub fields: Vec<(String, Pattern)>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WildcardPattern;

// ============================================================================
// Program Structure
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Annotation {
    pub name: String,
    pub args: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeDef {
    pub name: String,
    pub definition: TypeDefKind,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeDefKind {
    Alias(Type),
    Variant(Vec<(String, Vec<Type>)>),
    Record(RecordType),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub return_type: Type,
    pub requires: Vec<Expr>,
    pub ensures: Vec<Expr>,
    pub body: Expr,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternFunctionDecl {
    pub name: String,
    pub c_name: String,
    pub params: Vec<(String, Type)>,
    pub return_type: Type,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub type_defs: Vec<TypeDef>,
    pub func_defs: Vec<FunctionDef>,
    pub extern_funcs: Vec<ExternFunctionDecl>,
}
