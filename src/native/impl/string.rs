use std::borrow::Cow;
use crate::{interpreter::{any::AnyEval, context::Context, error::InterpreterError}, primitives::any::Any};
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
