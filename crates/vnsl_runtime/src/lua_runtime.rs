use std::collections::HashMap;

use mlua::{FromLua, FromLuaMulti, IntoLua, Lua, Table, Value};
use vnsl_core::model::VnslDataType;

use crate::result::{RuntimeError, RuntimeResult};

#[derive(Debug)]
pub struct LuaRuntime {
    lua: Lua,
    global_table: Table,
}

impl Default for LuaRuntime {
    fn default() -> Self {
        let lua = Lua::new();
        let global_table = lua.create_table().expect("create global table");
        lua.globals()
            .set("global", global_table.clone())
            .expect("assign global table to lua");
        Self { lua, global_table }
    }
}

impl LuaRuntime {
    pub fn try_get_globals_val<V>(&self, key: &str) -> Option<V>
    where
        V: FromLua,
    {
        let Ok(val) = self.global_table.get::<V>(key) else {
            return None;
        };
        Some(val.into())
    }

    pub fn set_globals_val_data_type(&self, key: &str, val: VnslDataType) -> RuntimeResult<()> {
        self.set_globals_val(key, val)
    }

    pub fn set_globals_val<T: IntoLua + Into<VnslDataType>>(
        &self,
        key: &str,
        val: T,
    ) -> RuntimeResult<()> {
        self.global_table
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

    pub fn import_globals(&self, globals: HashMap<String, VnslDataType>) -> RuntimeResult<()> {
        for (k, v) in globals.into_iter() {
            let val = v.into_lua(&self.lua)?;
            self.global_table.set(k, val)?;
        }
        Ok(())
    }

    pub fn export_globals(&self) -> HashMap<String, VnslDataType> {
        let pairs = self.global_table.pairs::<String, Value>();
        pairs
            .into_iter()
            .filter_map(|p| p.ok())
            .map(|p| (p.0, VnslDataType::try_from(p.1)))
            .filter_map(|p| {
                if let Some(val) = p.1 {
                    Some((p.0, val))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

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

    #[test]
    fn test_export_globals() {
        let rt = LuaRuntime::default();
        rt.set_globals_val("a", "val").unwrap();
        rt.set_globals_val("b", 20).unwrap();
        rt.set_globals_val("c", 30.123).unwrap();
        rt.set_globals_val("d", true).unwrap();

        let globals = rt.export_globals();
        let mut map = HashMap::new();
        map.insert("a".to_string(), VnslDataType::String("val".to_string()));
        map.insert("b".to_string(), VnslDataType::Number(20.));
        map.insert("c".to_string(), VnslDataType::Number(30.123));
        map.insert("d".to_string(), VnslDataType::Bool(true));
        assert_eq!(globals, map);
    }
}
