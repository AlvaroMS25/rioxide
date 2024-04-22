use thiserror::Error;

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("Option got None")]
    OptionNone,
    #[error("End of file")]
    Eof
}

pub trait OptionExt<T> {
    fn on(self) -> Result<T, LexerError>;
}

impl<T> OptionExt<T> for Option<T> {
    fn on(self) -> Result<T, LexerError> {
        self.ok_or(LexerError::OptionNone)
    }
}
