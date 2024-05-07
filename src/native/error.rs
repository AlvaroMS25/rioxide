use thiserror::Error;

#[derive(Debug, Error)]
pub enum NativeFnError {
    #[error("Arity mismatch, expected: {expected}, got: {got} arguments")]
    ArityMismatch {
        expected: u8,
        got: u8
    },
    #[error("Expected identifier on {call}, got: {got}")]
    IdentifierExpectedIn {
        call: &'static str,
        got: String
    }
}

#[derive(Debug, Error)]
pub enum DeclaredFunctionError {
    #[error("Arity mismatch, expected: {expected}, got: {got} arguments")]
    ArityMismatch {
        expected: u8,
        got: u8
    },
    #[error("Invalid expression")]
    InvalidExpression
}
