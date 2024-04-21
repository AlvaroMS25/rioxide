use super::{token::Token, LocatedToken};

pub struct Cursor<'a> {
    buf: &'a str,
    line: usize,
    buf_position: usize
}

impl<'a> Cursor<'a> {
    pub fn new(buf: &'a str) -> Self {
        Self {
            buf,
            line: 1,
            buf_position: 0
        }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    fn update_line(&mut self) {
        let mut line = 0;

        let mut newlines = self.buf.as_bytes().iter()
            .enumerate()
            .filter(|(_, c)| **c == '\n' as u8)
            .map(|(idx, _)| idx);

        let Some(mut next_newline) = newlines.next().map(|l| l) else { return; };

        if self.buf_position == next_newline && self.buf.len() > self.buf_position + 1 {
            self.buf_position += 1; // if we on \n char, advance one
        }
        
        while self.buf_position > next_newline {
            line += 1;

            if let Some(next) = newlines.next() {
                next_newline = next;
            } else {
                break;
            }
        }

        self.line = line;
    }

    pub fn remaining(&self) -> &'a str {
        &self.buf[self.buf_position..]
    }

    pub fn parse_with<F, R, E>(&mut self, fun: F) -> Result<LocatedToken<'a>, E>
    where
        F: FnOnce(&'a str) -> Result<Token<'a>, E>,
    {
        let res = fun(self.remaining());

        if let Ok(token) = res.as_ref() {
            self.buf_position += token.token_len();
        }

        let ret = res.map(|t| LocatedToken { line: self.line as _, token: t });

        self.update_line();

        ret
    }
}
