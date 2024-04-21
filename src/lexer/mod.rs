mod cursor;
mod error;
mod token;

use std::{iter::{Enumerate, Map}, path::Iter, str::{Chars, Lines}};

use token::Token;

use self::{cursor::Cursor, error::LexerError};

#[derive(Debug)]
pub struct LocatedToken<'a> {
    line: u32,
    token: Token<'a>
}

pub struct Lexer<'a> {
    cursor: Cursor<'a>
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

                Ok(Token::multiple(buf))
            })?);
        }

        Ok(out)
    }

    fn parse_line(line_idx: u32, line: &'a str) -> Vec<LocatedToken<'a>> {
        println!("LINE: {line}");
        let mut curr_index = 0;

        let mut out = Vec::new();
        
        if line.is_empty() {
            return out;
        }

        while curr_index < line.len() {
            println!("Current idx: {curr_index}");
            let mut current_chunk = &line[curr_index..curr_index+1];
            println!("Sliced, chunk: {:?}", current_chunk);

            if let Some(token) = Token::try_single(current_chunk) {
                out.push(LocatedToken {
                    line: line_idx,
                    token
                });

                curr_index += 1;
                continue;
            }

            if (&line[curr_index..]).starts_with("\"") {
                
            }

            println!("Not single");

            if let Some(space_idx) = (&line[curr_index..]).find(" ") {
                current_chunk = &line[curr_index..=space_idx];
                curr_index = space_idx + 1;
            } else {
                current_chunk = &line[curr_index..];
                curr_index += 1;
            }

            out.push(LocatedToken {
                line: line_idx,
                token: Token::multiple(current_chunk)
            });

            println!("Pushed: {:?}", out.last().unwrap());
        }

        out
    }

    pub fn parse(self) -> Result<Vec<LocatedToken<'a>>, LexerError> {
        self.parse_all()
    }
}
