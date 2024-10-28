use std::collections::HashMap;

use js_sys::Reflect;
use wasm_bindgen::prelude::*;

use bigdecimal::ToPrimitive;

use expression_core::{ExpressionParser, ExpressionValue};
use web_sys::console;
extern crate console_error_panic_hook;

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Expression(expression_core::Expression);

#[wasm_bindgen(js_name = parseExpression)]
pub fn parse_expression(expression: String) -> Result<Expression, String> {
    ExpressionParser::parse_expression(&expression)
        .map_err(|e| format!("{}", e))
        .map(Expression)
}

#[wasm_bindgen(js_name = evaluateExpression)]
pub fn evaluate_expression(
    expression: Expression,
    code: String,
    timestamp: u64,
    js_properties: &JsValue,
) -> Result<JsValue, JsValue> {
    let mut properties = HashMap::new();

    let keys = Reflect::own_keys(js_properties)?;

    for key in keys {
        let value = Reflect::get(js_properties, &key)?;
        console::log_1(&value);

        let insert = if value.is_string() {
            String::try_from(value)?
        } else {
            let n = u64::try_from(value)?;
            format!("{n}")
        };

        properties.insert(key.as_string().ok_or("expected string")?, insert);
    }

    let event = expression_core::Event {
        code,
        timestamp,
        properties,
    };

    expression
        .0
        .evaluate(&event)
        .map(|value| match value {
            ExpressionValue::Number(d) => d.to_f64().into(),
            ExpressionValue::String(s) => s.into(),
        })
        .map_err(|e| format!("{}", e).into())
}
