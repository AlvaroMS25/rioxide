use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use crate::ast::expr::Expr;
use crate::macros::get_enum;
use crate::primitives::composed::Composed;
use crate::primitives::DataType;

pub trait AnyDebug: std::any::Any + Debug {}

get_enum! {
    #[derive(Clone, Debug)]
    pub enum Any<'a> {
        Primitive(DataType<'a>),
        Composed(Box<Composed<'a>>),
        Expression(Expr<'a>),
        Void(()),
        Other(Arc<dyn AnyDebug>)
    }
}

impl<'a> Any<'a> {
}

impl<'a> From<&Expr<'a>> for Any<'a> {
    fn from(value: &Expr<'a>) -> Self {
        todo!()
    }
}

impl<'a> From<&Box<Expr<'a>>> for Any<'a> {
    fn from(value: &Box<Expr<'a>>) -> Self {
        value.deref().into()
    }
}