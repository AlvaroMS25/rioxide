use std::collections::LinkedList;
use crate::ast::expr::Expr;
use crate::macros::get_enum;

pub struct List<'a>(pub LinkedList<Expr<'a>>);

pub struct Function<'a>(pub LinkedList<Expr<'a>>);

pub struct Symbol<'a>(pub &'a str);

pub struct Pair<'a> {
    pub left: Expr<'a>,
    pub right: Expr<'a>
}

get_enum! {
    pub enum Composed<'a> {
        List(List<'a>),
        Function(Function<'a>),
        Symbol(Symbol<'a>),
        Pair(Pair<'a>)
    }
}
