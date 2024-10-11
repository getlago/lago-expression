use std::{collections::HashMap, fmt::Display};

use bigdecimal::BigDecimal;
use serde::Deserialize;
use thiserror::Error;

use crate::parser::{EventAttribute, Expression, Function, Operation};

#[derive(Debug)]
pub enum ExpressionValue {
    Number(BigDecimal),
    String(String),
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Event {
    code: String,
    timestamp: u64,
    properties: HashMap<String, String>,
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
    pub fn evaluate(&self, _event: &Event) -> EvaluationResult<ExpressionValue> {
        match self {
            Function::Concat(_) => todo!(),
            Function::Ceil(_) => todo!(),
            Function::Round(_, _) => todo!(),
        }
    }
}

impl EventAttribute {
    pub fn evaluate(&self, event: &Event) -> EvaluationResult<ExpressionValue> {
        let evaluated_attribute = match self {
            EventAttribute::Code => event.code.to_owned().into(),
            EventAttribute::Timestamp => ExpressionValue::Number(event.timestamp.into()),
            EventAttribute::Properties(name) => event
                .properties
                .get(name)
                .ok_or(ExpressionError::MissingVariable(name.clone()))?
                .clone()
                .into(),
        };
        Ok(evaluated_attribute)
    }
}

impl Expression {
    pub fn evaluate(&self, event: &Event) -> EvaluationResult<ExpressionValue> {
        let evaluated_expr = match self {
            Expression::EventAttribute(attr) => attr.evaluate(event)?,
            Expression::Function(f) => f.evaluate(event)?,
            Expression::String(s) => s.clone().into(),
            Expression::Decimal(d) => d.clone().into(),
            Expression::UnaryMinus(inner) => {
                ExpressionValue::Number(-(inner.evaluate(event)?.to_decimal()?))
            }
            Expression::BinOp { lhs, op, rhs } => op.evaluate(lhs.as_ref(), rhs.as_ref(), event)?,
        };

        Ok(evaluated_expr)
    }
}

impl Operation {
    pub fn evaluate(
        &self,
        lhs: &Expression,
        rhs: &Expression,
        event: &Event,
    ) -> EvaluationResult<ExpressionValue> {
        let lhs_decimal = lhs.evaluate(event)?.to_decimal()?;
        let rhs_decimal = rhs.evaluate(event)?.to_decimal()?;

        let evaluated = match self {
            Operation::Add => lhs_decimal + rhs_decimal,
            Operation::Subtract => lhs_decimal - rhs_decimal,
            Operation::Multiply => lhs_decimal * rhs_decimal,
            Operation::Divide => lhs_decimal / rhs_decimal,
        };

        Ok(evaluated.into())
    }
}
