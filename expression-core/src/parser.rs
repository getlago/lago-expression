use bigdecimal::BigDecimal;
use pest::{iterators::Pairs, pratt_parser::PrattParser};

use pest::Parser;
use thiserror::Error;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct ExpressionParser;

impl ExpressionParser {
    #[allow(clippy::result_large_err)]
    pub fn parse_expression(input: &str) -> ParseResult<Expression> {
        let mut pairs = Self::parse(Rule::root, input)?;

        let inner = pairs.next().unwrap().into_inner();
        parse_expr(inner)
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, PartialEq)]
pub enum Function {
    Concat(Vec<Expression>),
    Ceil(Box<Expression>),
    Round(Box<Expression>, Option<Box<Expression>>),
}

#[derive(Debug, PartialEq)]
pub enum EventAttribute {
    Code,
    Timestamp,
    Properties(String),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    EventAttribute(EventAttribute),
    Function(Function),
    String(String),
    Decimal(BigDecimal),
    UnaryMinus(Box<Expression>),
    BinOp {
        lhs: Box<Expression>,
        op: Operation,
        rhs: Box<Expression>,
    },
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Parsing error: {0}")]
    FailedToParse(#[from] ::pest::error::Error<Rule>),

    #[error("Wrong number of arguments to function {0}, expected: {1}, provided: {2}")]
    WrongNumberOfArguments(String, String, usize),

    #[error("bigdecimal parsing error: {0}")]
    FailedToParseBigDecimal(#[from] ::bigdecimal::ParseBigDecimalError),
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn parse_function(pairs: Pairs<Rule>) -> ParseResult<Function> {
    let mut iter = pairs.into_iter();
    let name = iter.next().unwrap();
    let function = match name.as_rule() {
        Rule::ceil => Function::Ceil(Box::new(parse_expr(iter)?)),
        Rule::concat => {
            let args = iter
                .map(|r| parse_expr(r.into_inner()))
                .collect::<ParseResult<Vec<Expression>>>()?;

            Function::Concat(args)
        }
        Rule::round => {
            let mut args = iter
                .map(|r| {
                    let expr = parse_expr(r.into_inner())?;
                    Ok(Box::new(expr))
                })
                .collect::<Vec<ParseResult<Box<Expression>>>>();

            match args.len() {
                1 => Function::Round(args.remove(0)?, None),
                2 => Function::Round(args.remove(0)?, Some(args.remove(0)?)),
                n => {
                    return Err(ParseError::WrongNumberOfArguments(
                        "round".to_owned(),
                        "1..2".to_owned(),
                        n,
                    ))
                }
            }
        }
        rule => unreachable!("Expected function name, got :{:?}", rule),
    };
    Ok(function)
}

fn parse_event_attribute(mut pairs: Pairs<Rule>) -> EventAttribute {
    let mut inner = pairs.next().unwrap().into_inner();
    match inner.next().unwrap().as_rule() {
        Rule::event_code => EventAttribute::Code,
        Rule::event_timestamp => EventAttribute::Timestamp,
        Rule::event_properties => {
            EventAttribute::Properties(inner.next().unwrap().as_str().to_owned())
        }
        rule => unreachable!("expected an event attribute, got: {rule:?}"),
    }
}

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
            .op(Op::prefix(unary_minus))
    };
}

fn parse_expr(pairs: Pairs<Rule>) -> ParseResult<Expression> {
    PRATT_PARSER
        .map_primary(|primary| {
            let value = match primary.as_rule() {
                Rule::function => Expression::Function(parse_function(primary.into_inner())?),
                Rule::decimal => Expression::Decimal(primary.as_str().parse()?),
                Rule::expr => parse_expr(primary.into_inner())?,
                Rule::variable => {
                    Expression::EventAttribute(parse_event_attribute(primary.into_inner()))
                }
                Rule::string => Expression::String(primary.into_inner().as_str().to_owned()),
                rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
            };
            Ok(value)
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => Operation::Add,
                Rule::subtract => Operation::Subtract,
                Rule::multiply => Operation::Multiply,
                Rule::divide => Operation::Divide,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Ok(Expression::BinOp {
                lhs: Box::new(lhs?),
                op,
                rhs: Box::new(rhs?),
            })
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::unary_minus => Ok(Expression::UnaryMinus(Box::new(rhs?))),
            rule => unreachable!("Expr::parse expected operation, found {:?}", rule),
        })
        .parse(pairs)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_and_compare(input: &str, expected_expr: Expression) {
        match ExpressionParser::parse_expression(input) {
            Ok(expr) => {
                assert_eq!(expr, expected_expr)
            }
            Err(e) => panic!("Failed to parse expression: {:?}", e),
        }
    }

    #[test]
    fn test_parse_expression() {
        parse_and_compare("1", Expression::Decimal(1.into()));
    }

    #[test]
    fn test_parse_event_attribute() {
        parse_and_compare(
            "event.timestamp",
            Expression::EventAttribute(EventAttribute::Timestamp),
        );
    }

    #[test]
    fn test_parse_event_properties() {
        parse_and_compare(
            "event.properties.blah",
            Expression::EventAttribute(EventAttribute::Properties("blah".to_owned())),
        );
    }

    #[test]
    fn test_parse_string() {
        parse_and_compare("'test'", Expression::String("test".to_owned()));
    }

    #[test]
    fn test_parse_concat() {
        parse_and_compare(
            "concat('a', 'b')",
            Expression::Function(Function::Concat(vec![
                Expression::String("a".to_owned()),
                Expression::String("b".to_owned()),
            ])),
        );
    }

    #[test]
    fn test_parse_ceil() {
        parse_and_compare(
            "ceil(123)",
            Expression::Function(Function::Ceil(Box::new(Expression::Decimal(123.into())))),
        );
    }

    #[test]
    fn test_parse_round() {
        parse_and_compare(
            "round(123, 1)",
            Expression::Function(Function::Round(
                Box::new(Expression::Decimal(123.into())),
                Some(Box::new(Expression::Decimal(1.into()))),
            )),
        );
    }
    #[test]
    fn test_parse_round_one_arg() {
        parse_and_compare(
            "round(123)",
            Expression::Function(Function::Round(
                Box::new(Expression::Decimal(123.into())),
                None,
            )),
        );
    }
}
