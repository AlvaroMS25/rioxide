use crate::{ast::expr::{Expr, Tree}, ext::OptionTupleExt, interpreter::context::Context, primitives::ops::ComparisonOperator};
use crate::interpreter::any::AnyEval;
use crate::interpreter::error::InterpreterError;
use crate::primitives::any::Any;
use crate::primitives::composed::{Composed, Pair};
use crate::primitives::DataType;

use super::super::error::NativeFnError;

pub fn eq<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    let items = args.iter().map(|i| cx.level_down().eval(i))
        .collect::<Result<Vec<Any<'a>>, InterpreterError>>()?;

    let construct_false = || Any::Primitive(DataType::Boolean(false));
    let mut left_idx = 0;

    for item in items.windows(2) {
        assert!(item.len() == 2);
        let (left, right) = (&item[0], &item[1]);

        if !left.is_primitive() || !right.is_primitive() {
            todo!();
        }

        let Any::Primitive(left) = left else { unreachable!(); };
        let Any::Primitive(right) = right else { unreachable!(); };

        let (left, right) = (
            ComparisonOperator::from_primitive(left),
            ComparisonOperator::from_primitive(right)
        ).untuple_none().ok_or(InterpreterError::NativeError(NativeFnError::UnexpectedType {
            function: "==",
            argument_position: 1,
            got: args[0].variant_name(),
            expected: "Number"
        }))?;

        if left != right {
            return Ok(construct_false());
        }

        left_idx += 1;
    }

    Ok(Any::Primitive(DataType::Boolean(true)))
}
