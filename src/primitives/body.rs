use std::collections::HashMap;
use crate::ast::expr::{Expr, Tree};
use crate::interpreter::any::AnyEval;
use crate::interpreter::context::Context;
use crate::interpreter::error::InterpreterError;
use crate::interpreter::eval_tree::{EvalTree};
use crate::primitives::any::Any;
use crate::primitives::composed::FunctionBody;

impl<'a> FunctionBody<'a> {
    fn get_ident(ident: &str, vars: &HashMap<String, AnyEval<'a>>) -> Option<AnyEval<'a>> {
        vars.get(ident).cloned()
    }

    fn substitute_needed(item: AnyEval<'a>, vars: &HashMap<String, AnyEval<'a>>) -> AnyEval<'a> {
        use AnyEval::*;

        match &item {
            Ident(i) => return if let Some(node) = Self::get_ident(i, vars) {
                node
            } else {
                item
            },
            Expression(e) => Expression(Box::new(EvalTree {
                node: e.node.as_ref().map(|n| Self::substitute_needed(n.clone(), vars)),
                children: e.children.iter().map(|c| Self::substitute_needed(c.clone(), vars)).collect()
            })),
            other => other.clone(),
        }
    }

    fn prepare(&self, args: &[AnyEval<'a>]) -> Vec<AnyEval<'a>> {
        let map = self.args.iter().zip(args.iter())
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect::<HashMap<_, _>>();

        let mut out = Vec::with_capacity(self.body.len());

        for item in self.body.iter() {
            out.push(Self::substitute_needed(item.clone(), &map));
        }

        out
    }

    pub fn call(&self, cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
        let body = self.prepare(args);

        let len = body.len();
        let mut iter = body.into_iter().enumerate();

        while let Some((idx, expr)) = iter.next() {
            if idx == len - 1 {
                return cx.level_down().eval(&expr)
            } else {
                cx.level_down().eval(&expr)?;
            }
        }

        Ok(Any::Void(()))
    }
}
