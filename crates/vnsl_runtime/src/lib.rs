mod block_runner;
mod lua_runtime;
mod result;
mod runstack;
mod runtime;
mod snapshot;

use lua_runtime::LuaRuntime;
pub use runtime::*;

#[derive(Debug, Default)]
pub struct RunContext {
    lua_runtime: LuaRuntime,
    current_character_id: Option<String>,
}
