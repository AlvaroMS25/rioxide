use std::{collections::{BTreeMap, LinkedList}, marker::PhantomData};

use crate::{lexer::Token, primitives::DataType};

/// Defines what an expression can be
#[derive(Debug)]
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

impl<'a> Expr<'a> {
    pub fn is_parenthesized(&self) -> bool {
        matches!(self, Self::Parenthesized(_))
    }

    pub fn is_primitive(&self) -> bool {
        matches!(self, Self::Primitive(_))
    }

    pub fn is_ident(&self) -> bool {
        matches!(self, Self::Ident(_))
    }

    pub fn is_quoted(&self) -> bool {
        matches!(self, Self::RawQuoted(_))
    }

    pub fn parenthesized_mut(&mut self) -> Option<&mut Tree<'a>> {
        if let Self::Parenthesized(t) = self {
            Some(t)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Tree<'a> {
    node: Option<Box<Expr<'a>>>,
    children: LinkedList<Expr<'a>>,
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
        self.children.push_back(item)
    }
}
