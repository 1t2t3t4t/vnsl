pub enum RuntimeError {
    LuaEvalError(mlua::Error),
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;
