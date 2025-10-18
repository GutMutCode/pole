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

// Simplified expression parser for now
fn parse_expr(input: &str) -> ParseResult<Expr> {
    alt((
        parse_literal,
        parse_variable,
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
    
    // Skip requires/ensures for now (simplified)
    let (input, _) = opt(preceded(multispace0, char(':')))(input)?;
    let (input, _) = multispace0(input)?;
    
    // Parse body (simplified - just one expression)
    let (input, body) = parse_expr(input)?;
    
    Ok((input, FunctionDef {
        name,
        params,
        return_type,
        requires: vec![],
        ensures: vec![],
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
}
