use std::collections::HashMap;

use expression_core::{Event, Expression, ExpressionParser, ExpressionValue, PropertyValue};
use magnus::{
    error, function, method, r_hash::ForEach, value::ReprValue, Error, IntoValue, Module, Object,
    RHash, Ruby, TryConvert, Value,
};

#[magnus::wrap(class = "Lago::Expression", free_immediately, size)]
struct ExpressionWrapper(Expression);

#[magnus::wrap(class = "Lago::Event", free_immediately, size)]
struct EventWrapper(Event);

impl EventWrapper {
    fn new(ruby: &Ruby, code: String, timestamp: u64, map: RHash) -> error::Result<EventWrapper> {
        let mut properties = HashMap::default();

        map.foreach(|key: String, value: Value| {
            let property_value = if value.is_kind_of(ruby.class_numeric()) {
                // Convert ruby numbers to a formatted string, that can be parsed into a BigDecimal
                let ruby_string = value.to_r_string()?;
                let big_d = ruby_string
                    .to_string()?
                    .parse()
                    .expect("Failed to parse a number as bigdecimal");
                PropertyValue::Number(big_d)
            } else if value.is_kind_of(ruby.class_string()) {
                PropertyValue::String(String::try_convert(value)?)
            } else {
                PropertyValue::String(value.to_string())
            };
            properties.insert(key, property_value);
            Ok(ForEach::Continue)
        })?;

        Ok(Self(Event {
            code,
            timestamp,
            properties,
        }))
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
) -> error::Result<magnus::Value> {
    let evaluated = expr
        .0
        .evaluate(&event.0)
        .map_err(|err| Error::new(ruby.exception_runtime_error(), err.to_string()))?;

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
