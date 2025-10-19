// IR Parser using nom
// Parses Pole IR (.pole-ir files) into AST

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, multispace1, space0, space1},
    combinator::{map, opt, recognize, value},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::ast::*;

type ParseResult<'a, T> = IResult<&'a str, T>;

// ============================================================================
// Helper Parsers
// ============================================================================

fn ws<'a, F, O>(parser: F) -> impl FnMut(&'a str) -> ParseResult<'a, O>
where
    F: FnMut(&'a str) -> ParseResult<'a, O>,
{
    delimited(multispace0, parser, multispace0)
}

fn identifier(input: &str) -> ParseResult<String> {
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |s: &str| s.to_string(),
    )(input)
}

fn comment(input: &str) -> ParseResult<()> {
    value(
        (),
        preceded(tag("//"), take_while(|c| c != '\n')),
    )(input)
}

fn skip_ws_and_comments(input: &str) -> ParseResult<()> {
    value(
        (),
        many0(alt((
            value((), multispace1),
            comment,
        ))),
    )(input)
}

// ============================================================================
// Type Parsers
// ============================================================================

fn parse_basic_type(input: &str) -> ParseResult<Type> {
    map(
        alt((
            tag("Int"),
            tag("Nat"),
            tag("Float64"),
            tag("Bool"),
            tag("String"),
            tag("Unit"),
            // Fallback: any identifier is a type name (for custom types)
            recognize(identifier),
        )),
        |s: &str| Type::Basic(BasicType { name: s.to_string() }),
    )(input)
}

fn parse_option_type(input: &str) -> ParseResult<Type> {
    map(
        delimited(
            tag("Option<"),
            parse_type,
            char('>'),
        ),
        |inner| Type::Option(OptionType { inner: Box::new(inner) }),
    )(input)
}

fn parse_result_type(input: &str) -> ParseResult<Type> {
    map(
        delimited(
            tag("Result<"),
            separated_pair(
                parse_type,
                ws(char(',')),
                parse_type,
            ),
            char('>'),
        ),
        |(ok, err)| Type::Result(ResultType {
            ok_type: Box::new(ok),
            err_type: Box::new(err),
        }),
    )(input)
}

fn parse_list_type(input: &str) -> ParseResult<Type> {
    map(
        delimited(
            tag("List<"),
            parse_type,
            char('>'),
        ),
        |elem| Type::List(ListType { element_type: Box::new(elem) }),
    )(input)
}

fn parse_pointer_type(input: &str) -> ParseResult<Type> {
    map(
        delimited(
            tag("Ptr<"),
            parse_type,
            char('>'),
        ),
        |pointee| Type::Pointer(PointerType { pointee_type: Box::new(pointee) }),
    )(input)
}

fn parse_tuple_type(input: &str) -> ParseResult<Type> {
    map(
        delimited(
            char('('),
            separated_list0(ws(char(',')), parse_type),
            char(')'),
        ),
        |types| Type::Tuple(TupleType { element_types: types }),
    )(input)
}

fn parse_type(input: &str) -> ParseResult<Type> {
    alt((
        parse_option_type,
        parse_result_type,
        parse_list_type,
        parse_pointer_type,
        parse_tuple_type,
        parse_basic_type,
    ))(input)
}

// ============================================================================
// Annotation Parsers
// ============================================================================

fn parse_annotation(input: &str) -> ParseResult<Annotation> {
    preceded(
        char('@'),
        map(
            pair(
                identifier,
                opt(delimited(
                    char('('),
                    take_until(")"),
                    char(')'),
                )),
            ),
            |(name, args_str)| {
                let args = if let Some(s) = args_str {
                    parse_annotation_args(s)
                } else {
                    vec![]
                };
                Annotation { name, args }
            },
        ),
    )(input)
}

