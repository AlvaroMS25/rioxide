use thiserror::Error;

use crate::lexer::Token;

use self::expr::{Expr, Tree};

pub mod expr;

#[derive(Debug, Error)]
pub enum AstError {
    #[error("Missing closing delimiter")]
    MissingClosingDelimiter,
    #[error("Invalid expression")]
    InvalidExpression,
    #[error("Missing token")]
    MissingToken
}

#[derive(Debug)]
pub struct Ast<'a> {
    pub inner: Box<[Expr<'a>]>,
}

impl<'a> Ast<'a> {
    pub fn empty() -> Self {
        Self {
            inner: Box::from([])
        }
    }

    fn parse_quoted<I: Iterator<Item = Token<'a>>>(iter: &mut I) -> Result<Expr<'a>, AstError> {
        let next = iter.next().ok_or(AstError::MissingToken)?;

        let inner = match next {
            Token::OpenBraces | Token::OpenBracket | Token::OpenParen 
                => Expr::Parenthesized(Self::parse_tree(iter)?),
            other => Self::parse_token(other).ok_or(AstError::InvalidExpression)?
        };

        Ok(Expr::RawQuoted(Box::new(inner)))
    }

    fn parse_token(token: Token<'a>) -> Option<Expr<'a>> {
        Some(match token {
            Token::Ident(ident) => Expr::Ident(ident),
            Token::Primitive(prim) => Expr::Primitive(prim),
            _ => return None
        })
    }

    fn parse_tree<I>(iter: &mut I) -> Result<Tree<'a>, AstError> 
    where
        I: Iterator<Item = Token<'a>>
    {
        let mut tree = Tree::new();

        while let Some(token) = iter.next() {
            let parsed = match token {
                Token::OpenBraces | Token::OpenBracket | Token::OpenParen => {
                    Some(Expr::Parenthesized(Self::parse_tree(iter)?))
                },
                Token::CloseBraces | Token::CloseBracket | Token::CloseParen => {
                    return Ok(tree) // close expression
                },
                Token::SingleQuote => Some(Self::parse_quoted(iter)?),
                other => Self::parse_token(other)  
            };

            if let Some(p) = parsed {
                tree.push_auto(p);
            }
        }

        Err(AstError::MissingClosingDelimiter)
    }

    fn parse_expr<I>(iter: &mut I) -> Result<Option<Expr<'a>>, AstError> 
    where
        I: Iterator<Item = Token<'a>>
    {
        let Some(token) = iter.next() else { return Ok(None); };

        Ok(match token {
            Token::OpenBraces | Token::OpenBracket | Token::OpenParen
                => Some(Expr::Parenthesized(Self::parse_tree(iter)?)),
            Token::SingleQuote => Some(Expr::RawQuoted(Box::new({
                if let Some(e) = Self::parse_expr(iter)? {
                    e
                } else {
                    return Ok(None);
                }
            }))),
            other => Self::parse_token(other)
        })
    }

    fn parse<I>(mut iter: I) -> Result<Vec<Expr<'a>>, AstError>
    where
        I: ExactSizeIterator<Item = Token<'a>>
    {
        // preallocate a fourth of the size of the iterator, this is an arbitrary measure
        let mut out = Vec::with_capacity(iter.len() / 4);

        while let Some(next) = Self::parse_expr(&mut iter)? {
            out.push(next);
        }

        Ok(out)
    }
}

impl<'a> TryFrom<Vec<Token<'a>>> for Ast<'a> {
    type Error = AstError;

    fn try_from(value: Vec<Token<'a>>) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: Self::parse(value.into_iter())?.into_boxed_slice()
        })
    }
}
