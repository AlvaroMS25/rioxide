use crate::ast::expr::Expr;
use crate::interpreter::any::AnyEval;
use crate::interpreter::context::Context;
use crate::interpreter::error::InterpreterError;
use crate::native::error::{DeclaredFunctionError, NativeFnError};
use crate::primitives::any::Any;
use crate::primitives::composed::{Composed, Function};
use crate::primitives::DataType;

pub fn define<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<Any<'a>, InterpreterError> {
    define2(cx, args).map(|i| Any::from(&i))
}

pub fn define2<'a>(cx: &mut Context<'_, 'a>, args: &[AnyEval<'a>]) -> Result<AnyEval<'a>, InterpreterError> {
    if args.len() != 2 {
        return Err(NativeFnError::ArityMismatch {expected: 2, got: args.len() as _}.into());
    }

    /*let ident = args[0].get_expression()
        .map(|i| i.get_ident().map(|s| s.to_string()))
        .flatten()
        .ok_or(InterpreterError::NativeError(NativeFnError::IdentifierExpectedIn {
            call: "define",
            got: format!("{:?}", args[0])
        }))?;*/

    let ident_error = |item| InterpreterError::NativeError(NativeFnError::IdentifierExpectedIn {
        call: "define",
        got: format!("{:?}", item)
    });


    let ident = match &args[0] {
        AnyEval::Ident(i) => i.to_string(),
        AnyEval::Expression(tree) => tree.node.as_ref().map(|n| n.get_ident())
            .flatten()
            .ok_or(InterpreterError::DeclaredFnError(DeclaredFunctionError::InvalidExpression))?
            .to_string(),
        _ => return Err(ident_error(&args[0]))
    };

    let item = match &args[0] {
        AnyEval::Ident(fn_name)
            if args[1].is_expression()
                && unsafe { args[1].get_expression_unchecked() }.node
                    .as_ref()
                    .map(|n| n.get_ident().map(|i| *i == "lambda"))
                    .flatten().unwrap_or(false)
            => {
                AnyEval::Composed(Box::new(Composed::Function(Function::from_lambda(
                    fn_name,
                    unsafe {
                        *args[1].get_expression_unchecked()
                            .clone()
                    }
                )?)))
            },
        AnyEval::Expression(_) => AnyEval::Composed(Box::new(Composed::Function(Function::parse_define(args)?))),
        _ => {
            AnyEval::from_any(cx.level_down().eval(&args[1])?)
        }
    };

    println!("Parsed {item:?}");

    cx
        .vars_mut()
        .insert(&ident, Any::from(&item));

    Ok(AnyEval::Void(()))
}