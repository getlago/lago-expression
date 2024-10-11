use parser::ParseResult;
pub use parser::{Expression, ExpressionParser, ParseError};
use pest::Parser;

mod evaluate;
mod parser;

impl ExpressionParser {
    #[allow(clippy::result_large_err)]
    pub fn parse_expression(input: &str) -> ParseResult {
        let pairs = Self::parse(parser::Rule::root, input)?;

        let expr = parser::parse_expr(pairs);
        Ok(expr)
    }
}
