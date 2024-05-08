use std::{collections::{BTreeMap, LinkedList}, marker::PhantomData};

use crate::{lexer::Token, primitives::DataType};
use crate::interpreter::context::Context;
use crate::interpreter::error::InterpreterError;
use crate::macros::get_enum;
use crate::primitives::any::Any;

get_enum! {
    /// Defines what an expression can be
    #[derive(Debug, Clone)]
    pub enum Expr<'a> {
        /// Parenthesized expression like (+ 2 3), this is a tree with root + and leaves 2 3
        Parenthesized(Tree<'a>),
        /// Any primitive
        Primitive(DataType<'a>),
        /// Raw identifier like "+" or a defined variable, basically anything not being a primitive
        /// and not being quoted
        Ident(&'a str),
        /// Quoted items
        RawQuoted(Box<Expr<'a>>),
    }
}

impl<'a> Expr<'a> {
    pub fn eval(&self, cx: &Context<'_, 'a>) -> Result<Any<'a>, InterpreterError> {
        todo!()
    }

    pub fn make_static(self) -> Expr<'static> {
        let this = match self {
            Self::Parenthesized(p) => Self::Parenthesized(p.make_static()),
            Self::Primitive(p) => Self::Primitive(p.make_static()),
            Self::Ident(ident) => {
                Self::Ident(Box::leak(ident.to_string().into_boxed_str()))
            },
            Self::RawQuoted(r) => Self::RawQuoted(Box::new(r.make_static()))
        };

        unsafe {
            std::mem::transmute::<_, Expr<'static>>(this)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tree<'a> {
    pub node: Option<Box<Expr<'a>>>,
    pub children: Vec<Expr<'a>>,
}

impl<'a> Tree<'a> {
    pub fn new() -> Self {
        Self {
            node: None,
            children: Default::default()
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            node: None,
            children: Vec::with_capacity(capacity)
        }
    }

    pub fn push_auto(&mut self, item: Expr<'a>) {
        if self.node.is_none() {
            self.set_node(item)
        } else {
            self.push(item)
        }
    }

    pub fn set_node(&mut self, item: Expr<'a>) {
        self.node = Some(Box::new(item));
    }

    pub fn push(&mut self, item: Expr<'a>) {
        self.children.push(item)
    }

    pub fn make_static(self) -> Tree<'static> {
        let mut new_tree = Tree::new();

        new_tree.node = self.node.map(|m| Box::new(m.make_static()));

        for item in self.children {
            new_tree.push(item.make_static());
        }

        new_tree
    }
}
