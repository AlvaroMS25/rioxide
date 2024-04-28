use crate::interpreter::any::Any;
use crate::interpreter::context::Context;
use crate::native::error::NativeFnError;
use crate::primitives::composed::{Composed, Pair};

pub fn cons<'a>(_: &Context<'a>, input: &Any<'a>) -> Result<Any<'a>, NativeFnError> {
    let expr = input.get_expression().map(|e| e.get_parenthesized())
        .flatten()
        .ok_or(NativeFnError::ArityMismatch {expected: 2, got: 1})?;

    if expr.children.len() > 2 {
        return Err(NativeFnError::ArityMismatch {expected: 2, got: expr.children.len() as u8});
    }

    Ok(Any::Composed(Composed::Pair(Pair {
        left: expr.children.get(0).unwrap().clone(),
        right: expr.children.get(1).unwrap().clone()
    })))
}