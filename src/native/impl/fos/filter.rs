use std::collections::LinkedList;

use crate::{interpreter::{any::AnyEval, context::Context, error::InterpreterError}, native::error::NativeFnError, primitives::{any::Any, composed::{Composed, List}, DataType}};

use super::super::util::*;

pub fn filter<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    if args.len() != 2 {
        return Err(NativeFnError::ArityMismatch { expected: 2, got: args.len() as _ }.into());
    }

    let fun = callable_for(cx, &args[0], "filter", 1)?;

    let evaluated = cx.eval(&args[1])?;
    let items = evaluated.get_composed()
        .map(|i| i.get_list())
        .flatten()
        .ok_or(NativeFnError::UnexpectedType {
            function: "filter",
            argument_position: 2,
            got: args[1].variant_name(),
            expected: "list"
        })?;

    let mut result = LinkedList::new();

    for item in items.0.clone().into_iter() {
        let res = match fun.call(cx, &[AnyEval::from_any(item.clone())])? {
            Any::Primitive(DataType::Boolean(b)) => b,
            other => return Err(NativeFnError::InvalidType(format!(
                "Boolean required on predicate, found {}",
                other.variant_name()
            )).into())
        };

        if res {
            result.push_back(item);
        }
    }

    Ok(Any::Composed(Box::new(Composed::List(List(result)))))
}