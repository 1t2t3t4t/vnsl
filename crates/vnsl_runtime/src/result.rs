use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("No scene loaded.")]
    NoSceneLoaded,
    #[error("End of stack.")]
    EndOfStack,
    #[error("Label with name {0} cannot be found.")]
    InvalidLabelJump(String),
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

impl PartialEq for RuntimeError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RuntimeError::NoSceneLoaded, RuntimeError::NoSceneLoaded)
            | (RuntimeError::EndOfStack, RuntimeError::EndOfStack) => true,
            (RuntimeError::InvalidLabelJump(label_a), RuntimeError::InvalidLabelJump(label_b)) => {
                label_a == label_b
            }
            (
                RuntimeError::LuaEvalError(line_a, err_a),
                RuntimeError::LuaEvalError(line_b, err_b),
            ) => format!("{} {:?}", line_a, err_a) == format!("{} {:?}", line_b, err_b),
            (RuntimeError::LuaError(err_a), RuntimeError::LuaError(err_b)) => {
                format!("{:?}", err_a) == format!("{:?}", err_b)
            }
            _ => false,
        }
    }
}
