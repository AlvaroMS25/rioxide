use thiserror::Error;
use crate::native::error::{DeclaredFunctionError, NativeFnError};

#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("Undefined function: {0}")]
    UndefinedFunction(String),
    #[error("Missing node on tree expression")]
    MissingTreeNode,
    #[error("Native fn error: {0}")]
    NativeError(#[from] NativeFnError),
    #[error("Declared function error: {0}")]
    DeclaredFnError(#[from] DeclaredFunctionError),
}
