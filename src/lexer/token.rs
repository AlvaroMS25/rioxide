use crate::primitives::DataType;

/// Tokens used on racket
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
    /// Primitive data type
    Primitive(DataType<'a>),
    /// Function usage
    Function(&'a str),
    /// if
    If,
    /// cond
    Cond,
    /// else
    Else,
    /// let
    Let,
    /// let*
    LetAsterisk,
    /// for
    For,
    /// for*
    ForAsterisk,
    /// begin
    Begin,
    /// when
    When,
    /// unless
    Unless,
    /// Whitespace
    Whitespace
}

impl<'a> Token<'a> {
    pub fn needs_space(&self) -> bool {
        use Token::*;

        !matches!(
            self, 
            OpenParen | CloseParen | OpenBracket | CloseBracket | OpenBraces | CloseBraces
            | Whitespace
        )
    }

    pub fn try_single(item: &'a str) -> Option<Token<'a>> {
        Some(match item {
            "(" => Token::OpenParen,
            ")" => Token::CloseParen,
            "[" => Token::OpenBracket,
            "]" => Token::CloseBracket,
            "{" => Token::OpenBraces,
            "}" => Token::CloseBraces,
            " " => Token::Whitespace,
            _ => return None
        })
    }

    pub fn multiple(item: &'a str) -> Token<'a> {
        match item {
            "if" => Token::If,
            "cond" => Token::Cond,
            "else" => Token::Else,
            "let" => Token::Let,
            "let*" => Token::LetAsterisk,
            "for" => Token::For,
            "for*" => Token::ForAsterisk,
            "begin" => Token::Begin,
            "when" => Token::When,
            "unless" => Token::Unless,
            other => {
                if let Some(data) = DataType::parse(other) {
                    Token::Primitive(data)
                } else {
                    Token::Function(other)
                }
            }
        }
    }
}
