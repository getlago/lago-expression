use bigdecimal::BigDecimal;
use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use pest::Parser;
use thiserror::Error;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct ExpressionParser;

impl ExpressionParser {
    #[allow(clippy::result_large_err)]
    pub fn parse_expression(input: &str) -> ParseResult {
        let mut pairs = Self::parse(Rule::root, input)?;

        let inner = pairs.next().unwrap().into_inner();
        let expr = parse_expr(inner);
        Ok(expr)
    }
}

pub type ParseResult = Result<Expression, ParseError>;
use pest::pratt_parser::{Assoc::*, Op};

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
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn parse_function(pairs: Pairs<Rule>) -> Function {
    let mut iter = pairs.into_iter();
    let name = iter.next().unwrap();
    match name.as_rule() {
        Rule::ceil => Function::Ceil(Box::new(parse_expr(iter))),
        Rule::concat => Function::Concat(
            iter.map(|r| parse_expr(r.into_inner()))
                .collect::<Vec<Expression>>(),
        ),
        rule => unreachable!("Expected function name, got :{:?}", rule),
    }
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

fn parse_expr(pairs: Pairs<Rule>) -> Expression {
    PrattParser::new()
        // Addition and subtract have equal precedence
        .op(Op::infix(Rule::add, Left) | Op::infix(Rule::subtract, Left))
        .op(Op::infix(Rule::multiply, Left) | Op::infix(Rule::divide, Left))
        .op(Op::prefix(Rule::unary_minus))
        .map_primary(|primary| match primary.as_rule() {
            Rule::function => Expression::Function(parse_function(primary.into_inner())),
            Rule::decimal => Expression::Decimal(primary.as_str().parse().unwrap()),
            Rule::expr => parse_expr(primary.into_inner()),
            Rule::variable => {
                Expression::EventAttribute(parse_event_attribute(primary.into_inner()))
            }
            Rule::string => Expression::String(primary.into_inner().as_str().to_owned()),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => Operation::Add,
                Rule::subtract => Operation::Subtract,
                Rule::multiply => Operation::Multiply,
                Rule::divide => Operation::Divide,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Expression::BinOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::unary_minus => Expression::UnaryMinus(Box::new(rhs)),
            rule => unreachable!("Expr::parse expected operation, found {:?}", rule),
        })
        .parse(pairs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expression() {
        let input = "1";
        match ExpressionParser::parse_expression(input) {
            Ok(expr) => {
                assert_eq!(expr, Expression::Decimal(1.into()))
            }
            Err(e) => panic!("Failed to parse expression: {:?}", e),
        }
    }

    #[test]
    fn test_parse_event_attribute() {
        let input = "event.timestamp";
        match ExpressionParser::parse_expression(input) {
            Ok(expr) => {
                assert_eq!(expr, Expression::EventAttribute(EventAttribute::Timestamp))
            }
            Err(e) => panic!("Failed to parse expression: {:?}", e),
        }
    }

    #[test]
    fn test_parse_event_properties() {
        let input = "event.properties.blah";
        match ExpressionParser::parse_expression(input) {
            Ok(expr) => {
                assert_eq!(
                    expr,
                    Expression::EventAttribute(EventAttribute::Properties("blah".to_owned()))
                )
            }
            Err(e) => panic!("Failed to parse expression: {:?}", e),
        }
    }
}
