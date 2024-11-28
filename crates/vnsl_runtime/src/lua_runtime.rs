use mlua::{FromLuaMulti, IntoLua, Lua, Table, Value};
use vnsl_core::model::VnslDataType;

use crate::runtime_result::{RuntimeError, RuntimeResult};

const GLOBAL_KEY: &str = "globals";

#[derive(Debug)]
pub struct LuaRuntime {
    lua: Lua,
}

impl Default for LuaRuntime {
    fn default() -> Self {
        let lua = Lua::new();
        let globals_table = lua.create_table().expect("create globals table");
        lua.globals()
            .set(GLOBAL_KEY, globals_table)
            .expect("assign globals table");
        Self { lua }
    }
}

impl LuaRuntime {
    fn ensure_globals(&self) -> RuntimeResult<()> {
        let exist = self
            .lua
            .globals()
            .contains_key(GLOBAL_KEY)
            .map_err(|e| RuntimeError::LuaEvalError(e))?;
        if !exist {
            let globals_table = self
                .lua
                .create_table()
                .map_err(|e| RuntimeError::LuaEvalError(e))?;
            self.lua
                .globals()
                .set(GLOBAL_KEY, globals_table)
                .map_err(|e| RuntimeError::LuaEvalError(e))?;
        }

        Ok(())
    }

    pub fn set_globals_val_data_type(&self, key: &str, val: VnslDataType) -> RuntimeResult<()> {
        self.ensure_globals()?;
        let global_table = self.lua.globals().get::<Table>(GLOBAL_KEY)?;
        global_table
            .set(key, val.to_lua(&self.lua)?)
            .map_err(|e| RuntimeError::LuaEvalError(e))
    }

    pub fn set_globals_val<T: IntoLua>(&self, key: &str, val: T) -> RuntimeResult<()> {
        self.ensure_globals()?;
        let global_table = self.lua.globals().get::<Table>(GLOBAL_KEY)?;
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
