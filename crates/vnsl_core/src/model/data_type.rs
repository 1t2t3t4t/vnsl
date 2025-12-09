use std::fmt::Display;

use mlua::{Error, FromLua, IntoLua, Lua, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
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

    pub fn try_from(value: Value) -> Option<Self> {
        match value {
            Value::Boolean(b) => Some(Self::Bool(b)),
            Value::Integer(i) => Some(Self::Number(i as f64)),
            Value::Number(n) => Some(Self::Number(n)),
            Value::String(s) => Some(Self::String(s.to_string_lossy())),
            Value::Nil => None,
            _ => None,
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
impl From<&str> for VnslDataType {
    fn from(value: &str) -> Self {
        VnslDataType::String(value.to_string())
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
impl From<i64> for VnslDataType {
    fn from(value: i64) -> Self {
        VnslDataType::Number(value as f64)
    }
}

impl From<bool> for VnslDataType {
    fn from(value: bool) -> Self {
        VnslDataType::Bool(value)
    }
}

impl From<Value> for VnslDataType {
    fn from(value: Value) -> Self {
        Self::try_from(value.clone()).expect(&format!("No compatible data type {:?}", value))
    }
}

impl IntoLua for VnslDataType {
    fn into_lua(self, lua: &Lua) -> mlua::Result<Value> {
        match self {
            VnslDataType::String(s) => s.into_lua(lua),
            VnslDataType::Number(n) => n.into_lua(lua),
            VnslDataType::Bool(b) => b.into_lua(lua),
        }
    }
}

impl FromLua for VnslDataType {
    fn from_lua(value: Value, _lua: &Lua) -> mlua::Result<Self> {
        Self::try_from(value.clone()).ok_or(Error::RuntimeError(format!(
            "Value of type {} cannot be converted from lua type",
            value.type_name()
        )))
    }
}
