use crate::{interpreter::{any::AnyEval, context::Context, error::InterpreterError}, native::error::NativeFnError, primitives::{any::Any, ops::ComparisonOperator, DataType}};

use super::comparison::with_comparable_window2;

pub fn create_comparable(item: &Any<'_>) -> Result<ComparisonOperator, NativeFnError> {
    let Any::Primitive(item) = item else { 
        return Err(NativeFnError::InvalidType(item.variant_name().to_string()));
    };

    ComparisonOperator::from_primitive(item)
        .ok_or(NativeFnError::InvalidType(item.variant_name().to_string()))
}

pub fn add<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    if args.len() < 2 {
        return Err(NativeFnError::ArityMismatch { expected: 2, got: args.len() as _ }.into())
    }

    let mut items = args.iter().map(|i| cx.level_down().eval(i))
        .collect::<Result<Vec<Any<'a>>, InterpreterError>>()?.into_iter();

    let mut first = create_comparable(&items.next().unwrap())?;

    for i in items {
        first = (first + create_comparable(&i)?)?;
    }

    Ok(Any::Primitive(first.to_datatype()))
}

pub fn sub<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    if args.len() < 2 {
        return Err(NativeFnError::ArityMismatch { expected: 2, got: args.len() as _ }.into())
    }

    let mut items = args.iter().map(|i| cx.level_down().eval(i))
        .collect::<Result<Vec<Any<'a>>, InterpreterError>>()?.into_iter();

    let mut first = create_comparable(&items.next().unwrap())?;

    for i in items {
        first = (first - create_comparable(&i)?)?;
    }

    Ok(Any::Primitive(first.to_datatype()))
}

pub fn mul<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    if args.len() < 2 {
        return Err(NativeFnError::ArityMismatch { expected: 2, got: args.len() as _ }.into())
    }

    let mut items = args.iter().map(|i| cx.level_down().eval(i))
        .collect::<Result<Vec<Any<'a>>, InterpreterError>>()?.into_iter();

    let mut first = create_comparable(&items.next().unwrap())?;

    for i in items {
        first = (first * create_comparable(&i)?)?;
    }

    Ok(Any::Primitive(first.to_datatype()))
}

pub fn div<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    if args.len() < 2 {
        return Err(NativeFnError::ArityMismatch { expected: 2, got: args.len() as _ }.into())
    }

    let mut items = args.iter().map(|i| cx.level_down().eval(i))
        .collect::<Result<Vec<Any<'a>>, InterpreterError>>()?.into_iter();

    let mut first = create_comparable(&items.next().unwrap())?;

    for i in items {
        first = (first / create_comparable(&i)?)?;
    }

    Ok(Any::Primitive(first.to_datatype()))
}
