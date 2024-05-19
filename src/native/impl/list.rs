use std::borrow::Cow;
use std::collections::LinkedList;

use crate::{interpreter::{any::AnyEval, context::Context, error::InterpreterError}, macros::require_arity, native::error::NativeFnError, primitives::{any::Any, composed::{Composed, List}, DataType}};
use crate::ext::LinkedListExt;

pub fn require_list<'a>(
    cx: &mut Context<'_, 'a>,
    arg: &AnyEval<'a>,
    fn_name: &'static str,
    position: u8,
) -> Result<List<'a>, InterpreterError>
{
    match cx.eval(arg)? {
        Any::Composed(c) if c.is_list() => {
            let Composed::List(l) = *c else {
                unsafe { std::hint::unreachable_unchecked(); };
            };
            Ok(l)
        },
        other => Err(NativeFnError::UnexpectedType {
            function: fn_name,
            argument_position: position,
            got: other.variant_name(),
            expected: "list"
        }.into())
    }
}

pub fn list_to_string<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    require_arity!(exact 1, args);

    let evaluated = cx.eval(&args[0])?;

    let items = evaluated.get_composed()
        .map(|p| p.get_list())
        .flatten()
        .ok_or(NativeFnError::UnexpectedType {
            function: "list->string",
            argument_position: 1,
            got: evaluated.variant_name(),
            expected: "list"
        })?;

    let mut buf = String::with_capacity(items.0.len());

    for item in items.0.iter() {
        let Any::Primitive(DataType::Character(c)) = item else {
            return Err(NativeFnError::InvalidType(item.variant_name().to_string()).into());
        };

        buf.push_str(&*DataType::character_to_string(c));
    }

    Ok(Any::Primitive(DataType::String(Cow::Owned(buf))))
}

pub fn len<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    require_arity!(exact 1, args);
    Ok(Any::Primitive(DataType::Integer(require_list(cx, &args[0], "length", 1)?.0.len() as _)))
}

pub fn list_ref<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    require_arity!(exact 2, args);

    let list = require_list(cx, &args[0], "list-ref", 1)?;
    let index = *args[1]
        .get_primitive()
        .map(|p| p.get_integer().map(|i| if *i >= 0 {
            Some(i)
        } else {
            None
        }))
        .flatten()
        .flatten()
        .ok_or(NativeFnError::UnexpectedType {
            function: "list-ref",
            argument_position: 1,
            got: args[1].variant_name(),
            expected: "non negative integer"
        })? as usize;
    
    let len = list.0.len();
    let item = list.0.get_owned(index)
        .ok_or(InterpreterError::OutOfBounds { length: len, got: index })?;

    Ok(item)
}

pub fn list_tail<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    require_arity!(exact 2, args);

    let list = require_list(cx, &args[0], "list-ref", 1)?;
    let index = *args[1]
        .get_primitive()
        .map(|p| p.get_integer().map(|i| if *i >= 0 {
            Some(i)
        } else {
            None
        }))
        .flatten()
        .flatten()
        .ok_or(NativeFnError::UnexpectedType {
            function: "list-ref",
            argument_position: 1,
            got: args[1].variant_name(),
            expected: "non negative integer"
        })? as usize;

    let item = list.0.into_iter().skip(index)
        .collect::<LinkedList<_>>();

    Ok(Any::Composed(Box::new(Composed::List(List(item)))))
}

pub fn append<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    require_arity!(at_least 2, args);
    let mut first = require_list(cx, &args[0], "append", 1)?;

    for(idx, item) in args.iter().enumerate().skip(1) {
        first.0.extend(require_list(cx, item, "append", idx as _)?.0.into_iter());
    }

    Ok(Any::Composed(Box::new(Composed::List(first))))
}

pub fn reverse<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    require_arity!(exact 1, args);

    let res = require_list(cx, &args[0], "reverse", 1)?
        .0
        .into_iter()
        .rev()
        .collect::<LinkedList<_>>();

    Ok(Any::Composed(Box::new(Composed::List(List(res)))))
}

pub fn build_list<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    todo!()
}
