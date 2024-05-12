use crate::ast::expr::{Expr, Tree};
use crate::ext::StrExt;
use crate::interpreter::any::AnyEval::Primitive;
use crate::interpreter::eval_tree::EvalTree;
use crate::macros::get_enum;
use crate::primitives::any::Any;
use crate::primitives::composed::Composed;
use crate::primitives::DataType;

get_enum! {
    #[derive(Debug, Clone)]
    pub enum AnyEval<'a> {
        Primitive(DataType<'a>),
        Composed(Box<Composed<'a>>),
        Expression(Box<EvalTree<'a>>),
        Ident(&'a str),
        RawQuoted(Box<AnyEval<'a>>),
        Void(()),
    }
}

impl<'a> AnyEval<'a> {
    pub fn from_expr(expr: Expr<'a>) -> AnyEval<'a> {
        match expr {
            Expr::Ident(i) => AnyEval::Ident(i),
            Expr::Primitive(p) => AnyEval::Primitive(p),
            Expr::Parenthesized(t) => AnyEval::Expression(Box::new(EvalTree::new_singleton(&t))),
            Expr::RawQuoted(q) => AnyEval::RawQuoted(Box::new(Self::from_expr(*q)))
        }
    }

    pub fn make_static(self) -> AnyEval<'static> {
        use AnyEval::*;

        match self {
            Primitive(p) => Primitive(p.make_static()),
            Composed(c) => Composed(Box::new(c.make_static())),
            Expression(e) => Expression(Box::new(e.make_static())),
            Ident(i) => Ident(i.make_static()),
            RawQuoted(q) => RawQuoted(q.make_static()),
            Void(_) => Void(())
        }
    }

    pub fn from_any(item: Any<'a>) -> AnyEval<'a> {
        match item {
            Any::Primitive(p) => AnyEval::Primitive(p),
            Any::Composed(c) => AnyEval::Composed(c),
            Any::Expression(e) => Self::from_expr(e),
            Any::Void(_) => AnyEval::Void(())
        }
    }

    pub fn to_expr(self) -> Expr<'a> {
        match self {
            AnyEval::Expression(e) => {
                Expr::Parenthesized(Tree {
                    node: e.node.map(|i| i.to_expr()).map(Box::new),
                    children: e.children.into_iter().map(|i| i.to_expr()).collect()
                })
            },
            AnyEval::Ident(i) => Expr::Ident(i),
            AnyEval::RawQuoted(rq) => Expr::RawQuoted(Box::new(rq.to_expr())),
            AnyEval::Primitive(p) => Expr::Primitive(p),
            _ => unreachable!()
        }
    }
}
