pub mod cursor;
mod error;
mod token;

use std::{iter::{Enumerate, Map}, path::Iter, str::{Chars, Lines}};

pub use {token::Token, error::LexerError};

use cursor::Cursor;

#[derive(Debug)]
pub struct LocatedToken<'a> {
    line: u32,
    token: Token<'a>
}

pub struct Lexer<'a> {
    cursor: Cursor<'a>
}

fn remove_last_tokens<'a>(item: &'a str) -> &'a str {
    println!("Item in: {}", item);
    if item.len() <= 1 {
        return item;
    }

    let mut finish_at = item.len() - 1;

    while finish_at > 0 && Token::try_single(&item[finish_at..finish_at+1]).is_some() {
        finish_at -= 1;
    }

    &item[..=finish_at]
}

impl<'a> Lexer<'a> {
    pub fn new(buf: &'a str) -> Self{
        Self {
            cursor: Cursor::new(buf)
        }
    }

    /*pub fn parse_tokens(self) -> Vec<LocatedToken<'a>> {
        let mut out = Vec::new();

        for (line, content) in self.inner.enumerate() {
            let current_line = content.split(" ").enumerate();

            if let Some(hint) = current_line.size_hint().1 {
                out.reserve(hint); // reserve if known length
            }

            for (column, chunk) in current_line {
                out.push(LocatedToken {
                    line: line as _,
                    column: column as _,
                    token: Token::from(chunk)
                })
            }
        }

        out
    }*/

    pub fn parse_all(mut self) -> Result<Vec<LocatedToken<'a>>, LexerError> {
        let mut out = Vec::new();

        while self.cursor.remaining().len() > 0 {
            out.push(self.cursor.parse_with::<_, LexerError, _>(|buf| {
                if buf.is_empty() {
                    return Err(LexerError::Eof);
                }

                if let Some(single) = Token::try_single(&buf[0..1]) {
                    return Ok(single);
                }

                Ok(Token::multiple(remove_last_tokens(buf)))
            })?);
        }

        Ok(out)
    }

    pub fn parse(self) -> Result<Vec<LocatedToken<'a>>, LexerError> {
        self.parse_all()
    }
}
