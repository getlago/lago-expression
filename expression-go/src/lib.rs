use std::ffi::{c_char, CStr, CString};

use expression_core::{Event, Expression, ExpressionParser};

#[no_mangle]
/// # Safety
/// Pass in a valid string
pub unsafe extern "C" fn parse(input: *const c_char) -> *mut Expression {
    let input = unsafe { CStr::from_ptr(input).to_str().unwrap().to_owned() };
    let expression = ExpressionParser::parse_expression(&input).unwrap();
    Box::into_raw(Box::new(expression))
}

#[no_mangle]
/// # Safety
/// Pass in a valid string
pub unsafe extern "C" fn evaluate(expr: *mut Expression, event: *const c_char) -> *const c_char {
    let json = unsafe { CStr::from_ptr(event).to_str().unwrap() };
    let event: Event = serde_json::from_str(json).unwrap();

    let expr = unsafe { expr.as_ref().unwrap() };
    match expr.evaluate(&event).unwrap() {
        expression_core::ExpressionValue::Number(d) => {
            let temp = CString::new(d.to_string()).unwrap();
            temp.into_raw()
        }
        expression_core::ExpressionValue::String(d) => {
            let temp = CString::new(d).unwrap();
            temp.into_raw()
        }
    }
}
