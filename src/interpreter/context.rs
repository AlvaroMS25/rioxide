use std::marker::PhantomData;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{ast::expr::{Expr, Tree}, cell::Cell, native::NativeStorage};
use crate::interpreter::error::InterpreterError;
use crate::interpreter::Interpreter;
use crate::native::error::DeclaredFunctionError;
use crate::primitives::any::Any;

use super::vars::VarsStorage;

pub struct Context<'interpreter, 'inner> {
    interpreter: &'interpreter Interpreter<'inner>,
    local_variables: Cell<VarsStorage<'inner>>,
    root: bool
}

impl<'interpreter, 'inner> Context<'interpreter, 'inner> {
    pub fn new(interpreter: &'interpreter Interpreter<'inner>) -> Self {
        Self {
            interpreter,
            local_variables: Cell::new(VarsStorage::new()),
            root: true
        }
    }

    pub fn interpreter(&self) -> &Interpreter<'inner> {
        &self.interpreter
    }

    pub fn vars_mut(&mut self) -> &mut VarsStorage<'inner> {
        if self.root {
            self.interpreter.vars_mut()
        } else {
            unsafe { self.local_variables.get_mut_unchecked() }
        }
    }

    pub fn vars(&self) -> &VarsStorage<'inner> {
        if self.root {
            self.interpreter.vars()
        } else{
            &self.local_variables
        }
    }

    pub fn level_down(&self) -> Self {
        Self {
            interpreter: self.interpreter,
            local_variables: self.local_variables.clone(),
            root: false
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

        if let Some(arity) = fun.arity {
            if args.len() != arity as _ {
                return Err(InterpreterError::DeclaredFnError(DeclaredFunctionError::ArityMismatch {
                    got: args.len() as _,
                    expected: arity
                }))
            }
        }

        

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
            Ok(self.interpreter.storage.get(fun).unwrap().call(self, children.as_slice())?)
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
