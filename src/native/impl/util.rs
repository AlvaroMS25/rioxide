use crate::{interpreter::{any::AnyEval, context::Context, error::InterpreterError}, native::{error::NativeFnError, function::NativeFunction}, primitives::{any::Any, composed::{Function, LambdaFunction}}};

pub enum Callable<'a> {
    Lambda(LambdaFunction<'a>),
    Function(Function<'a>),
    Native(NativeFunction)
}

impl<'a> Callable<'a> {
    pub fn call(&self, cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
        match self {
            Self::Lambda(l) => l.body.call(cx, args),
            Self::Function(f) => f.body.call(cx, args),
            Self::Native(n) => n.call(cx, args)
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
    fun: &AnyEval<'a>
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
            function: "map",
            argument_position: 1,
            got: fun.variant_name(),
            expected: "list of function pointer"
        }.into())
    }
}