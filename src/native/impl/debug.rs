use crate::{interpreter::{context::Context, error::InterpreterError}, primitives::{any::Any, DataType}};
use crate::interpreter::any::AnyEval;

use super::super::error::NativeFnError;

pub fn ast<'a>(cx: &mut Context<'_, 'a>, _: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    println!("{:?}", cx.interpreter().ast());

    Ok(Any::Void(()))
}

pub fn clear_terminal<'a>(cx: &mut Context, _: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    print!("\x1B[2J\x1B[1;1H");
    Ok(Any::Void(()))
}

pub fn exit<'a>(_: &mut Context, _: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    std::process::exit(0);
}

pub fn ast_with<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    if args.len() < 2 {
        return Err(InterpreterError::NativeError(NativeFnError::ArityMismatch { expected: 2, got: args.len() as _ }));
    }

    let Some(pretty) = args[0].get_primitive().map(|v| v.get_boolean()).flatten() else {
        return Err(InterpreterError::NativeError(NativeFnError::UnexpectedType {
            function: "ast-with",
            argument_position: 1,
            got: args[0].variant_name(),
            expected: DataType::Boolean(false).variant_name()
        }))
    };

    if *pretty {
        println!("{:#?}", cx.interpreter().ast());
    } else {
        println!("{:?}", cx.interpreter().ast());
    }

    cx.eval(&args[1])
}

pub fn show_memory<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    if args.len() < 1 {
        return Err(InterpreterError::NativeError(NativeFnError::ArityMismatch { expected: 1, got: args.len() as _ }));
    }

    let Some(pretty) = args[0].get_primitive().map(|v| v.get_boolean()).flatten() else {
        return Err(InterpreterError::NativeError(NativeFnError::UnexpectedType {
            function: "show-memory",
            argument_position: 1,
            got: args[0].variant_name(),
            expected: DataType::Boolean(false).variant_name()
        }))
    };

    if *pretty {
        println!("Global: {:#?}", cx.global_vars());
        println!("Local: {:#?}", cx.local_vars());
    } else {
        println!("Global: {:?}", cx.global_vars());
        println!("Local: {:?}", cx.local_vars());
    }

    Ok(Any::Void(()))
}
