use mlua::{FromLua, FromLuaMulti, IntoLua, Lua, Value};
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
    pub fn try_get_globals_val_data_type<V>(&self, key: &str) -> Option<VnslDataType>
    where
        V: FromLua + Into<VnslDataType>,
    {
        let Ok(val) = self.lua.globals().get::<V>(key) else {
            return None;
        };
        Some(val.into())
    }

    pub fn try_get_globals_val<V>(&self, key: &str) -> Option<V>
    where
        V: FromLua,
    {
        let Ok(val) = self.lua.globals().get::<V>(key) else {
            return None;
        };
        Some(val.into())
    }

    pub fn set_globals_val_data_type(&self, key: &str, val: VnslDataType) -> RuntimeResult<()> {
        self.lua
            .globals()
            .set(key, val.to_lua(&self.lua)?)
            .map_err(|e| RuntimeError::LuaError(e))
    }

    #[allow(dead_code)]
    pub fn set_globals_val<T: IntoLua + Into<VnslDataType>>(
        &self,
        key: &str,
        val: T,
    ) -> RuntimeResult<()> {
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

    pub fn exec_expr(&self, expr: &str) -> RuntimeResult<()> {
        self.lua
            .load(expr)
            .exec()
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

#[cfg(test)]
mod test {
    use vnsl_core::model::VnslDataType;

    use super::LuaRuntime;

    #[test]
    fn test_get_val() {
        let runtime = LuaRuntime::default();
        runtime.set_globals_val("test", 20).unwrap();

        let val = runtime.try_get_globals_val_data_type::<f64>("test");
        assert_eq!(val, Some(vnsl_core::model::VnslDataType::Number(20f64)));

        runtime
            .set_globals_val_data_type("test2", VnslDataType::String("yoo".to_string()))
            .unwrap();
        let val = runtime.try_get_globals_val_data_type::<String>("test2");
        assert_eq!(
            val,
            Some(vnsl_core::model::VnslDataType::String("yoo".to_string()))
        );
    }
}
