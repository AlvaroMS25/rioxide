use std::collections::LinkedList;

use crate::{interpreter::{any::AnyEval, context::Context, error::InterpreterError}, native::{error::NativeFnError, function::{NativeFn, NativeFunction}}, primitives::{any::Any, composed::{Composed, Function, LambdaFunction, List}}};

use super::super::util::*;

pub fn map<'a>(
    cx: &mut Context<'_, 'a>,
    args: &[AnyEval<'a>]
) -> Result<Any<'a>, InterpreterError>
{
    if args.len() != 2 {
        return Err(NativeFnError::ArityMismatch { expected: 2, got: args.len() as _ }.into());
    }
    
    let fun = &args[0];
    let evaluated = cx.eval(&args[1])?;
    let items = evaluated.get_composed()
        .map(|i| i.get_list())
        .flatten()
        .ok_or(NativeFnError::UnexpectedType {
            function: "map",
            argument_position: 2,
            got: args[1].variant_name(),
            expected: "list"
        })?;

    let callable = callable_for(cx, fun)?;

    let mut result = LinkedList::new();

    for item in &items.0 {
        result.push_back(callable.call(cx, &[AnyEval::from_any(item.clone())])?)
    }

    Ok(Any::Composed(Box::new(Composed::List(List(result)))))
}
