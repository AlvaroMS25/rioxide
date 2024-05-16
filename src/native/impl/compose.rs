use std::collections::LinkedList;
use crate::interpreter::any::AnyEval;

use crate::macros::require_arity;
use crate::primitives::any::Any;
use crate::interpreter::context::Context;
use crate::interpreter::error::InterpreterError;
use crate::native::error::NativeFnError;
use crate::primitives::composed::{Composed, List, Pair};

pub fn cons<'a>(cx: &mut Context<'_, 'a>, inputs: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    require_arity!(exact 2, inputs);

    Ok(Any::Composed(Box::new(Composed::Pair(Pair {
        left: cx.level_down().eval(&inputs[0])?,
        right: cx.level_down().eval(&inputs[1])?
    }))))
}

pub fn list<'a>(cx: &mut Context<'_, 'a>, inputs: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    let mut items = LinkedList::new();

    for item in inputs {
        items.push_back(cx.level_down().eval(item)?);
    }

    Ok(Any::Composed(Box::new(Composed::List(List(items)))))
}
