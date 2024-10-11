use bigdecimal::BigDecimal;
use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use thiserror::Error;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct ExpressionParser;

pub type ParseResult = Result<Expression, ParseError>;

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

#[derive(Debug)]
pub enum Function {
    Concat(Vec<Expression>),
    Ceil(Box<Expression>),
    Round(Box<Expression>, Option<Box<Expression>>),
}

#[derive(Debug)]
pub enum Expression {
    Variable(String),
    Function(Function),
    String(String),
    Decimal(BigDecimal),
    UnaryMinus(Box<Expression>),
    BinOp {
        lhs: Box<Expression>,
        op: Op,
        rhs: Box<Expression>,
    },
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Failed to parse")]
    FailedToParse(#[from] ::pest::error::Error<Rule>),
}

#[derive(Debug)]
pub enum Op {
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

pub fn parse_expr(pairs: Pairs<Rule>) -> Expression {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::function => Expression::Function(parse_function(primary.into_inner())),
            Rule::decimal => Expression::Decimal(primary.as_str().parse().unwrap()),
            Rule::expr => parse_expr(primary.into_inner()),
            Rule::variable => Expression::Variable(primary.as_str().to_owned()),
            Rule::string => Expression::String(primary.into_inner().as_str().to_owned()),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => Op::Add,
                Rule::subtract => Op::Subtract,
                Rule::multiply => Op::Multiply,
                Rule::divide => Op::Divide,
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
            _ => unreachable!(),
        })
        .parse(pairs)
}
