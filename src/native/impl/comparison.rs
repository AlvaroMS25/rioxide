use crate::{ast::expr::{Expr, Tree}, ext::OptionTupleExt, interpreter::context::Context, primitives::ops::ComparisonOperator};
use crate::interpreter::any::AnyEval;
use crate::interpreter::error::InterpreterError;
use crate::primitives::any::Any;
use crate::primitives::composed::{Composed, Pair};
use crate::primitives::DataType;

use super::super::error::NativeFnError;

pub fn with_comparable_window2<'a, F>(
    items: &[Any<'a>],
    predicate: F,
) -> Result<Any<'a>, NativeFnError>
where
    F: Fn(ComparisonOperator, ComparisonOperator) -> bool,
{
    let construct_false = || Any::Primitive(DataType::Boolean(false));

    for window in items.windows(2) {
        assert!(window.len() == 2);
        let (left, right) = (&window[0], &window[1]);

        if !left.is_primitive() || !right.is_primitive() {
            return Err(NativeFnError::InvalidType(format!("{} and {}", left.variant_name(), right.variant_name())));
        }

        let Any::Primitive(left) = left else { 
            return Err(NativeFnError::InvalidType(left.variant_name().to_string()));
        };
        let Any::Primitive(right) = right else { 
            return Err(NativeFnError::InvalidType(right.variant_name().to_string()));
        };

        let (left, right) = (
            ComparisonOperator::from_primitive(left),
            ComparisonOperator::from_primitive(right)
        ).untuple_none()
        .ok_or(NativeFnError::InvalidType(format!("{} and {}", left.variant_name(), right.variant_name())))?;

        if !predicate(left, right) {
            return Ok(construct_false());
        }
    }

    Ok(Any::Primitive(DataType::Boolean(true)))
}

pub fn eq<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    if args.len() < 2 {
        return Err(NativeFnError::ArityMismatch { expected: 2, got: args.len() as _ }.into())
    }

    let items = args.iter().map(|i| cx.level_down().eval(i))
        .collect::<Result<Vec<Any<'a>>, InterpreterError>>()?;

    Ok(with_comparable_window2(
        items.as_slice(),
        |left, right| left == right
    )?)
}

pub fn neq<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    if args.len() < 2 {
        return Err(NativeFnError::ArityMismatch { expected: 2, got: args.len() as _ }.into())
    }

    let Any::Primitive(DataType::Boolean(equals)) = eq(cx, args)? else { unreachable!() };
    Ok(Any::Primitive(DataType::Boolean(!equals)))
}

pub fn gt<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    if args.len() < 2 {
        return Err(NativeFnError::ArityMismatch { expected: 2, got: args.len() as _ }.into())
    }

    let items = args.iter().map(|i| cx.level_down().eval(i))
        .collect::<Result<Vec<Any<'a>>, InterpreterError>>()?;

    Ok(with_comparable_window2(
        items.as_slice(),
        |left, right| left > right
    )?)
}

pub fn ge<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    if args.len() < 2 {
        return Err(NativeFnError::ArityMismatch { expected: 2, got: args.len() as _ }.into())
    }

    let items = args.iter().map(|i| cx.level_down().eval(i))
        .collect::<Result<Vec<Any<'a>>, InterpreterError>>()?;

    Ok(with_comparable_window2(
        items.as_slice(),
        |left, right| left >= right
    )?)
}

pub fn lt<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    if args.len() < 2 {
        return Err(NativeFnError::ArityMismatch { expected: 2, got: args.len() as _ }.into())
    }

    let items = args.iter().map(|i| cx.level_down().eval(i))
        .collect::<Result<Vec<Any<'a>>, InterpreterError>>()?;

    Ok(with_comparable_window2(
        items.as_slice(),
        |left, right| left < right
    )?)
}

pub fn le<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    if args.len() < 2 {
        return Err(NativeFnError::ArityMismatch { expected: 2, got: args.len() as _ }.into())
    }

    let items = args.iter().map(|i| cx.level_down().eval(i))
        .collect::<Result<Vec<Any<'a>>, InterpreterError>>()?;

    Ok(with_comparable_window2(
        items.as_slice(),
        |left, right| left <= right
    )?)
}
