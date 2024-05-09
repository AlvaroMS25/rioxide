use crate::interpreter::context::Context;
use crate::interpreter::error::InterpreterError;
use crate::native::error::NativeFnError;
use crate::primitives::any::Any;
use crate::primitives::DataType;

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

    /*let item = match &args[1] {
        Any::Expression(e) => cx.eval(e)?,
        other => other.clone()
    };*/

    cx
        .vars_mut()
        .insert(&ident, args[1].clone());

    Ok(Any::Void(()))
}