fn parse_annotation_args(input: &str) -> Vec<(String, String)> {
    input.split(',')
        .enumerate()
        .filter_map(|(idx, s)| {
            let s = s.trim();
            let parts: Vec<&str> = s.splitn(2, '=').collect();
            if parts.len() == 2 {
                // key=value format
                Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
            } else if !s.is_empty() {
                // Positional argument (e.g., @extern("printf"))
                // Strip quotes if present
                let value = s.trim_matches('"').to_string();
                Some((idx.to_string(), value))
            } else {
                None
            }
        })
        .collect()
}

// ============================================================================
// Type Definition Parsers
// ============================================================================

// Multiline record: type User = {\n  name: String,\n  ...}
fn parse_multiline_record_type_def(annotations: Vec<Annotation>) -> impl FnMut(&str) -> ParseResult<TypeDef> {
    move |input: &str| {
        let (input, _) = tag("type")(input)?;
        let (input, _) = space1(input)?;
        let (input, name) = identifier(input)?;
        let (input, _) = ws(char('='))(input)?;
        let (input, _) = ws(char('{'))(input)?;
        
        let mut remaining = input;
        let mut fields = vec![];
        
        loop {
            // Skip whitespace and comments
            let (new_input, _) = skip_ws_and_comments(remaining)?;
            remaining = new_input;
            
            // Check for closing brace
            if let Ok((new_input, _)) = char::<_, nom::error::Error<&str>>('}')(remaining) {
                remaining = new_input;
                break;
            }
            
            // Parse field: name: Type,?
            let (new_input, field_name) = identifier(remaining)?;
            let (new_input, _) = ws(char(':'))(new_input)?;
            let (new_input, field_type) = parse_type(new_input)?;
            let (new_input, _) = opt(ws(char(',')))(new_input)?;
            
            fields.push((field_name, field_type));
            remaining = new_input;
        }
        
        Ok((remaining, TypeDef {
            name,
            definition: TypeDefKind::Record(RecordType { fields }),
            annotations: annotations.clone(),
        }))
    }
}

// Variant: type Error =\n  | Constructor1\n  | Constructor2
fn parse_variant_type_def(annotations: Vec<Annotation>) -> impl FnMut(&str) -> ParseResult<TypeDef> {
    move |input: &str| {
        let (input, _) = tag("type")(input)?;
        let (input, _) = space1(input)?;
        let (input, name) = identifier(input)?;
        let (input, _) = ws(char('='))(input)?;
        
        let mut remaining = input;
        let mut constructors = vec![];
        
        loop {
            // Skip whitespace
            let (new_input, _) = skip_ws_and_comments(remaining)?;
            remaining = new_input;
            
            // Check if next line starts with |
            if let Ok((new_input, _)) = char::<_, nom::error::Error<&str>>('|')(remaining) {
                let (new_input, _) = space0(new_input)?;
                let (new_input, cons_name) = identifier(new_input)?;
                
                // Check for constructor arguments
                let (new_input, args) = opt(delimited(
                    ws(char('(')),
                    separated_list0(ws(char(',')), parse_type),
                    ws(char(')')),
                ))(new_input)?;
                
                constructors.push((cons_name, args.unwrap_or_default()));
                remaining = new_input;
            } else {
                // No more constructors
                break;
            }
        }
        
        if constructors.is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)));
        }
        
        Ok((remaining, TypeDef {
            name,
            definition: TypeDefKind::Variant(constructors),
            annotations: annotations.clone(),
        }))
    }
}

