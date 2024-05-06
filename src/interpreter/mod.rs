pub mod context;
pub mod vars;
pub mod error;

use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::ast::Ast;
use crate::ast::expr::{Expr, Tree};
use crate::interpreter::context::Context;
use crate::interpreter::error::InterpreterError;
use crate::interpreter::vars::VarsStorage;
use crate::native::NativeStorage;
use crate::primitives::any::Any;

pub struct Interpreter<'a> {
    ast: Ast<'a>,
    storage: NativeStorage,
    vars: RwLock<VarsStorage<'a>>,
}

impl<'a> Interpreter<'a> {
    pub fn new(ast: Ast<'a>) -> Self {
        Self {
            ast,
            storage: NativeStorage::new(),
            vars: RwLock::new(VarsStorage::new())
        }
    }

    pub fn context(&self) -> Context<'_, 'a> {
        Context::new(self)
    }

    pub fn vars(&self) -> RwLockReadGuard<VarsStorage<'a>> {
        self.vars.read()
    }

    pub fn vars_mut(&self) -> RwLockWriteGuard<VarsStorage<'a>> {
        self.vars.write()
    }

    pub fn ast(&self) -> &Ast<'a> {
        &self.ast
    }

    pub fn is_native(&self, item: &str) -> bool {
        self.storage.get(item).is_some()
    }

    pub fn is_declared_function(&self, item: &str) -> bool {
        let v = self.vars();
        let Some(dec) = v.get(item) else { return false; };
        dec.get_composed().map(|c| c.is_function()).unwrap_or(false)
    }
}
