use std::{collections::HashMap, fmt::Display};

use bigdecimal::BigDecimal;
use thiserror::Error;

use crate::parser::{Expression, Function, Operation};

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

impl Function {
    pub fn evaluate(&self, map: &HashMap<String, String>) -> EvaluationResult<ExpressionValue> {
        match self {
            Function::Concat(_) => todo!(),
            Function::Ceil(_) => todo!(),
            Function::Round(_, _) => todo!(),
        }
    }
}

impl Expression {
    pub fn evaluate(&self, map: &HashMap<String, String>) -> EvaluationResult<ExpressionValue> {
        let evaluated_expr = match self {
            Expression::Variable(name) => ExpressionValue::String(
                map.get(name)
                    .ok_or(ExpressionError::MissingVariable(name.clone()))?
                    .to_owned(),
            ),
            Expression::Function(f) => f.evaluate(map)?,
            Expression::String(s) => s.clone().into(),
            Expression::Decimal(d) => d.clone().into(),
            Expression::UnaryMinus(inner) => {
                ExpressionValue::Number(-(inner.evaluate(map)?.to_decimal()?))
            }
            Expression::BinOp { lhs, op, rhs } => op.evaluate(lhs.as_ref(), rhs.as_ref(), map)?,
        };

        Ok(evaluated_expr)
    }
}

impl Operation {
    pub fn evaluate(
        &self,
        lhs: &Expression,
        rhs: &Expression,
        map: &HashMap<String, String>,
    ) -> EvaluationResult<ExpressionValue> {
        let lhs_decimal = lhs.evaluate(map)?.to_decimal()?;
        let rhs_decimal = rhs.evaluate(map)?.to_decimal()?;

        let evaluated = match self {
            Operation::Add => lhs_decimal + rhs_decimal,
            Operation::Subtract => lhs_decimal - rhs_decimal,
            Operation::Multiply => lhs_decimal * rhs_decimal,
            Operation::Divide => lhs_decimal / rhs_decimal,
        };

        Ok(evaluated.into())
    }
}
