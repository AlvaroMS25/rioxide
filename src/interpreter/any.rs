use crate::ast::expr::Expr;
use crate::macros::get_enum;
use crate::primitives::composed::Composed;
use crate::primitives::DataType;

get_enum! {
    pub enum Any<'a> {
        Primitive(DataType<'a>),
        Composed(Composed<'a>),
        Expression(Expr<'a>),
        Other(Box<dyn std::any::Any>)
    }
}


