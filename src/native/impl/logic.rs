use crate::interpreter::any::AnyEval;
use crate::interpreter::context::Context;
use crate::interpreter::error::InterpreterError;
use crate::macros::require_arity;
use crate::primitives::any::Any;
use crate::primitives::DataType;

fn boolean_value<'a>(cx: &mut Context<'_, 'a>, arg: &AnyEval<'a>) -> Result<(Any<'a>, bool), InterpreterError> {
    match cx.level_down().eval(arg)? {
        v @ Any::Primitive(DataType::Boolean(false)) => Ok((v, false)),
        other => Ok((other, true))
    }
}

pub fn and<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    if args.is_empty() {
        return Ok(Any::Primitive(DataType::Boolean(true)));
    }

    let mut value = None;

    for item in args.iter() {
        let (item, b) = boolean_value(cx, item)?;

        if b {
            value = Some(item);
        } else {
            return Ok(Any::Primitive(DataType::Boolean(false)));
        }
    }

    Ok(value.unwrap())
}

pub fn or<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    if args.is_empty() {
        return Ok(Any::Primitive(DataType::Boolean(false)));
    }

    let mut value = None;

    for item in args.iter() {
        let (item, b) = boolean_value(cx, item)?;

        if b {
            return Ok(item);
        } else {
            value = Some(item);
        }
    }

    Ok(Any::Primitive(DataType::Boolean(false)))
}

pub fn not<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    require_arity!(exact 1, args);

    boolean_value(cx, &args[0])
        .map(|(_, b)| Any::Primitive(DataType::Boolean(!b)))
}

pub fn nand<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    let args = &[AnyEval::from_any(and(cx, args)?)];
    not(cx, args)
}

pub fn nor<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    let args = &[AnyEval::from_any(or(cx, args)?)];
    not(cx, args)
}

pub fn xor<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    require_arity!(exact 2, args);
    let first = boolean_value(cx, &args[0])?;
    let second = boolean_value(cx, &args[1])?;

    Ok(match (first.1, second.1) {
        (true, false) => first.0,
        (false, true) => second.0,
        _ => Any::Primitive(DataType::Boolean(false))
    })
}
