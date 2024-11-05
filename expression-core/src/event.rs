use std::collections::HashMap;

use bigdecimal::BigDecimal;
use serde::Deserialize;

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct Event {
    pub code: String,
    pub timestamp: u64,
    pub properties: HashMap<String, PropertyValue>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum PropertyValue {
    String(String),
    Number(BigDecimal),
}

impl From<&str> for PropertyValue {
    fn from(value: &str) -> Self {
        PropertyValue::String(value.into())
    }
}
impl From<String> for PropertyValue {
    fn from(value: String) -> Self {
        PropertyValue::String(value)
    }
}

impl From<u64> for PropertyValue {
    fn from(value: u64) -> Self {
        PropertyValue::Number(value.into())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_deserialize_with_numbers() {
        let json = json!({
            "code": "testing",
            "timestamp": 123124123123i64,
            "properties": {
                "text": "text_value",
                "number": 300
            }
        });

        let event = serde_json::from_value::<Event>(json).expect("expected json to parse");

        let code = "testing".to_owned();
        let timestamp = 123124123123;
        let mut properties = HashMap::default();
        properties.insert("number".to_owned(), PropertyValue::Number(300.into()));
        properties.insert(
            "text".to_owned(),
            PropertyValue::String("text_value".into()),
        );

        assert_eq!(
            event,
            Event {
                code,
                timestamp,
                properties
            }
        )
    }

    #[test]
    fn test_deserialize_with_floating_point() {
        let json = r#"{
            "code": "testing",
            "timestamp": 123124123123,
            "properties": {
                "text": "text_value",
                "number": 3.2
            }
        }"#;

        let event = serde_json::from_str::<Event>(json).expect("expected json to parse");

        let code = "testing".to_owned();
        let timestamp = 123124123123;
        let mut properties = HashMap::default();
        properties.insert(
            "number".to_owned(),
            PropertyValue::Number("3.2".parse().unwrap()),
        );
        properties.insert(
            "text".to_owned(),
            PropertyValue::String("text_value".into()),
        );

        assert_eq!(
            event,
            Event {
                code,
                timestamp,
                properties
            }
        )
    }
}
