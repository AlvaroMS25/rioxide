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
        println!("Called substitute with {item:?}");
        todo!()

        /*let Any::Expression(expr) = &item else { return AnyEval::from_any(item); };

        match expr {
            Expr::Ident(i) => return if let Some(item) = Self::get_ident(*i, vars) {
                item
            } else {
                AnyEval::from_any(item)
            },
            Expr::Parenthesized(tree) => AnyEval::Expression(Box::new(EvalTree::new_singleton(tree))),
            _ => AnyEval::from_any(item)
        }*/
    }

    fn prepare(&self, args: &[AnyEval<'a>]) -> Vec<AnyEval<'a>> {
        println!("Before preparing: {:?}", self.body);

        let map = self.args.iter().zip(args.iter())
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect::<HashMap<_, _>>();

        println!("Args map: {map:?}");

        let mut out = Vec::with_capacity(self.body.len());

        for item in self.body.iter() {
            out.push(Self::substitute_needed(item.clone(), &map));
        }

        println!("After preparing: {out:?}");
        out
    }

    pub fn call(&self, cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
        self.prepare(args);

        let len = self.body.len();
        let mut iter = self.body.iter().enumerate();

        while let Some((idx, expr)) = iter.next() {
            if idx == len - 1 {
                return cx.level_down().eval(&expr)
            } else {
                cx.level_down().eval(expr)?;
            }
        }

        Ok(Any::Void(()))
    }
}
