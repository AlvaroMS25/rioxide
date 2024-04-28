use thiserror::Error;

#[derive(Debug, Error)]
pub enum NativeFnError {
    #[error("Arity mismatch, expected: {expected}, got: {got} arguments")]
    ArityMismatch {
        expected: u8,
        got: u8
    }
}