// Inline: type UserId = String  or  type Point = { x: Int, y: Int }
fn parse_inline_type_def(annotations: Vec<Annotation>) -> impl FnMut(&str) -> ParseResult<TypeDef> {
    move |input: &str| {
        let (input, _) = tag("type")(input)?;
        let (input, _) = space1(input)?;
        let (input, name) = identifier(input)?;
        let (input, _) = ws(char('='))(input)?;
        
        // Check if it's an inline record
        if let Ok((new_input, _)) = char::<_, nom::error::Error<&str>>('{')(input) {
            // Parse inline record: { x: Int, y: Int }
            let (new_input, fields) = separated_list0(
                ws(char(',')),
                separated_pair(
                    identifier,
                    ws(char(':')),
                    parse_type,
                ),
            )(new_input)?;
            let (new_input, _) = ws(char('}'))(new_input)?;
            
            Ok((new_input, TypeDef {
                name,
                definition: TypeDefKind::Record(RecordType { fields }),
                annotations: annotations.clone(),
            }))
        } else {
            // Type alias
            let (input, aliased_type) = parse_type(input)?;
            
            Ok((input, TypeDef {
                name,
                definition: TypeDefKind::Alias(aliased_type),
                annotations: annotations.clone(),
            }))
        }
    }
}

// ============================================================================
// Expression Parsers  
// ============================================================================

fn parse_literal(input: &str) -> ParseResult<Expr> {
    alt((
        parse_unit_literal,
        parse_int_literal,
        parse_float_literal,
        parse_bool_literal,
        parse_string_literal,
    ))(input)
}

fn parse_unit_literal(input: &str) -> ParseResult<Expr> {
    value(
        Expr::Literal(Literal {
            value: LiteralValue::Unit,
            type_name: "Unit".to_string(),
        }),
        tuple((char('('), space0, char(')'))),
    )(input)
}

fn parse_int_literal(input: &str) -> ParseResult<Expr> {
    map(
        recognize(pair(opt(char('-')), digit1)),
        |s: &str| Expr::Literal(Literal {
            value: LiteralValue::Int(s.parse().unwrap()),
            type_name: "Int".to_string(),
        }),
    )(input)
}

fn parse_float_literal(input: &str) -> ParseResult<Expr> {
    map(
        recognize(tuple((opt(char('-')), digit1, char('.'), digit1))),
        |s: &str| Expr::Literal(Literal {
            value: LiteralValue::Float(s.parse().unwrap()),
            type_name: "Float64".to_string(),
        }),
    )(input)
}

fn parse_bool_literal(input: &str) -> ParseResult<Expr> {
    alt((
        value(
            Expr::Literal(Literal {
                value: LiteralValue::Bool(true),
                type_name: "Bool".to_string(),
            }),
            tag("true"),
        ),
        value(
            Expr::Literal(Literal {
                value: LiteralValue::Bool(false),
                type_name: "Bool".to_string(),
            }),
            tag("false"),
        ),
    ))(input)
}

fn parse_string_literal(input: &str) -> ParseResult<Expr> {
    map(
        delimited(char('"'), take_while(|c| c != '"'), char('"')),
        |s: &str| Expr::Literal(Literal {
            value: LiteralValue::String(s.to_string()),
            type_name: "String".to_string(),
        }),
    )(input)
}

fn parse_variable(input: &str) -> ParseResult<Expr> {
    map(identifier, |name| Expr::Variable(Variable { name }))(input)
}

// ============================================================================
// Pattern Parsers
// ============================================================================

fn parse_literal_pattern(input: &str) -> ParseResult<Pattern> {
    alt((
        map(
            recognize(pair(opt(char('-')), digit1)),
            |s: &str| Pattern::Literal(LiteralPattern {
                value: LiteralValue::Int(s.parse().unwrap()),
            }),
        ),
        map(
            tag("true"),
            |_| Pattern::Literal(LiteralPattern {
                value: LiteralValue::Bool(true),
            }),
        ),
        map(
            tag("false"),
            |_| Pattern::Literal(LiteralPattern {
                value: LiteralValue::Bool(false),
            }),
        ),
    ))(input)
}

fn parse_wildcard_pattern(input: &str) -> ParseResult<Pattern> {
    value(Pattern::Wildcard(WildcardPattern), char('_'))(input)
}

fn parse_variable_pattern(input: &str) -> ParseResult<Pattern> {
    map(identifier, |name| {
        Pattern::Variable(VariablePattern { name })
    })(input)
}

