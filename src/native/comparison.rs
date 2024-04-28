use crate::{ast::expr::{Expr, Tree}, interpreter::context::Context};
use crate::interpreter::any::Any;
use crate::primitives::composed::{Composed, Pair};

use super::error::NativeFnError;

pub fn eq<'a>(cx: &Context<'a>, input: &Any<'a>) -> Result<Any<'a>, NativeFnError> {
    todo!()
}