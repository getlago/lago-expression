use std::collections::HashMap;

use expression_core::{Event, Expression, ExpressionParser, ExpressionValue};
use magnus::{function, method, value::ReprValue, Error, IntoValue, Module, Object, Ruby};

#[magnus::wrap(class = "Lago::Expression", free_immediately, size)]
struct ExpressionWrapper(Expression);

#[magnus::wrap(class = "Lago::Event", free_immediately, size)]
struct EventWrapper(Event);

impl EventWrapper {
    fn new(code: String, timestamp: u64, map: HashMap<String, String>) -> EventWrapper {
        Self(Event {
            code,
            timestamp,
            properties: map,
        })
    }
}

/// Parse the given input and return an Optional ExpressionWrapper,
/// will return None when the expression is not valid
fn parse(input: String) -> Option<ExpressionWrapper> {
    ExpressionParser::parse_expression(&input)
        .ok()
        .map(ExpressionWrapper)
}

/// Validate the given expression, returns None if the expression is Valid
/// an String is returned if the expression is invalid
fn validate(input: String) -> Option<String> {
    ExpressionParser::parse_expression(&input)
        .err()
        .map(|e| e.to_string())
}

fn evaluate(
    ruby: &Ruby,
    expr: &ExpressionWrapper,
    event: &EventWrapper,
) -> Result<magnus::Value, magnus::Error> {
    let evaluated = expr.0.evaluate(&event.0).unwrap();

    match evaluated {
        ExpressionValue::Number(d) => d
            .to_string()
            .into_value_with(ruby)
            .funcall_public("to_d", ()),
        ExpressionValue::String(s) => Ok(s.into_value_with(ruby)),
    }
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Lago")?;

    let class = module.define_class("ExpressionParser", ruby.class_object())?;
    class.define_singleton_method("parse", function!(parse, 1))?;
    class.define_singleton_method("validate", function!(validate, 1))?;

    let class = module.define_class("Expression", ruby.class_object())?;
    class.define_method("evaluate", method!(evaluate, 1))?;

    let class = module.define_class("Event", ruby.class_object())?;
    class.define_singleton_method("new", function!(EventWrapper::new, 3))?;

    Ok(())
}
