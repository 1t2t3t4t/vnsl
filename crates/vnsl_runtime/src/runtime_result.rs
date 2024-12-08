use std::fmt::Debug;

pub enum RuntimeError {
    NoSceneLoaded,
    LuaEvalError(String, mlua::Error),
    LuaError(mlua::Error),
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;

impl From<mlua::Error> for RuntimeError {
    fn from(value: mlua::Error) -> Self {
        Self::LuaError(value)
    }
}

impl Debug for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::NoSceneLoaded => writeln!(f, "No scene is loaded for current runtime"),
            RuntimeError::LuaEvalError(line, error) => {
                writeln!(f, "Lua eval error for line \"{}\"", line)?;
                writeln!(f, "{:#?}", error)
            }
            RuntimeError::LuaError(error) => writeln!(f, "Lua runtime error {:#?}", error),
        }
    }
}
