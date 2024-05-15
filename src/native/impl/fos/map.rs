use std::collections::LinkedList;

use crate::{interpreter::{any::AnyEval, context::Context, error::InterpreterError}, native::{error::NativeFnError, function::{NativeFn, NativeFunction}}, primitives::{any::Any, composed::{Composed, Function, LambdaFunction, List}}};

enum Callable<'a> {
    Lambda(LambdaFunction<'a>),
    Function(Function<'a>),
    Native(NativeFunction)
}

impl<'a> Callable<'a> {
    fn call(&self, cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
        match self {
            Self::Lambda(l) => l.body.call(cx, args),
            Self::Function(f) => f.body.call(cx, args),
            Self::Native(n) => n.call(cx, args)
        }
    }
}

fn callable_for<'a>(
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

pub fn map<'a>(
    cx: &mut Context<'_, 'a>,
    args: &[AnyEval<'a>]
) -> Result<Any<'a>, InterpreterError>
{
    if args.len() != 2 {
        return Err(NativeFnError::ArityMismatch { expected: 2, got: args.len() as _ }.into());
    }
    
    let fun = &args[0];
    let evaluated = cx.eval(&args[1])?;
    let items = evaluated.get_composed()
        .map(|i| i.get_list())
        .flatten()
        .ok_or(NativeFnError::UnexpectedType {
            function: "map",
            argument_position: 2,
            got: args[1].variant_name(),
            expected: "list"
        })?;

    let callable = callable_for(cx, fun)?;

    let mut result = LinkedList::new();

    for item in &items.0 {
        result.push_back(callable.call(cx, &[AnyEval::from_any(item.clone())])?)
    }

    Ok(Any::Composed(Box::new(Composed::List(List(result)))))
}