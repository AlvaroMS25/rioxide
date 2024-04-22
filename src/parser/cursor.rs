use crate::lexer::LocatedToken;

pub struct Cursor<'a> {
    tokens: Vec<LocatedToken<'a>>,
    current_idx: usize
}

impl<'a> Cursor<'a> {
    pub fn new(tokens: Vec<LocatedToken<'a>>) -> Self {
        Self {
            tokens,
            current_idx: 0
        }
    }
}
