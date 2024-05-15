use std::{collections::HashMap, ops::Deref};

use crate::{ast::expr::{Expr, Tree}, primitives::{any::Any, composed::{Function, LambdaFunction}}};
use crate::interpreter::any::AnyEval;

use super::{context::Context, error::InterpreterError};

#[derive(Clone, Debug)]
pub struct EvalTree<'a> {
    pub node: Option<AnyEval<'a>>,
    pub children: Vec<AnyEval<'a>>,
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
            node: source.node.as_ref().map(|n| AnyEval::from_expr(*n.clone())),
            children: source.children.iter().map(|c| AnyEval::from_expr(c.clone())).collect()
        }
    }

    pub fn make_static(self) -> EvalTree<'static> {
        EvalTree {
            node: self.node.map(|n| n.make_static()),
            children: self.children.into_iter().map(|c| c.make_static()).collect()
        }
    }

    pub fn evaluate(mut self, cx: &mut Context<'_, 'a>) -> Result<Any<'a>, InterpreterError> {
        //cx.eval_any(item)

        let mut args = Vec::with_capacity(self.children.len());

        for child in self.children {
            args.push(AnyEval::from_any(cx.level_down().eval(&child)?));
        }

        self.children = args;

        cx.eval_tree(&self)
    }

    pub fn ident_vec(&self) -> Vec<AnyEval<'a>> {
        let mut out = Vec::with_capacity(self.children.len() + 1);

        if let Some(node) = &self.node {
            if let Some(i) = node.get_ident() {
                out.push(AnyEval::Ident(*i));
            }
        }

        for i in &self.children {
            if let Some(i) = i.get_ident() {
                out.push(AnyEval::Ident(*i));
            }
        }

        out
    }

    pub fn try_parse_lambda(self) -> Result<LambdaFunction<'a>, InterpreterError> {
        let fun = Function::from_lambda("lambda", self)?;

        Ok(LambdaFunction {
            arity: fun.arity,
            body: fun.body
        })
    }
}

pub fn into_any<'a>(item: &Expr<'a>, vars: &HashMap<&'a str, Option<&'a Any<'a>>>) -> AnyEval<'a> {
    match item {
        Expr::Ident(i) if vars.contains_key(i) => {
            AnyEval::from_any(vars.get(i).cloned().flatten().unwrap().clone())
        },
        other => AnyEval::from_expr(other.clone())
    }
}
