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
    // Simplified: just parse key=value pairs
    input.split(',')
        .filter_map(|s| {
            let parts: Vec<&str> = s.splitn(2, '=').collect();
            if parts.len() == 2 {
                Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
            } else {
                None
            }
        })
        .collect()
}

// ============================================================================
// Expression Parsers  
// ============================================================================

fn parse_literal(input: &str) -> ParseResult<Expr> {
    alt((
        parse_int_literal,
        parse_float_literal,
        parse_bool_literal,
        parse_string_literal,
    ))(input)
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
    let (input, left) = parse_primary_expr(input)?;
    
    let (input, op_and_right) = opt(tuple((
        ws(alt((
            tag("*"), tag("/"), tag("%"),
            tag("+"), tag("-"),
            tag("=="), tag("!="), 
            tag("<="), tag(">="), 
            tag("<"), tag(">"),
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
    map(
        pair(
            identifier,
            delimited(
                ws(char('(')),
                parse_expr,
                ws(char(')')),
            ),
        ),
        |(func_name, arg)| {
            Expr::Application(Application {
                func: Box::new(Expr::Variable(Variable { name: func_name })),
                arg: Box::new(arg),
            })
        },
    )(input)
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
    let (input, then_branch) = ws(parse_simple_expr)(input)?;
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
        parse_literal,
        parse_application,
        parse_variable,
        delimited(char('('), parse_expr, char(')')),
    ))(input)
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

// ============================================================================
// Program Parser
// ============================================================================

pub fn parse_ir(input: &str) -> Result<Program, String> {
    let (input, _) = skip_ws_and_comments(input)
        .map_err(|e| format!("Parse error: {:?}", e))?;
    
    let (input, func_defs) = many0(terminated(
        parse_function_def,
        skip_ws_and_comments,
    ))(input)
        .map_err(|e| format!("Parse error: {:?}", e))?;
    
    Ok(Program {
        type_defs: vec![], // TODO: implement type def parsing
        func_defs,
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
