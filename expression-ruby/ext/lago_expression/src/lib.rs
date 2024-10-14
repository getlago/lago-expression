use std::collections::HashMap;

use expression_core::{Event, Expression, ExpressionParser, ExpressionValue};
use magnus::{function, method, value::ReprValue, Error, IntoValue, Module, Object, Ruby};

#[magnus::wrap(class = "Expression", free_immediately, size)]
struct ExpressionWrapper(Expression);

#[magnus::wrap(class = "Event", free_immediately, size)]
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

fn parse(input: String) -> ExpressionWrapper {
    ExpressionWrapper(ExpressionParser::parse_expression(&input).unwrap())
}

fn evaluate(ruby: &Ruby, expr: &ExpressionWrapper, event: &EventWrapper) -> magnus::Value {
    let evaluated = expr.0.evaluate(&event.0).unwrap();

    match evaluated {
        ExpressionValue::Number(d) => d
            .to_string()
            .into_value_with(ruby)
            .funcall_public("to_d", ())
            .unwrap(),
        ExpressionValue::String(s) => s.into_value_with(ruby),
    }
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let class = ruby.define_class("ExpressionParser", ruby.class_object())?;
    class.define_singleton_method("parse", function!(parse, 1))?;

    let class = ruby.define_class("Expression", ruby.class_object())?;
    class.define_method("evaluate", method!(evaluate, 1))?;

    let class = ruby.define_class("Event", ruby.class_object())?;
    class.define_singleton_method("new", function!(EventWrapper::new, 3))?;

    Ok(())
}
