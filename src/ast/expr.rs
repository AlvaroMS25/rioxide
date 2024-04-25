/*// (procedure a b..)
pub struct CallExpr<'a> {
    proc: &'a str,
    
}

pub enum ExprOperator<'a> {
    Expr(Box<Expr<'a>>),
    Primitive()
}*/

use std::collections::BTreeMap;

use crate::lexer::Token;

pub enum Expr<'a> {
    Token(Token<'a>),
    Tree(Tree<'a>),
}

pub struct Ast<'a> {
    trees: Box<[Tree<'a>]>,
}

pub struct Tree<'a> {
    tree: BTreeMap<usize, Expr<'a>>
}

impl<'a> Tree<'a> {
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::new(),
        }
    }

    pub fn push(&mut self, node: Expr<'a>) {
        let new_index = self.tree.last_entry().map(|e| (*e.key()) + 1).unwrap_or(0);

        self.tree.insert(new_index, node);
    }
}
