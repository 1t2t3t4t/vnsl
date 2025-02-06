use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum VnslDataType {
    String(String),
    Number(f64),
    Bool(bool),
}

impl VnslDataType {
    pub fn get_number(self) -> f64 {
        match self {
            VnslDataType::Number(n) => n,
            _ => panic!("Data type {:?} is not a number", self),
        }
    }
}

impl Display for VnslDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VnslDataType::String(s) => write!(f, "{}", s),
            VnslDataType::Number(n) => write!(f, "{}", n),
            VnslDataType::Bool(b) => write!(f, "{}", b),
        }
    }
}

impl From<String> for VnslDataType {
    fn from(value: String) -> Self {
        VnslDataType::String(value)
    }
}

impl From<f64> for VnslDataType {
    fn from(value: f64) -> Self {
        VnslDataType::Number(value)
    }
}
impl From<f32> for VnslDataType {
    fn from(value: f32) -> Self {
        VnslDataType::Number(value as f64)
    }
}
impl From<i32> for VnslDataType {
    fn from(value: i32) -> Self {
        VnslDataType::Number(value as f64)
    }
}

impl From<bool> for VnslDataType {
    fn from(value: bool) -> Self {
        VnslDataType::Bool(value)
    }
}
