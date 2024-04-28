mod comparison;
mod error;
mod compose;

use error::NativeFnError;

use std::collections::HashMap;

use crate::{ast::expr::{Expr, Tree}, interpreter::{context::Context, Interpreter}};
use crate::interpreter::any::Any;
use crate::macros::hashmap;

/// Functions that get executed natively by the interpreter. Functions will receive the whole call
/// tree, where the node is the called function, and the children are its arguments
pub type NativeFn<'a> = fn(&Context<'a>, &Any<'a>) -> Result<Any<'a>, NativeFnError>;

pub struct NativeStorage<'a> {
    table: HashMap<&'static str, NativeFn<'a>>
}

impl<'a> NativeStorage<'a> {
    pub fn new() -> Self {
        Self {
            table: hashmap! {
                cons => compose::cons
            }
        }
    }
}
