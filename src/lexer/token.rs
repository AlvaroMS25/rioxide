use crate::{macros::swm, primitives::DataType};

/// Tokens used on racket
#[derive(Debug)]
pub enum Token<'a> {
    /// (
    OpenParen, 
    /// )
    CloseParen, 
    /// [
    OpenBracket, 
    /// ]
    CloseBracket,
    /// {
    OpenBraces,
    /// }
    CloseBraces,
    /// '
    SingleQuote,
    /// Primitive data type
    Primitive(DataType<'a>),
    /// Function usage
    Ident(&'a str),
    /// .
    Dot,
    /// Whitespace
    Whitespace,
    /// ; are comments in racket
    Comment
}

impl<'a> Token<'a> {
    pub fn token_len(&self) -> usize {
        use Token::*;
        
        match self {
            OpenParen | CloseParen | OpenBracket | CloseBracket | OpenBraces | CloseBraces | SingleQuote
            | Dot | Whitespace | Comment => 1,
            Ident(f) => f.len(),
            Primitive(p) => p.len(),
        }
    }

    pub fn try_single(item: &'a str) -> Option<Token<'a>> {
        Some(match item {
            "(" => Token::OpenParen,
            ")" => Token::CloseParen,
            "[" => Token::OpenBracket,
            "]" => Token::CloseBracket,
            "{" => Token::OpenBraces,
            "}" => Token::CloseBraces,
            "'" => Token::SingleQuote,
            "." => Token::Dot,
            " " => Token::Whitespace,
            ";" => Token::Comment,
            _ => return None
        })
    }

    pub fn multiple(item: &'a str) -> Token<'a> {
        if let Some(data) = DataType::parse(item) {
            Token::Primitive(data)
        } else {
            Self::parse_function(item)
        }
    }

    pub fn parse_function(item: &'a str) -> Token<'a> {
        let space = item.find(" ").unwrap_or(item.len());
        Token::Ident(&item[0..space])
    }
}
