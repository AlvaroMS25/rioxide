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
    },
    #[error("Wrong argument on function {function} at position {argument_position}, got {got}, expected {expected}")]
    UnexpectedType {
        function: &'static str,
        argument_position: u8,
        got: &'static str,
        expected: &'static str
    },
    #[error("Invalid operands provided: {expected}")]
    InvalidOperands {
        expected: &'static str,
    },
    #[error("Feature not yet implemented: {0}")]
    NotYetImplemented(&'static str)
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
