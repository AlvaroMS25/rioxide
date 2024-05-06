use std::marker::PhantomData;
use crate::{ast::expr::{Expr, Tree}, native::NativeStorage};
use crate::interpreter::error::InterpreterError;
use crate::interpreter::Interpreter;
use crate::primitives::any::Any;

use super::vars::VarsStorage;

pub struct Context<'interpreter, 'inner> {
    interpreter: &'interpreter Interpreter<'inner>,
    local_variables: VarsStorage<'inner>
}

impl<'interpreter, 'inner> Context<'interpreter, 'inner> {
    pub fn new(interpreter: &'interpreter Interpreter<'inner>) -> Self {
        Self {
            interpreter,
            local_variables: VarsStorage::new(),
        }
    }

    pub fn interpreter(&self) -> &Interpreter<'inner> {
        &self.interpreter
    }

    pub fn level_down(&self) -> Self {
        Self {
            interpreter: self.interpreter,
            local_variables: self.local_variables.clone()
        }
    }

    pub fn eval(&mut self, expr: &Expr<'inner>) -> Result<Any<'inner>, InterpreterError> {
        todo!()
    }

    pub fn call_declared(
        &mut self,
        fun: &str,
        args: &[Any<'inner>]
    ) -> Result<Any<'inner>, InterpreterError> {
        let vars = self.interpreter.vars();
        let fun = vars.get(fun).unwrap().get_composed().unwrap().get_function().unwrap();

        todo!()
    }

    pub fn eval_tree(&mut self, tree: &Tree<'inner>) -> Result<Any<'inner>, InterpreterError> {
        let node = tree.node.as_ref().ok_or(InterpreterError::MissingTreeNode)?;

        let Some(fun) = node.get_ident() else {
            return Ok(Any::from(node));
        };

        let children = tree.children.iter().map(|c| self.eval(c))
            .collect::<Result<Vec<_>, InterpreterError>>()?;

        if self.interpreter.is_native(fun) {
            Ok(self.interpreter.storage.get(fun).unwrap().call(&mut self.level_down(), children.as_slice())?)
        } else if self.interpreter.is_declared_function(fun) {
            Ok(self.call_declared(fun, children.as_slice())?)
        } else {
            Err(InterpreterError::UndefinedFunction(fun.to_string()))
        }
    }

    pub fn get_local_var(&self, name: &str) -> Option<&Any<'inner>> {
        self.local_variables.get(name)
    }
}
