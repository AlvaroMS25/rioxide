use std::collections::LinkedList;
use clap::arg;
use crate::ast::expr::Expr;
use crate::interpreter::context::Context;
use crate::interpreter::error::InterpreterError;
use crate::macros::get_enum;
use crate::native::error::DeclaredFunctionError;
use crate::primitives::any::Any;

#[derive(Clone, Debug)]
pub struct List<'a>(pub LinkedList<Any<'a>>);

#[derive(Clone, Debug)]
pub struct Function<'a> {
    pub body: Expr<'a>,
    pub arity: Option<u8>
}

#[derive(Clone, Debug)]
pub struct Symbol<'a>(pub &'a str);

#[derive(Clone, Debug)]
pub struct Pair<'a> {
    pub left: Any<'a>,
    pub right: Any<'a>
}

get_enum! {
    /// Data types composed by more of a single item
    #[derive(Clone, Debug)]
    pub enum Composed<'a> {
        List(List<'a>),
        Function(Function<'a>),
        Symbol(Symbol<'a>),
        Pair(Pair<'a>)
    }
}

impl<'a> Function<'a> {
    pub fn is_anonymous(&self) -> bool {
        if let Some(ident) = self.body.get_ident() {
            ident == &"lambda"
        } else {
            false
        }
    }        

    pub fn find_replace(&self, cx: &mut Context<'_, 'a>) -> Result<Any<'a>, InterpreterError> {
        todo!()
    }
}
