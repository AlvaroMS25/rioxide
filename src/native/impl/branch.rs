use crate::interpreter::any::AnyEval;
use crate::interpreter::context::Context;
use crate::interpreter::error::InterpreterError;
use crate::macros::require_arity;
use crate::native::error::NativeFnError;
use crate::native::r#impl::logic::boolean_value;
use crate::primitives::any::Any;

pub fn r#if<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    require_arity!(exact 3, args);

    let (_, b) = boolean_value(cx, &args[0])?;

    if b {
        cx.level_down().eval(&args[1])
    } else {
        cx.level_down().eval(&args[2])
    }
}

pub fn cond<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    require_arity!(at_least 1, args);

    for arg in args.iter() {
        let AnyEval::Expression(e) = arg else {
            return Err(InterpreterError::InvalidExpression);
        };

        if e.node.is_none() || e.children.is_empty() {
            return Err(InterpreterError::InvalidExpression);
        }

        let is_else = e.node.as_ref().unwrap().is_ident()
            && *e.node.as_ref().unwrap().get_ident().unwrap() == "else";

        if is_else || boolean_value(cx, e.node.as_ref().unwrap())?.1 {

            for (idx, item) in e.children.iter().enumerate() {
                let res = cx.eval(item);

                if idx == e.children.len() - 1 {
                    return res;
                } else {
                    res?;
                }
            }
            //return e.shift_left().evaluate(&mut cx.level_down());
        }
    }

    Ok(Any::Void(()))
}
