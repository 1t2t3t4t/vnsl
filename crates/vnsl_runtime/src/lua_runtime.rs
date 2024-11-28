use mlua::{FromLuaMulti, Lua};

use crate::runtime_result::{RuntimeError, RuntimeResult};

#[derive(Debug)]
pub struct LuaRuntime {
    lua: Lua,
}

impl Default for LuaRuntime {
    fn default() -> Self {
        Self { lua: Lua::new() }
    }
}

impl LuaRuntime {
    pub fn eval_expr<T: FromLuaMulti>(&self, expr: &str) -> RuntimeResult<T> {
        self.lua
            .load(expr)
            .eval::<T>()
            .map_err(|e| RuntimeError::LuaEvalError(e))
    }
}
