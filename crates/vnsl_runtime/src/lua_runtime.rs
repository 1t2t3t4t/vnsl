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
    fn test_get_val_vnsl_data_type_empty() {
        let runtime = LuaRuntime::default();
        assert_eq!(runtime.try_get_globals_val::<f64>("test"), None);
    }

    #[test]
    fn test_global_value_as_f64() {
        let runtime = LuaRuntime::default();
        runtime.set_globals_val("test", 20).unwrap();
        assert_eq!(runtime.try_get_globals_val::<f64>("test"), Some(20f64));
    }

    #[test]
    fn test_global_value_as_string() {
        let runtime = LuaRuntime::default();
        runtime.set_globals_val("test", 20).unwrap();
        assert_eq!(
            runtime.try_get_globals_val::<String>("test"),
            Some("20".to_string())
        );
    }

    #[test]
    fn test_global_value_with_custom_data_type() {
        let runtime = LuaRuntime::default();
        runtime
            .set_globals_val_data_type("test2", VnslDataType::String("yoo".to_string()))
            .unwrap();
        assert_eq!(
            runtime.try_get_globals_val::<String>("test2"),
            Some("yoo".to_string())
        );
    }

    #[test]
    fn test_global_value_as_bool() {
        let runtime = LuaRuntime::default();
        runtime.set_globals_val("test3", false).unwrap();
        assert_eq!(runtime.try_get_globals_val::<bool>("test3"), Some(false));
    }

    #[test]
    fn test_global_value_as_bool_to_string_fails() {
        let runtime = LuaRuntime::default();
        runtime.set_globals_val("test3", false).unwrap();
        assert_eq!(runtime.try_get_globals_val::<String>("test3"), None);
    }

    #[test]
    fn test_global_value_into_vnsl_data_type() {
        let runtime = LuaRuntime::default();
        runtime.set_globals_val("test", false).unwrap();
        assert_eq!(
            runtime
                .try_get_globals_val::<bool>("test")
                .map(|x| x.into()),
            Some(VnslDataType::Bool(false))
        );

        runtime.set_globals_val("test2", "someString").unwrap();
        assert_eq!(
            runtime
                .try_get_globals_val::<String>("test2")
                .map(|x| x.into()),
            Some(VnslDataType::String("someString".to_string()))
        );
    }
}
