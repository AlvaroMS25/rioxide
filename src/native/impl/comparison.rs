use crate::{ast::expr::{Expr, Tree}, interpreter::context::Context};
use crate::interpreter::any::AnyEval;
use crate::interpreter::error::InterpreterError;
use crate::primitives::any::Any;
use crate::primitives::composed::{Composed, Pair};
use crate::primitives::DataType;

use super::super::error::NativeFnError;

pub fn equals<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    let items = args.iter().map(|i| cx.level_down().eval(i))
        .collect::<Result<Vec<Any<'a>>, InterpreterError>>()?;

    let construct_false = || Any::Primitive(DataType::Boolean(false));

    for [left, right] in items.windows(2) {

    }

    Ok(Any::Primitive(DataType::Boolean(true)))
}
