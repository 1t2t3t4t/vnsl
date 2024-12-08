use mlua::{FromLuaMulti, IntoLua, Lua, Value};
use vnsl_core::model::VnslDataType;

use crate::runtime_result::{RuntimeError, RuntimeResult};

#[derive(Debug)]
pub struct LuaRuntime {
    lua: Lua,
}

impl Default for LuaRuntime {
    fn default() -> Self {
        let lua = Lua::new();
        Self { lua }
    }
}

impl LuaRuntime {
    pub fn set_globals_val_data_type(&self, key: &str, val: VnslDataType) -> RuntimeResult<()> {
        self.lua
            .globals()
            .set(key, val.to_lua(&self.lua)?)
            .map_err(|e| RuntimeError::LuaError(e))
    }

    pub fn set_globals_val<T: IntoLua>(&self, key: &str, val: T) -> RuntimeResult<()> {
        self.lua
            .globals()
            .set(key, val)
            .map_err(|e| RuntimeError::LuaError(e))
    }

    pub fn eval_expr<T: FromLuaMulti>(&self, expr: &str) -> RuntimeResult<T> {
        self.lua
            .load(expr)
            .eval::<T>()
            .map_err(|e| RuntimeError::LuaEvalError(expr.to_string(), e))
    }
}

pub trait ToLua {
    fn to_lua(self, lua: &Lua) -> mlua::Result<Value>;
}

impl ToLua for VnslDataType {
    fn to_lua(self, lua: &Lua) -> mlua::Result<Value> {
        match self {
            VnslDataType::String(s) => s.into_lua(lua),
            VnslDataType::Number(n) => n.into_lua(lua),
            VnslDataType::Bool(b) => b.into_lua(lua),
        }
    }
}
