use std::f32::consts::E;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use crate::ast::expr::Expr;
use crate::display::InterpreterDisplay;
use crate::macros::get_enum;
use crate::primitives::composed::Composed;
use crate::primitives::DataType;

pub trait AnyDebug: std::any::Any + Debug + InterpreterDisplay {}

get_enum! {
    #[derive(Clone, Debug)]
    pub enum Any<'a> {
        Primitive(DataType<'a>),
        Composed(Box<Composed<'a>>),
        Expression(Expr<'a>),
        Void(()),
    }
}

impl<'a> From<Expr<'a>> for Any<'a> {
    fn from(value: Expr<'a>) -> Self {
        match value {
            Expr::Primitive(p) => Self::Primitive(p),
            other => Self::Expression(other)
        }       
    }
}

impl<'a> From<&Expr<'a>> for Any<'a> {
    fn from(value: &Expr<'a>) -> Self {
        Self::from(value.clone())
    }
}

impl<'a> From<&Box<Expr<'a>>> for Any<'a> {
    fn from(value: &Box<Expr<'a>>) -> Self {
        value.deref().into()
    }
}

impl Any<'_> {
    pub fn make_static(self) -> Any<'static> {
        use Any::*;
        match self {
            Primitive(p) => Primitive(p.make_static()),
            Composed(c) => Composed(Box::new(c.make_static())),
            Expression(e) => Expression(e.make_static()),
            Void(()) => Void(())
        }
    }
}
