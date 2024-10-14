use expression_core::{Expression, ExpressionParser};
use magnus::{function, method, Error, IntoValue, Module, Object, Ruby};

#[magnus::wrap(class = "Expression", free_immediately, size)]
struct ExpressionWrapper(Expression);

fn parse(input: String) -> ExpressionWrapper {
    ExpressionWrapper(ExpressionParser::parse_expression(&input).unwrap())
}

fn evaluate(ruby: &Ruby, expr: &ExpressionWrapper) -> magnus::Value {
    ruby.fixnum_from_u64(1).unwrap().into_value_with(ruby)
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let class = ruby.define_class("ExpressionParser", ruby.class_object())?;
    class.define_singleton_method("parse", function!(parse, 1))?;

    let class = ruby.define_class("Expression", ruby.class_object())?;
    class.define_method("evaluate", method!(evaluate, 0))?;

    Ok(())
}
