pub use evaluate::EvaluationResult;
pub use parser::{Expression, ExpressionParser, ParseError};
pub use pest::Parser;

mod evaluate;
mod parser;
