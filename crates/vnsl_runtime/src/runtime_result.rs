#[derive(Debug)]
pub enum RuntimeError {
    LuaEvalError(mlua::Error),
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;

impl From<mlua::Error> for RuntimeError {
    fn from(value: mlua::Error) -> Self {
        Self::LuaEvalError(value)
    }
}
