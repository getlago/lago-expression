use std::{collections::HashMap, fmt::Display};

use bigdecimal::BigDecimal;
use serde::Deserialize;
use thiserror::Error;

use crate::parser::{EventAttribute, Expression, Function, Operation};

#[derive(Debug, PartialEq)]
pub enum ExpressionValue {
    Number(BigDecimal),
    String(String),
}

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct Event {
    code: String,
    timestamp: u64,
    properties: HashMap<String, String>,
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
            EventAttribute::Properties(name) => {
                let value = event
                    .properties
                    .get(name)
                    .ok_or(ExpressionError::MissingVariable(name.clone()))?;

                if let Ok(decimal_value) = value.parse() {
                    ExpressionValue::Number(decimal_value)
                } else {
                    ExpressionValue::String(value.clone())
                }
            }
        };
        Ok(evaluated_attribute)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn evaluate_and_compare(expr: Expression, event: &Event, expected_result: ExpressionValue) {
        match expr.evaluate(event) {
            Ok(expr) => {
                assert_eq!(expr, expected_result)
            }
            Err(e) => panic!("Failed to evalaute expression: {:?}", e),
        }
    }

    #[test]
    fn test_evaluate_bigdecimal() {
        let expr = Expression::Decimal(123.into());
        let event = Default::default();
        evaluate_and_compare(expr, &event, ExpressionValue::Number(123.into()));
    }

    #[test]
    fn test_evaluate_event_attribute_code() {
        let expr = Expression::EventAttribute(EventAttribute::Code);
        let event = Event {
            code: "result_code".into(),
            ..Default::default()
        };
        evaluate_and_compare(expr, &event, ExpressionValue::String("result_code".into()));
    }

    #[test]
    fn test_evaluate_event_attribute_timestamp() {
        let expr = Expression::EventAttribute(EventAttribute::Timestamp);
        let event = Event {
            timestamp: 1234,
            ..Default::default()
        };
        evaluate_and_compare(expr, &event, ExpressionValue::Number(1234.into()));
    }

    #[test]
    fn test_evaluate_event_attribute_property_decimal() {
        let expr = Expression::EventAttribute(EventAttribute::Properties("bar".into()));
        let properties = vec![("bar".into(), "123".into())].into_iter().collect();
        let event = Event {
            properties,
            ..Default::default()
        };
        evaluate_and_compare(expr, &event, ExpressionValue::Number(123.into()));
    }

    #[test]
    fn test_evaluate_event_attribute_property_no_decimal() {
        let expr = Expression::EventAttribute(EventAttribute::Properties("bar".into()));
        let properties = vec![("bar".into(), "foo".into())].into_iter().collect();
        let event = Event {
            properties,
            ..Default::default()
        };
        evaluate_and_compare(expr, &event, ExpressionValue::String("foo".into()));
    }

    #[test]
    fn test_evaluate_string() {
        let expr = Expression::String("bar".into());
        let event = Default::default();
        evaluate_and_compare(expr, &event, ExpressionValue::String("bar".into()));
    }

    #[test]
    fn test_evaluate_binop_plus() {
        let expr = Expression::BinOp {
            lhs: Box::new(Expression::Decimal(2.into())),
            op: Operation::Add,
            rhs: Box::new(Expression::Decimal(4.into())),
        };
        let event = Default::default();
        evaluate_and_compare(expr, &event, ExpressionValue::Number(6.into()));
    }

    #[test]
    fn test_evaluate_binop_minus() {
        let expr = Expression::BinOp {
            lhs: Box::new(Expression::Decimal(2.into())),
            op: Operation::Subtract,
            rhs: Box::new(Expression::Decimal(4.into())),
        };
        let event = Default::default();
        evaluate_and_compare(expr, &event, ExpressionValue::Number((-2).into()));
    }

    #[test]
    fn test_evaluate_binop_multiply() {
        let expr = Expression::BinOp {
            lhs: Box::new(Expression::Decimal(2.into())),
            op: Operation::Multiply,
            rhs: Box::new(Expression::Decimal(4.into())),
        };
        let event = Default::default();
        evaluate_and_compare(expr, &event, ExpressionValue::Number(8.into()));
    }

    #[test]
    fn test_evaluate_binop_divide() {
        let expr = Expression::BinOp {
            lhs: Box::new(Expression::Decimal(4.into())),
            op: Operation::Divide,
            rhs: Box::new(Expression::Decimal(2.into())),
        };
        let event = Default::default();
        evaluate_and_compare(expr, &event, ExpressionValue::Number(2.into()));
    }
    #[test]
    fn test_evaluate_unary_minus() {
        let expr = Expression::UnaryMinus(Box::new(Expression::Decimal(12.into())));
        let event = Default::default();
        evaluate_and_compare(expr, &event, ExpressionValue::Number((-12).into()));
    }
}
