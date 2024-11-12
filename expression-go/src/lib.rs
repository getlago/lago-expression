use std::{
    ffi::{c_char, CStr, CString},
    ptr::null_mut,
};

use expression_core::ExpressionParser;

#[no_mangle]
/// # Safety
/// Pass in a valid strings
pub unsafe extern "C" fn evaluate(input: *const c_char, event: *const c_char) -> *mut c_char {
    let input = unsafe { CStr::from_ptr(input).to_str().unwrap().to_owned() };

    // Cannot parse expression -> return null
    let Ok(expr) = ExpressionParser::parse_expression(&input) else {
        return null_mut();
    };

    let json = unsafe { CStr::from_ptr(event).to_str().unwrap() };

    let Ok(event) = serde_json::from_str(json) else {
        return null_mut();
    };

    // evaluate expression, errors are not returned, but we do catch them and return null
    let Ok(res) = expr.evaluate(&event) else {
        return null_mut();
    };

    let Ok(temp) = CString::new(res.to_string()) else {
        return null_mut();
    };
    temp.into_raw()
}

#[no_mangle]
/// # Safety
/// Only pass in pointers to strings that have been obtained through `evaluate`
pub unsafe extern "C" fn free_evaluate(ptr: *mut c_char) {
    unsafe { drop(CString::from_raw(ptr)) }
}
