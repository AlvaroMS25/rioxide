use std::collections::LinkedList;

use crate::{interpreter::{any::AnyEval, context::Context, error::InterpreterError}, native::error::NativeFnError, primitives::any::Any};
use super::super::util::*;

pub fn fold_with<'a>(
    cx: &mut Context<'_, 'a>, 
    args: &[AnyEval<'a>],
    reverse: bool
) -> Result<Any<'a>, InterpreterError>
{
    if args.len() != 3 {
        return Err(NativeFnError::ArityMismatch { expected: 3, got: args.len() as _ }.into());
    }

    let fun = &args[0];
    let mut init = args[1].clone();

    let evaluated = cx.eval(&args[2])?;

    let items = evaluated.get_composed()
        .map(|i| i.get_list())
        .flatten()
        .ok_or(NativeFnError::UnexpectedType {
            function: if reverse { "foldr" } else { "foldl" },
            argument_position: 2,
            got: args[1].variant_name(),
            expected: "list"
        })?;

    let callable = callable_for(cx, fun)?;

    if let Some(arity) = callable.arity() {
        if arity != 2 {
            return Err(NativeFnError::ArityMismatch { expected: 2, got: arity }.into());
        }
    }
    
    if reverse {
        for item in items.0.clone().into_iter().rev() {
            init = AnyEval::from_any(callable.call(cx, &[init, AnyEval::from_any(item)])?);
        }
    } else {
        for item in items.0.clone().into_iter() {
            init = AnyEval::from_any(callable.call(cx, &[init, AnyEval::from_any(item)])?);
        }
    }

    Ok(Any::from(&init))
}

pub fn foldr<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    fold_with(cx, args, true)
}

pub fn foldl<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    fold_with(cx, args, false)
}
