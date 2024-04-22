use super::{token::Token, LocatedToken};

pub struct LexerCursor<'a> {
    buf: &'a str,
    line: usize,
    buf_position: usize
}

impl<'a> LexerCursor<'a> {
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
        let mut line = 1;

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

    fn ignore_newline(&mut self) -> &'a str {
        if self.buf[self.buf_position..].len() < 1 {
            return &self.buf[self.buf_position..];
        }

        let mut item = &self.buf[self.buf_position..];
        while item.len() > 1 && (&item[0..1] == "\n" || &item[0..1] == "\r") {
            item = &item[1..];
            self.buf_position += 1;
        }

        let mut finish = item.len();

        while finish > 0 && (&item[finish-1..finish] == "\n" || &item[finish-1..finish] == "\r") {
            finish -= 1;
        }

        &item[..finish]
    }

    pub fn remaining(&mut self) -> &'a str {
        self.ignore_newline()
    }

    pub fn parse_with<F, R, E>(&mut self, fun: F) -> Result<LocatedToken<'a>, E>
    where
        F: FnOnce(&'a str) -> Result<Token<'a>, E>,
    {
        let res = fun(self.remaining());

        if let Ok(token) = res.as_ref() {
            self.buf_position += token.token_len();
        }

        let ret = res.map(|t| LocatedToken { line: self.line as u32, token: t });

        self.update_line();

        ret
    }
}
