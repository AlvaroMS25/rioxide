use std::{collections::HashMap, marker::PhantomData, ops::Deref};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{ast::expr::{Expr, Tree}, cell::Cell, container::VarsContainer, native::NativeStorage};
use crate::interpreter::error::InterpreterError;
use crate::interpreter::Interpreter;
use crate::native::error::DeclaredFunctionError;
use crate::primitives::any::Any;

use super::{eval_tree::EvalTree, vars::{OwnedStorage, VarsStorage}};

pub struct Context<'interpreter, 'inner> {
    interpreter: &'interpreter Interpreter<'inner>,
    local_variables: Cell<VarsStorage<'inner>>,
    root: bool,
}

impl<'interpreter, 'inner> Context<'interpreter, 'inner> {
    pub fn new(interpreter: &'interpreter Interpreter<'inner>) -> Self {
        Self {
            interpreter,
            local_variables: Cell::new(VarsStorage::new()),
            root: true,
        }
    }

    pub fn interpreter(&self) -> &Interpreter<'inner> {
        &self.interpreter
    }

    pub fn vars_mut(&mut self) -> &mut dyn VarsContainer<'inner> {
        if self.root {
            self.interpreter.vars_mut()
        } else {
            unsafe { self.local_variables.get_mut_unchecked() }
        }
    }

    pub fn vars(&self) -> &dyn VarsContainer<'inner> {
        if self.root {
            self.interpreter.vars()
        } else{
            self.local_variables.deref()
        }
    }

    pub fn local_vars(&self) -> &VarsStorage<'inner> {
        &self.local_variables
    }

    pub fn global_vars(&self) -> &OwnedStorage {
        self.interpreter.vars()
    }

    pub fn get_var(&self, var: &str) -> Option<&Any<'inner>> {
        self.local_variables.get(var)
            .or_else(|| self.interpreter.vars().get(var))
    }

    pub fn level_down(&self) -> Self {
        Self {
            interpreter: self.interpreter,
            local_variables: self.local_variables.clone(),
            root: false,
        }
    }

    pub fn is_declared_function(&self, key: &str) -> bool {
        self.local_variables.get(key).is_some()
            || self.interpreter().vars().get(key).is_some()
    }

    pub fn eval_any(&mut self, item: &Any<'inner>) -> Result<Any<'inner>, InterpreterError> {
        /*match item {
            Any::Expression(expr) => self.eval(expr),
            other => Ok(other.clone())
        }*/
        self.eval(item)
    }

    

    pub fn eval_expr(&mut self, expr: &Expr<'inner>) -> Result<Any<'inner>, InterpreterError> {
        self.eval(&Any::from(expr))
    }

    pub fn eval(&mut self, expr: &Any<'inner>) -> Result<Any<'inner>, InterpreterError> {
        let Some(expr) = expr.get_expression() else { return Ok(expr.clone()); };

        match expr {
            Expr::Parenthesized(tree) => self.eval_tree(&EvalTree::new_singleton(tree)),
            other => Ok(Any::from(other))
        }
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

        let mut iter = fun.body.body.iter().enumerate();

        while let Some((idx, expr)) = iter.next() {
            if idx == fun.body.body.len() - 1 {
                return self.call_maybe_tree(expr)
            } else {
                self.call_maybe_tree(expr)?;
            }
        }

        Ok(Any::Void(()))
    }

    pub fn eval_tree(&mut self, tree: &EvalTree<'inner>) -> Result<Any<'inner>, InterpreterError> {
        let node = tree.node.as_ref().ok_or(InterpreterError::MissingTreeNode)?;

        let Some(fun) = node.get_expression().map(|n| n.get_ident()).flatten() else {
            return Ok(node.clone());
        };

        let children = tree.children.iter().map(|c| self.eval(&c))
            .collect::<Result<Vec<_>, InterpreterError>>()?;

        if self.interpreter.is_native(fun) {
            Ok(self.interpreter.storage.get(fun).unwrap().call(self, children.as_slice())?)
        } else if self.is_declared_function(fun) {
            Ok(self.call_declared(fun, children.as_slice())?)
        } else {
            Err(InterpreterError::UndefinedFunction(fun.to_string()))
        }
    }

    fn call_maybe_tree(&mut self, expr: &Expr<'inner>) -> Result<Any<'inner>, InterpreterError> {
        match expr {
            Expr::Parenthesized(p) => self.call_eval_tree(p),
            other => Ok(Any::from(other))
        }
    }

    fn call_eval_tree(&mut self, tree: &Tree<'inner>) -> Result<Any<'inner>, InterpreterError> {
        let mut map = HashMap::new();

        for (k, v) in &self.interpreter.vars().table {
            map.insert(k.as_str(), Some(v.deref()));
        }

        for (k, v) in &self.local_variables.table {
            map.insert(k, Some(v.deref()));
        }

        EvalTree::new(&tree, unsafe { std::mem::transmute(&map) })
            .evaluate(self)
    }

    pub fn get_local_var(&self, name: &str) -> Option<&Any<'inner>> {
        self.local_variables.get(name)
    }
}
