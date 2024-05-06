use crate::primitives::any::Any;
use crate::interpreter::context::Context;
use crate::interpreter::error::InterpreterError;
use crate::native::error::NativeFnError;
use crate::primitives::composed::{Composed, Pair};

pub fn cons<'a>(_: &mut Context, inputs: &[Any<'a>]) -> Result<Any<'a>, InterpreterError> {
    if inputs.len() > 2 {
        return Err(NativeFnError::ArityMismatch {expected: 2, got: inputs.len() as u8}.into());
    }

    Ok(Any::Composed(Box::new(Composed::Pair(Pair {
        left: inputs[0].clone(),
        right: inputs[1].clone()
    }))))
}
