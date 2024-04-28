use std::{collections::{BTreeMap, LinkedList}, marker::PhantomData};

use crate::{lexer::Token, primitives::DataType};
use crate::macros::get_enum;

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
}
