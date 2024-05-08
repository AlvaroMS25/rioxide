pub mod context;
pub mod vars;
pub mod error;

use crate::display::InterpreterDisplay;

use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::ast::Ast;
use crate::ast::expr::{Expr, Tree};
use crate::cell::Cell;
use crate::interpreter::context::Context;
use crate::interpreter::error::InterpreterError;
use crate::interpreter::vars::VarsStorage;
use crate::native::NativeStorage;
use crate::primitives::any::Any;

use self::vars::OwnedStorage;

pub struct Interpreter<'a> {
    ast: Ast<'a>,
    storage: NativeStorage,
    pub(super) vars: Cell<OwnedStorage>,
}

impl<'a> Interpreter<'a> {
    pub fn new(ast: Ast<'a>) -> Self {
        Self {
            ast,
            storage: NativeStorage::new(),
            vars: Cell::new(OwnedStorage::new())
        }
    }

    pub fn with_vars(ast: Ast<'a>, vars: Cell<OwnedStorage>) -> Self {
        Self {
            ast,
            storage: NativeStorage::new(),
            vars
        }
    }

    pub fn context(&self) -> Context<'_, 'a> {
        Context::new(self)
    }

    pub fn vars(&self) -> &OwnedStorage {
        &self.vars
    }

    pub fn vars_mut(&self) -> &mut OwnedStorage {
        unsafe { self.vars.get_mut_unchecked() }
    }

    pub fn ast(&self) -> &Ast<'a> {
        &self.ast
    }

    pub fn is_native(&self, item: &str) -> bool {
        self.storage.get(item).is_some()
    }

    pub fn is_declared_function(&self, item: &str) -> bool {
        let vars = self.vars();
        let Some(dec) = vars.get(item) else { return false; };
        dec.get_composed().map(|c| c.is_function()).unwrap_or(false)
    }

    pub fn run(&self) -> Result<(), InterpreterError> {
        for expr in self.ast.inner.iter() {
            let mut writer = String::new();
            self.context()
                .eval(expr)?
                .fmt(&mut writer, self)
                .unwrap();
            println!("{writer}");
        }

        Ok(())
    }
}
