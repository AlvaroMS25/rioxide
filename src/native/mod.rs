mod comparison;
pub mod error;
mod compose;
mod common;
pub mod function;

use error::NativeFnError;

use std::collections::HashMap;

use crate::{ast::expr::{Expr, Tree}, interpreter::{context::Context, Interpreter}};
use crate::primitives::any::Any;
use crate::macros::map_native_hashmap;
use crate::native::function::NativeFunction;

pub struct NativeStorage {
    table: HashMap<&'static str, NativeFunction>
}

impl NativeStorage {
    pub fn new() -> Self {
        Self {
            table: map_native_hashmap! {
                cons => compose::cons,
                define => common::define,
                list => compose::list,
                ast => common::ast,
                clear => common::clear_terminal,
                exit => common::exit
            }
        }
    }

    pub fn get(&self, item: &str) -> Option<&NativeFunction> {
        self.table.get(item)
    }
}
