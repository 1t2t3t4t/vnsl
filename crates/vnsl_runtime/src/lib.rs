mod block_runner;
mod block_stack;
mod lua_runtime;
mod runtime;
mod runtime_result;

use lua_runtime::LuaRuntime;
pub use runtime::*;

pub trait RuntimeDelegateHandler {}

#[derive(Debug, Default)]
struct RunContext {
    lua_runtime: LuaRuntime,
}