fn parse_constructor_pattern(input: &str) -> ParseResult<Pattern> {
    map(
        pair(
            identifier,
            opt(delimited(
                ws(char('(')),
                separated_list0(ws(char(',')), parse_pattern),
                ws(char(')')),
            )),
        ),
        |(name, args)| {
            // Check if starts with uppercase (constructor)
            if name.chars().next().unwrap().is_uppercase() {
                Pattern::Constructor(ConstructorPattern {
                    name,
                    args: args.unwrap_or_default(),
                })
            } else {
                Pattern::Variable(VariablePattern { name })
            }
        },
    )(input)
}

fn parse_pattern(input: &str) -> ParseResult<Pattern> {
    alt((
        parse_wildcard_pattern,
        parse_literal_pattern,
        parse_constructor_pattern,
        parse_variable_pattern,
    ))(input)
}

// ============================================================================
// Complex Expression Parsers
// ============================================================================

fn parse_binary_op(input: &str) -> ParseResult<Expr> {
    // Simple binary op parser - can be improved with precedence
    let (input, left) = parse_postfix_expr(input)?;
    
    let (input, op_and_right) = opt(tuple((
        ws(alt((
            tag("&&"), tag("||"),  // Logical operators (must be before single &, |)
            tag("=="), tag("!="), 
            tag("<="), tag(">="), 
            tag("<"), tag(">"),
            tag("*"), tag("/"), tag("%"),
            tag("+"), tag("-"),
            tag("=>"),
        ))),
        parse_expr,
    )))(input)?;
    
    if let Some((op, right)) = op_and_right {
        Ok((input, Expr::BinaryOp(BinaryOp {
            op: op.to_string(),
            left: Box::new(left),
            right: Box::new(right),
        })))
    } else {
        Ok((input, left))
    }
}

fn parse_application(input: &str) -> ParseResult<Expr> {
    let (input, func_name) = identifier(input)?;
    let (input, _) = ws(char('('))(input)?;
    
    // Parse comma-separated arguments
    let (input, args) = separated_list0(
        ws(char(',')),
        parse_expr,
    )(input)?;
    
    let (input, _) = ws(char(')'))(input)?;
    
    // If no arguments, this is invalid
    if args.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)));
    }
    
    // Build nested Application for curried form
    // f(x, y) becomes Application(Application(f, x), y)
    let mut expr = Expr::Variable(Variable { name: func_name });
    for arg in args {
        expr = Expr::Application(Application {
            func: Box::new(expr),
            arg: Box::new(arg),
        });
    }
    
    Ok((input, expr))
}

fn parse_match_expr(input: &str) -> ParseResult<Expr> {
    let (input, _) = ws(tag("match"))(input)?;
    let (input, scrutinee) = ws(parse_simple_expr)(input)?;
    let (input, _) = ws(tag("with"))(input)?;
    
    // Parse match arms: | pattern -> expr
    let (input, arms) = many1(preceded(
        multispace0,
        map(
            tuple((
                preceded(ws(char('|')), ws(parse_pattern)),
                preceded(ws(tag("->")), ws(parse_simple_expr)),
            )),
            |(pattern, expr)| (pattern, expr),
        ),
    ))(input)?;
    
    Ok((input, Expr::Match(MatchExpr {
        scrutinee: Box::new(scrutinee),
        arms,
    })))
}

fn parse_if_expr(input: &str) -> ParseResult<Expr> {
    let (input, _) = ws(tag("if"))(input)?;
    let (input, condition) = ws(parse_simple_expr)(input)?;
    let (input, _) = ws(tag("then"))(input)?;
    let (input, then_branch) = ws(parse_expr)(input)?;
    let (input, _) = ws(tag("else"))(input)?;
    let (input, else_branch) = ws(parse_expr)(input)?;
    
    Ok((input, Expr::If(IfExpr {
        condition: Box::new(condition),
        then_branch: Box::new(then_branch),
        else_branch: Box::new(else_branch),
    })))
}

