use wasm_bindgen::prelude::*;
use web_sys::console;

use expression_core::ExpressionParser;
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
pub fn evaluate_expression(expression: Expression, _event: &JsValue) -> Result<String, String> {
    console::log_1(&format!("{:?}", expression.0).into());
    let event = expression_core::Event {
        ..Default::default()
    };
    expression
        .0
        .evaluate(&event)
        .map(|value| match value {
            expression_core::ExpressionValue::Number(d) => d.to_string(),
            expression_core::ExpressionValue::String(s) => s,
        })
        .map_err(|e| format!("{}", e))
}
