use std::collections::HashMap;
use crate::ast::expr::{Expr, Tree};
use crate::interpreter::context::Context;
use crate::interpreter::error::InterpreterError;
use crate::interpreter::eval_tree::{EvalTree};
use crate::primitives::any::Any;
use crate::primitives::composed::FunctionBody;

impl<'a> FunctionBody<'a> {
    fn get_ident(ident: &str, vars: &HashMap<String, Any<'a>>) -> Option<Any<'a>> {
        vars.get(ident).cloned()
    }

    fn substitute_needed(item: Any<'a>, vars: &HashMap<String, Any<'a>>) -> Any<'a> {
        println!("Called substitute with {item:?}");

        let Any::Expression(expr) = &item else { return item; };

        match expr {
            Expr::Ident(i) => return if let Some(item) = Self::get_ident(*i, vars) {
                item
            } else {
                item
            },
            Expr::Parenthesized(tree) => {
                todo!()
            },
            _ => item
        }
        /*match item {
            Any::Expression(Expr::Ident(i)) if vars.contains_key(i) => {
                vars.get(i).cloned().unwrap()
            },
            other => other
        }*/
    }

    fn prepare(&mut self, args: &[Any<'a>]) {
        println!("Before preparing: {:?}", self.body);

        let map = self.args.iter().zip(args.iter())
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect::<HashMap<_, _>>();

        println!("Args map: {map:?}");

        for item in self.body.iter_mut() {
            let prev = std::mem::replace(item, Any::Void(()));
            let _ = std::mem::replace(item, Self::substitute_needed(prev, &map));
        }

        println!("After preparing: {:?}", self.body);
    }

    pub fn call(mut self, cx: &mut Context<'_, 'a>, args: &[Any<'a>]) -> Result<Any<'a>, InterpreterError> {
        self.prepare(args);

        let len = self.body.len();
        let mut iter = self.body.into_iter().enumerate();

        while let Some((idx, expr)) = iter.next() {
            if idx == len - 1 {
                return cx.level_down().eval_any(&expr)
            } else {
                cx.level_down().eval_any(&expr)?;
            }
        }

        Ok(Any::Void(()))
    }
}