fn parse_let_expr(input: &str) -> ParseResult<Expr> {
    let (input, _) = ws(tag("let"))(input)?;
    let (input, var_name) = ws(identifier)(input)?;
    let (input, _) = ws(char('='))(input)?;
    let (input, value) = ws(parse_simple_expr)(input)?;
    let (input, _) = ws(tag("in"))(input)?;
    let (input, body) = ws(parse_expr)(input)?;
    
    Ok((input, Expr::Let(LetExpr {
        var_name,
        value: Box::new(value),
        body: Box::new(body),
    })))
}

// Primary expressions (literals, variables, parenthesized)
fn parse_primary_expr(input: &str) -> ParseResult<Expr> {
    alt((
        parse_list_literal,
        parse_record_expr,
        parse_literal,
        parse_application,
        parse_variable,
        delimited(char('('), parse_expr, char(')')),
    ))(input)
}

// Postfix expressions (field access)
fn parse_postfix_expr(input: &str) -> ParseResult<Expr> {
    let (input, mut expr) = parse_primary_expr(input)?;
    
    let (input, fields) = many0(preceded(char('.'), identifier))(input)?;
    
    for field in fields {
        expr = Expr::FieldAccess(FieldAccess {
            record: Box::new(expr),
            field,
        });
    }
    
    Ok((input, expr))
}

// Simple expressions (no complex control flow)
fn parse_simple_expr(input: &str) -> ParseResult<Expr> {
    parse_binary_op(input)
}

// Full expression parser
fn parse_expr(input: &str) -> ParseResult<Expr> {
    alt((
        parse_match_expr,
        parse_if_expr,
        parse_let_expr,
        parse_simple_expr,
    ))(input)
}

// ============================================================================
// Function Definition Parser
// ============================================================================

fn parse_function_param(input: &str) -> ParseResult<(String, Type)> {
    separated_pair(
        identifier,
        ws(char(':')),
        parse_type,
    )(input)
}

fn parse_requires(input: &str) -> ParseResult<Expr> {
    preceded(
        ws(tag("requires")),
        ws(parse_simple_expr),
    )(input)
}

fn parse_ensures(input: &str) -> ParseResult<Expr> {
    preceded(
        ws(tag("ensures")),
        ws(parse_simple_expr),
    )(input)
}

fn parse_function_def(input: &str) -> ParseResult<FunctionDef> {
    let (input, annotations) = many0(terminated(parse_annotation, multispace0))(input)?;
    let (input, _) = ws(tag("func"))(input)?;
    let (input, name) = ws(identifier)(input)?;
    let (input, params) = delimited(
        char('('),
        separated_list0(ws(char(',')), parse_function_param),
        char(')'),
    )(input)?;
    let (input, _) = ws(tag("->"))(input)?;
    let (input, return_type) = ws(parse_type)(input)?;
    
    // Parse requires/ensures clauses
    let (input, requires) = many0(terminated(parse_requires, multispace0))(input)?;
    let (input, ensures) = many0(terminated(parse_ensures, multispace0))(input)?;
    
    // Expect ':' before body
    let (input, _) = ws(char(':'))(input)?;
    let (input, _) = multispace0(input)?;
    
    // Parse body
    let (input, body) = parse_expr(input)?;
    
    Ok((input, FunctionDef {
        name,
        params,
        return_type,
        requires,
        ensures,
        body,
        annotations,
    }))
}

