use mlua::{FromLuaMulti, IntoLua, Lua, Table};

use crate::runtime_result::{RuntimeError, RuntimeResult};

#[derive(Debug)]
pub struct LuaRuntime {
    lua: Lua,
}

impl Default for LuaRuntime {
    fn default() -> Self {
        let lua = Lua::new();
        let globals_table = lua.create_table().expect("create globals table");
        lua.globals().set("globals", globals_table).unwrap();
        Self { lua }
    }
}

impl LuaRuntime {
    pub fn set_global<T: IntoLua>(&self, key: &str, val: T) -> RuntimeResult<()> {
        let global_table = self.lua.globals().get::<Table>("globals")?;
        global_table
            .set(key, val)
            .map_err(|e| RuntimeError::LuaEvalError(e))
    }

    pub fn eval_expr<T: FromLuaMulti>(&self, expr: &str) -> RuntimeResult<T> {
        self.lua
            .load(expr)
            .eval::<T>()
            .map_err(|e| RuntimeError::LuaEvalError(e))
    }
}
