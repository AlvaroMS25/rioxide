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
    LetAskterisk,
    /// for
    For,
    /// for*
    ForAskterisk,
    /// begin
    Begin,
    /// when
    When,
    /// unless
    Unless,
    /// Whitespace
    Whitespace
}

impl<'a> From<&'a str> for Token<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "(" => Token::OpenParen,
            ")" => Token::CloseParen,
            "[" => Token::OpenBracket,
            "]" => Token::CloseBracket,
            "{" => Token::OpenBraces,
            "}" => Token::CloseBraces,
            "if" => Token::If,
            "cond" => Token::Cond,
            "else" => Token::Else,
            "let" => Token::Let,
            "let*" => Token::LetAskterisk,
            "for" => Token::For,
            "for*" => Token::ForAskterisk,
            "begin" => Token::Begin,
            "when" => Token::When,
            "unless" => Token::Unless,
            " " => Token::Whitespace,
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
