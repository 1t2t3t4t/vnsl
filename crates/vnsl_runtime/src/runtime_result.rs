use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("No scene loaded.")]
    NoSceneLoaded,
    #[error("End of stack.")]
    EndOfStack,
    #[error("Lua eval error for line {0}.\nError:{1:#?}")]
    LuaEvalError(String, mlua::Error),
    #[error("Lua error:{0:#?}")]
    LuaError(mlua::Error),
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;

impl From<mlua::Error> for RuntimeError {
    fn from(value: mlua::Error) -> Self {
        Self::LuaError(value)
    }
}
