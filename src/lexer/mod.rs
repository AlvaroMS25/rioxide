mod token;

use std::str::Lines;

use token::Token;

pub struct LocatedToken<'a> {
    line: u32,
    column: u32,
    token: Token<'a>
}

pub struct Lexer<'a> {
    inner: Lines<'a>
}

impl<'a> Lexer<'a> {
    pub fn new(lines: Lines<'a>) -> Self{
        Self {
            inner: lines
        }
    }

    pub fn parse_tokens(mut self) -> Vec<LocatedToken<'a>> {
        let mut out = Vec::new();

        for (line, content) in self.inner.enumerate() {
            let mut current_line = content.split(" ").enumerate();

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
    }
}
