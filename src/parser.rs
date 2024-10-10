use bigdecimal::BigDecimal;
use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use std::collections::HashMap;
use std::fmt::Display;
use thiserror::Error;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct ExpressionParser;

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
    Concat(Vec<Expr>),
    Ceil(Box<Expr>),
    Round(Box<Expr>, Option<Box<Expr>>),
}

#[derive(Debug)]
pub enum Expr {
    Variable(String),
    Function(Function),
    String(String),
    Decimal(BigDecimal),
    UnaryMinus(Box<Expr>),
    BinOp {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum ExpressionValue {
    Number(BigDecimal),
    String(String),
}

impl ExpressionValue {
    pub fn to_decimal(&self) -> EvaluationResult<BigDecimal> {
        match self {
            ExpressionValue::Number(d) => Ok(d.clone()),
            ExpressionValue::String(_) => Err(ExpressionError::ExpectedDecimal),
        }
    }
}

impl From<String> for ExpressionValue {
    fn from(value: String) -> Self {
        ExpressionValue::String(value)
    }
}
impl From<BigDecimal> for ExpressionValue {
    fn from(value: BigDecimal) -> Self {
        ExpressionValue::Number(value)
    }
}

impl Display for ExpressionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionValue::Number(d) => d.fmt(f),
            ExpressionValue::String(s) => s.fmt(f),
        }
    }
}

#[derive(Debug)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Error, Debug)]
pub enum ExpressionError {
    #[error("Expected a decimal")]
    ExpectedDecimal,

    #[error("Expected {0} arguments")]
    MissingArguments(usize),

    #[error("Variable: {0} not found")]
    MissingVariable(String),
}

pub type EvaluationResult<T> = Result<T, ExpressionError>;

impl Op {
    pub fn evaluate(
        &self,
        lhs: &Expr,
        rhs: &Expr,
        map: &HashMap<String, String>,
    ) -> EvaluationResult<ExpressionValue> {
        let lhs_decimal = lhs.evaluate(map)?.to_decimal()?;
        let rhs_decimal = rhs.evaluate(map)?.to_decimal()?;

        let evaluated = match self {
            Op::Add => lhs_decimal + rhs_decimal,
            Op::Subtract => lhs_decimal - rhs_decimal,
            Op::Multiply => lhs_decimal * rhs_decimal,
            Op::Divide => lhs_decimal / rhs_decimal,
        };

        Ok(evaluated.into())
    }
}

impl Function {
    pub fn evaluate(&self, map: &HashMap<String, String>) -> EvaluationResult<ExpressionValue> {
        match self {
            Function::Concat(_) => todo!(),
            Function::Ceil(_) => todo!(),
            Function::Round(_, _) => todo!(),
        }
    }
}

impl Expr {
    pub fn evaluate(&self, map: &HashMap<String, String>) -> EvaluationResult<ExpressionValue> {
        let evaluated_expr: ExpressionValue = match self {
            Expr::Variable(name) => ExpressionValue::String(
                map.get(name)
                    .ok_or(ExpressionError::MissingVariable(name.clone()))?
                    .to_owned(),
            ),
            Expr::Function(f) => f.evaluate(map)?,
            Expr::String(s) => s.clone().into(),
            Expr::Decimal(d) => d.clone().into(),
            Expr::UnaryMinus(inner) => {
                ExpressionValue::Number(-(inner.evaluate(map)?.to_decimal()?))
            }
            Expr::BinOp { lhs, op, rhs } => op.evaluate(lhs.as_ref(), rhs.as_ref(), map)?,
        };

        Ok(evaluated_expr)
    }
}

pub fn parse_function(pairs: Pairs<Rule>) -> Function {
    let mut iter = pairs.into_iter();
    let name = iter.next().unwrap();
    match name.as_rule() {
        Rule::ceil => Function::Ceil(Box::new(parse_expr(iter))),
        Rule::concat => Function::Concat(
            iter.map(|r| parse_expr(r.into_inner()))
                .collect::<Vec<Expr>>(),
        ),
        rule => unreachable!("Expected function name, got :{:?}", rule),
    }
}

pub fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::function => Expr::Function(parse_function(primary.into_inner())),
            Rule::decimal => Expr::Decimal(primary.as_str().parse().unwrap()),
            Rule::expr => parse_expr(primary.into_inner()),
            Rule::variable => Expr::Variable(primary.as_str().to_owned()),
            Rule::string => Expr::String(primary.into_inner().as_str().to_owned()),
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
            Expr::BinOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::unary_minus => Expr::UnaryMinus(Box::new(rhs)),
            _ => unreachable!(),
        })
        .parse(pairs)
}
