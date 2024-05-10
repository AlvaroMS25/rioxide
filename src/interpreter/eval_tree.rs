use std::{collections::HashMap, ops::Deref};

use crate::{ast::expr::{Expr, Tree}, primitives::any::Any};

use super::{context::Context, error::InterpreterError};

pub struct EvalTree<'a> {
    pub node: Option<Any<'a>>,
    pub children: Vec<Any<'a>>,
}

impl<'a> EvalTree<'a> {
    pub fn new(tree: &Tree<'a>, vars: &HashMap<&'a str, Option<&'a Any<'a>>>) -> EvalTree<'a> {
        let mut this = EvalTree {
            node: None,
            children: Vec::new()
        };

        if let Some(node) = &tree.node {
            this.node = Some(into_any(node, vars));
        }

        for child in &tree.children {
            this.children.push(into_any(child, vars));
        }

        this
    }

    pub fn new_singleton(source: &Tree<'a>) -> EvalTree<'a> {
        EvalTree {
            node: source.node.as_ref().map(|n| Any::from(n)),
            children: source.children.iter().map(Any::from).collect()
        }
    }

    pub fn evaluate(mut self, cx: &mut Context<'_, 'a>) -> Result<Any<'a>, InterpreterError> {
        //cx.eval_any(item)

        let mut args = Vec::with_capacity(self.children.len());

        for child in self.children {
            args.push(cx.level_down().eval_any(&child)?);
        }

        self.children = args;

        cx.eval_tree(&self)
    }
}

fn into_any<'a>(item: &Expr<'a>, vars: &HashMap<&'a str, Option<&'a Any<'a>>>) -> Any<'a> {
    match item {
        Expr::Ident(i) if vars.contains_key(i) => {
            vars.get(i).cloned().flatten().unwrap().clone()
        },
        other => Any::from(other)
    }
}
