mod token;

use std::{iter::{Enumerate, Map}, path::Iter, str::{Chars, Lines}};

use token::Token;

#[derive(Debug)]
pub struct LocatedToken<'a> {
    line: u32,
    token: Token<'a>
}

pub struct Lexer<'a> {
    iter: Enumerate<Lines<'a>>
}

impl<'a> Lexer<'a> {
    pub fn new(iter: Enumerate<Lines<'a>>) -> Self{
        Self {
            iter
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

    pub fn parse(self) -> Vec<LocatedToken<'a>> {
        let mut out = Vec::new();

        for(line, c) in self.iter {
            out.extend(Self::parse_line(line as _, c));
        }

        out
    }
}
