use crate::{interpreter::{any::AnyEval, context::Context, error::InterpreterError}, native::{error::NativeFnError, function::NativeFunction}, primitives::{any::Any, composed::{Function, LambdaFunction}}};

pub enum Callable<'a> {
    Lambda(LambdaFunction<'a>),
    Function(Function<'a>),
    Native(NativeFunction)
}

impl<'a> Callable<'a> {
    pub fn call(&self, cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
        match self {
            Self::Lambda(l) => l.body.call(&mut cx.level_down(), args),
            Self::Function(f) => f.body.call(&mut cx.level_down(), args),
            Self::Native(n) => n.call(&mut cx.level_down(), args)
        }
    }

    pub fn arity(&self) -> Option<u8> {
        match self {
            Self::Function(f) => f.arity,
            Self::Lambda(l) => l.arity,
            _ => None
        }
    }
}

pub fn callable_for<'a>(
    cx: &mut Context<'_, 'a>,
    fun: &AnyEval<'a>,
    fn_name: &'static str,
    arg_pos: u8
) -> Result<Callable<'a>, InterpreterError>
where
{
    match fun {
        AnyEval::Ident(i) => {
            if cx.interpreter().is_native(i) {
                Ok(Callable::Native(*cx.interpreter().native_vars()
                    .get(i).unwrap()))
            } else if cx.is_declared_function(i) {
                Ok(Callable::Function(unsafe {
                    cx.interpreter().vars().get(i)
                        .unwrap()
                        .get_composed_unchecked()
                        .get_function_unchecked()
                        .clone()
                }))
            } else {
                return Err(InterpreterError::UndefinedFunction(i.to_string()));
            }
        },
        AnyEval::Composed(c) if c.is_function() => {
            Ok(Callable::Function(unsafe { c.get_function_unchecked().clone() }))
        },
        AnyEval::Expression(e) => {
            Ok(Callable::Lambda(e.clone().try_parse_lambda()?))
        }
        _ => return Err(NativeFnError::UnexpectedType {
            function: fn_name,
            argument_position: arg_pos,
            got: fun.variant_name(),
            expected: "lambda or function"
        }.into())
    }
}

pub fn non_negative_int<'a>(
    cx: &mut Context<'_, 'a>,
    item: &AnyEval<'a>,
    fun_name: &'static str,
    position: u8
) -> Result<usize, InterpreterError> {
    Ok(*cx.eval(item)?
        .get_primitive()
        .map(|p| p.get_integer().map(|i| if *i >= 0 {
            Some(i)
        } else {
            None
        }))
        .flatten()
        .flatten()
        .ok_or(NativeFnError::UnexpectedType {
            function: fun_name,
            argument_position: position,
            got: item.variant_name(),
            expected: "non negative integer"
        })? as usize)
}