fn parse_extern_function_decl(annotations: Vec<Annotation>) -> impl FnMut(&str) -> ParseResult<ExternFunctionDecl> {
    move |input: &str| {
        // Find @extern annotation to get C name
        // Can be either @extern("printf") or @extern(name="printf")
        let extern_ann = annotations.iter().find(|ann| ann.name == "extern");
        let c_name = if let Some(ann) = extern_ann {
            if !ann.args.is_empty() {
                ann.args[0].1.clone()
            } else {
                "unknown_noargs".to_string()
            }
        } else {
            "unknown_noextern".to_string()
        };
        
        let (input, _) = ws(tag("func"))(input)?;
        let (input, name) = ws(identifier)(input)?;
        let (input, params) = delimited(
            char('('),
            separated_list0(ws(char(',')), parse_function_param),
            char(')'),
        )(input)?;
        let (input, _) = ws(tag("->"))(input)?;
        let (input, return_type) = ws(parse_type)(input)?;
        
        Ok((input, ExternFunctionDecl {
            name,
            c_name,
            params,
            return_type,
            annotations: annotations.clone(),
        }))
    }
}

// ============================================================================
// Program Parser
// ============================================================================

pub fn parse_ir(input: &str) -> Result<Program, String> {
    let (mut remaining, _) = skip_ws_and_comments(input)
        .map_err(|e| format!("Parse error: {:?}", e))?;
    
    let mut type_defs = vec![];
    let mut func_defs = vec![];
    let mut extern_funcs = vec![];
    
    // Parse type definitions and function definitions in order
    loop {
        let (new_input, _) = skip_ws_and_comments(remaining)
            .map_err(|e| format!("Parse error: {:?}", e))?;
        
        if new_input.is_empty() {
            break;
        }
        
        // Parse annotations if present
        let (new_input, annotations) = if new_input.starts_with("@") {
            many0(terminated(
                parse_annotation,
                skip_ws_and_comments,
            ))(new_input).map_err(|e| format!("Parse error: {:?}", e))?
        } else {
            (new_input, vec![])
        };
        
        let (new_input, _) = skip_ws_and_comments(new_input)
            .map_err(|e| format!("Parse error: {:?}", e))?;
        
        // Try to parse type definition
        if new_input.starts_with("type ") {
            // Try each type definition parser in order
            let parse_result = parse_multiline_record_type_def(vec![])(new_input)
                .or_else(|_| parse_variant_type_def(vec![])(new_input))
                .or_else(|_| parse_inline_type_def(vec![])(new_input));
            
            if let Ok((new_input, mut type_def)) = parse_result {
                type_def.annotations = annotations;
                type_defs.push(type_def);
                remaining = new_input;
                continue;
            }
        }
        
        // Check if this is an extern function (has @extern annotation)
        let has_extern = annotations.iter().any(|ann| ann.name == "extern");
        
        // Try to parse function definition
        if new_input.starts_with("func ") {
            if has_extern {
                // Parse as extern function declaration (no body)
                if let Ok((new_input, extern_func)) = parse_extern_function_decl(annotations)(new_input) {
                    extern_funcs.push(extern_func);
                    remaining = new_input;
                    continue;
                }
            } else {
                // Parse as regular function definition (with body)
                if let Ok((new_input, mut func_def)) = parse_function_def(new_input) {
                    func_def.annotations = [annotations, func_def.annotations].concat();
                    func_defs.push(func_def);
                    remaining = new_input;
                    continue;
                }
            }
        }
        
        // Skip unrecognized lines
        if let Some(newline_pos) = new_input.find('\n') {
            remaining = &new_input[newline_pos + 1..];
        } else {
            break;
        }
    }
    
    Ok(Program {
        type_defs,
        func_defs,
        extern_funcs,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_type() {
        assert_eq!(
            parse_type("Int"),
            Ok(("", Type::Basic(BasicType { name: "Int".to_string() })))
        );
    }

    #[test]
    fn test_parse_identifier() {
        assert_eq!(identifier("factorial"), Ok(("", "factorial".to_string())));
        assert_eq!(identifier("_test"), Ok(("", "_test".to_string())));
    }

    #[test]
    fn test_parse_int_literal() {
        let result = parse_int_literal("123");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_annotation() {
        let result = parse_annotation("@test_case(input=5, expected=120)");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_pattern() {
        // Literal pattern
        let result = parse_pattern("0");
        assert!(result.is_ok());
        
        // Variable pattern
        let result = parse_pattern("n");
        assert!(result.is_ok());
        
        // Wildcard pattern
        let result = parse_pattern("_");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_binary_op() {
        let result = parse_simple_expr("n * factorial (n - 1)");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_application() {
        let result = parse_application("factorial (n - 1)");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_match_expr() {
        let input = r#"match n with
  | 0 -> 1
  | n -> n * factorial (n - 1)"#;
        let result = parse_match_expr(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_factorial() {
        let input = r#"
@source("examples/01-factorial.pole", line=3)
@test_case(input=0, expected=1)
func factorial (n: Nat) -> Nat
  requires n >= 0
  ensures result >= 1
:
  match n with
  | 0 -> 1
  | n -> n * factorial (n - 1)
"#;
        let result = parse_ir(input);
        assert!(result.is_ok(), "Failed to parse factorial: {:?}", result.err());
        
        let program = result.unwrap();
        assert_eq!(program.func_defs.len(), 1);
        assert_eq!(program.func_defs[0].name, "factorial");
        assert_eq!(program.func_defs[0].requires.len(), 1);
        assert_eq!(program.func_defs[0].ensures.len(), 1);
    }
}

    #[test]
    fn test_parse_type_alias() {
        let result = parse_inline_type_def(vec![])("type UserId = String");
        assert!(result.is_ok(), "Failed: {:?}", result.err());
        let (_, type_def) = result.unwrap();
        assert_eq!(type_def.name, "UserId");
    }

    #[test]
    fn test_parse_ir_with_type_def() {
        let input = "type UserId = String";
        let result = parse_ir(input);
        assert!(result.is_ok(), "Failed: {:?}", result.err());
        let program = result.unwrap();
        assert_eq!(program.type_defs.len(), 1);
        assert_eq!(program.type_defs[0].name, "UserId");
    }

    #[test]
    fn test_parse_type_then_func() {
        let input = "type UserId = String\n\nfunc get_id (x: Int) -> UserId :\n  x";
        let result = parse_ir(input);
        assert!(result.is_ok(), "Failed: {:?}", result.err());
        let program = result.unwrap();
        assert_eq!(program.type_defs.len(), 1);
        assert_eq!(program.func_defs.len(), 1);
    }

    #[test]
    fn test_parse_simple_func() {
        let input = "func get_id (x: Int) -> UserId :\n  x";
        let result = parse_function_def(input);
        assert!(result.is_ok(), "Failed: {:?}", result.err());
    }

    #[test]
    fn test_parse_max_func() {
        let input = "@source(\"examples/07-max.pole\")\nfunc max(a: Int, b: Int) -> Int\n  requires true\n  ensures result >= a\n:\n  if a >= b then a else b";
        let result = parse_ir(input);
        assert!(result.is_ok(), "Failed: {:?}", result.err());
        let program = result.unwrap();
        assert_eq!(program.func_defs.len(), 1);
    }

fn parse_record_expr(input: &str) -> ParseResult<Expr> {
    let (input, _) = char('{')(input)?;
    let (input, _) = multispace0(input)?;
    
    let (input, fields) = separated_list0(
        ws(char(',')),
        separated_pair(
            ws(identifier),
            ws(char('=')),
            ws(parse_expr),
        ),
    )(input)?;
    
    let (input, _) = multispace0(input)?;
    let (input, _) = char('}')(input)?;
    
    Ok((input, Expr::Record(RecordExpr { fields })))
}

fn parse_list_literal(input: &str) -> ParseResult<Expr> {
    let (input, _) = char('[')(input)?;
    let (input, _) = multispace0(input)?;
    
    let (input, elements) = separated_list0(
        ws(char(',')),
        ws(parse_expr),
    )(input)?;
    
    let (input, _) = multispace0(input)?;
    let (input, _) = char(']')(input)?;
    
    // Represent list literal as Constructor
    Ok((input, Expr::Constructor(Constructor {
        name: "List".to_string(),
        args: elements,
    })))
}
