use std::{borrow::Cow, collections::LinkedList};
use crate::{interpreter::{any::AnyEval, context::Context, error::InterpreterError}, primitives::{any::Any, composed::{Composed, List}}};
use crate::macros::require_arity;
use crate::native::error::NativeFnError;
use crate::primitives::DataType;

pub fn require_string<'a>(
    cx: &mut Context<'_, 'a>,
    arg: &AnyEval<'a>,
    fn_name: &'static str,
    position: u8,
) -> Result<Cow<'a, str>, InterpreterError>
{
    match cx.eval(arg)? {
        Any::Primitive(DataType::String(s)) => Ok(s),
        other => Err(NativeFnError::UnexpectedType {
            function: fn_name,
            argument_position: position,
            got: other.variant_name(),
            expected: "string"
        }.into())
    }
}

pub fn is_string<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    require_arity!(exact 1, args);

    let string = cx.eval(&args[0])?
        .get_primitive()
        .map(|p| p.get_string())
        .flatten()
        .map(|s| true)
        .unwrap_or(false);

    Ok(Any::Primitive(DataType::Boolean(string)))
}

pub fn string_append<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    require_arity!(at_least 2, args);

    let mut first = require_string(cx, &args[0], "string-append", 1)?.to_string();

    for (idx, item) in args.iter().skip(1).enumerate() {
        first.push_str(&*require_string(cx, item, "string-append", idx as _)?);
    }

    Ok(Any::Primitive(DataType::String(Cow::Owned(first))))
}

pub fn make_string<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    require_arity!(exact 2, args);

    let times = *args[0]
        .get_primitive()
        .map(|p| p.get_integer())
        .flatten()
        .ok_or(NativeFnError::UnexpectedType {
            function: "make-string",
            argument_position: 1,
            got: args[0].variant_name(),
            expected: "Integer"
        })?;

    let character = args[1]
        .get_primitive()
        .map(|p| p.get_character())
        .flatten()
        .ok_or(NativeFnError::UnexpectedType {
            function: "make-string",
            argument_position: 1,
            got: args[1].variant_name(),
            expected: "character"
        })?;
    
    let mut result = String::with_capacity(times as usize);

    for _ in 0..times {
        result.push_str(&*character);
    }

    Ok(Any::Primitive(DataType::String(Cow::Owned(result))))
}

pub fn len<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    require_arity!(exact 1, args);

    Ok(Any::Primitive(DataType::Integer(
        require_string(cx, &args[0], "string-length", 1)?.len() as i32
    )))
}

pub fn string_ref<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    require_arity!(exact 2, args);

    let string = require_string(cx, &args[0], "string-ref", 1)?;
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
            function: "string-ref",
            argument_position: 1,
            got: args[1].variant_name(),
            expected: "non negative integer"
        })? as usize;

    Ok(Any::Primitive(DataType::Character(Cow::Owned(string[index..index+1].to_string()))))
}

pub fn substring<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    require_arity!(exact 3, args);

    let string = require_string(cx, &args[0], "substring", 1)?;

    let start = *args[1]
        .get_primitive()
        .map(|p| p.get_integer().map(|i| if *i >= 0 {
            Some(i)
        } else {
            None
        }))
        .flatten()
        .flatten()
        .ok_or(NativeFnError::UnexpectedType {
            function: "string-ref",
            argument_position: 1,
            got: args[1].variant_name(),
            expected: "non negative integer"
        })? as usize;

    let end = args[2]
        .get_primitive()
        .map(|p| p.get_integer().map(|i| if *i >= 0 {
            Some(i)
        } else {
            None
        }))
        .flatten()
        .flatten()
        .map(|i| *i as usize);

    let res = if let Some(end) = end {
        string[start..end].to_string()
    } else {
        string[start..].to_string()
    };

    Ok(Any::Primitive(DataType::String(Cow::Owned(res))))
}

pub fn string_to_list<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    require_arity!(exact 1, args);
    let string = require_string(cx, &args[0], "string->list", 1)?;

    let mut result = LinkedList::new();

    for i in string.chars() {
        result.push_back(Any::Primitive(DataType::Character(Cow::Owned(i.to_string()))));
    }

    Ok(Any::Composed(Box::new(Composed::List(List(result)))))
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

        buf.push_str(*&c);
    }

    Ok(Any::Primitive(DataType::String(Cow::Owned(buf))))
}
