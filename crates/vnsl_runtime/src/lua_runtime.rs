use mlua::Lua;

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
    pub fn hi(&self) {
        println!("{:?}", self.lua)
    }
}
