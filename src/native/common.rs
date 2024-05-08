use crate::interpreter::context::Context;
use crate::interpreter::error::InterpreterError;
use crate::native::error::NativeFnError;
use crate::primitives::any::Any;

pub fn define<'a>(cx: &mut Context<'_, 'a>, args: &[Any<'a>]) -> Result<Any<'a>, InterpreterError> {
    if args.len() != 2 {
        return Err(NativeFnError::ArityMismatch {expected: 2, got: args.len() as _}.into());
    }

    let ident = args[0].get_expression()
        .map(|i| i.get_ident().map(|s| s.to_string()))
        .flatten()
        .ok_or(InterpreterError::NativeError(NativeFnError::IdentifierExpectedIn {
            call: "define",
            got: format!("{:?}", args[0])
        }))?;

    let item = match &args[1] {
        Any::Expression(e) => cx.eval(e)?,
        other => other.clone()
    };

    cx
        .vars_mut()
        .insert(&ident, item);

    Ok(Any::Void(()))
}

pub fn ast<'a>(cx: &mut Context<'_, 'a>, _: &[Any<'a>]) -> Result<Any<'a>, InterpreterError> {
    println!("{:?}", cx.interpreter().ast());

    Ok(Any::Void(()))
}

pub fn clear_terminal<'a>(cx: &mut Context, _: &[Any<'a>]) -> Result<Any<'a>, InterpreterError> {
    print!("\x1B[2J\x1B[1;1H");
    Ok(Any::Void(()))
}

pub fn exit<'a>(_: &mut Context, _: &[Any<'a>]) -> Result<Any<'a>, InterpreterError> {
    std::process::exit(0);
}
