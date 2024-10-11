use bigdecimal::BigDecimal;
use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use thiserror::Error;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct ExpressionParser;

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

pub fn parse_function(pairs: Pairs<Rule>) -> Function {
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

pub fn parse_event_attribute(pairs: Pairs<Rule>) -> EventAttribute {
    dbg!(&pairs);
    EventAttribute::Code
}

pub fn parse_expr(pairs: Pairs<Rule>) -> Expression {
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
