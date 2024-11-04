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
