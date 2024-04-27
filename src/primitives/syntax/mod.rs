use thiserror::Error;

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("Invalid syntax")]
    InvalidSyntax
}

pub trait SyntaxVerifier {
    fn verify(&self) -> Result<(), SyntaxError>;
}

struct A {}
impl SyntaxVerifier for A {
    fn verify(&self) -> Result<(), SyntaxError> {
        Ok(())
    }
}
