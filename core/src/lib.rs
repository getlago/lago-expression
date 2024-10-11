use parser::ParseResult;
pub use parser::{Expression, ExpressionParser, ParseError, Rule};
pub use pest::Parser;

mod evaluate;
mod parser;

impl ExpressionParser {
    #[allow(clippy::result_large_err)]
    pub fn parse_expression(input: &str) -> ParseResult {
        let mut pairs = Self::parse(parser::Rule::root, input)?;

        let inner = pairs.next().unwrap().into_inner();
        let expr = parser::parse_expr(inner);
        Ok(expr)
    }
}
#[cfg(test)]
mod tests {
    use parser::EventAttribute;

    use super::*;

    #[test]
    fn test_parse_expression() {
        let input = "1";
        match ExpressionParser::parse_expression(input) {
            Ok(expr) => {
                assert_eq!(expr, Expression::Decimal(1.into()))
            }
            Err(e) => panic!("Failed to parse expression: {:?}", e),
        }
    }

    #[test]
    fn test_parse_event_attribute() {
        let input = "event.timestamp";
        match ExpressionParser::parse_expression(input) {
            Ok(expr) => {
                assert_eq!(expr, Expression::EventAttribute(EventAttribute::Timestamp))
            }
            Err(e) => panic!("Failed to parse expression: {:?}", e),
        }
    }

    #[test]
    fn test_parse_event_properties() {
        let input = "event.properties.blah";
        match ExpressionParser::parse_expression(input) {
            Ok(expr) => {
                assert_eq!(
                    expr,
                    Expression::EventAttribute(EventAttribute::Properties("blah".to_owned()))
                )
            }
            Err(e) => panic!("Failed to parse expression: {:?}", e),
        }
    }
}
