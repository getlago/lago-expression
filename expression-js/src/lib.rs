use wasm_bindgen::prelude::*;

use lago_expression::ExpressionParser;
extern crate console_error_panic_hook;

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(js_name = parseExpression)]
pub fn parse_expression(expression: String) -> Result<String, String> {
    ExpressionParser::parse_expression(&expression)
        .map_err(|e| format!("{}", e))
        .map(|e| format!("{e:?}"))
}
