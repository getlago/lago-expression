use magnus::{function, Error, Ruby};

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    ruby.define_global_function("parse_and_test", function!(ruby_parse_and_test, 2));
    Ok(())
}
