use crate::ast::expr::Expr;
use crate::interpreter::context::Context;
use crate::interpreter::error::InterpreterError;
use crate::native::error::{DeclaredFunctionError, NativeFnError};
use crate::primitives::any::Any;
use crate::primitives::composed::{Composed, Function};
use crate::primitives::DataType;

pub fn define<'a>(cx: &mut Context<'_, 'a>, args: &[Any<'a>]) -> Result<Any<'a>, InterpreterError> {
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

    let ident = match args[0].get_expression().ok_or_else(|| ident_error(&args[0]))? {
        Expr::Ident(i) => i.to_string(),
        Expr::Parenthesized(tree) => tree.node.as_ref().map(|n| n.get_ident()).flatten()
            .ok_or(InterpreterError::DeclaredFnError(DeclaredFunctionError::InvalidExpression))?
            .to_string(),
        _ => return Err(ident_error(&args[0]))
    };

    let item = match &args[0] {
        Any::Expression(Expr::Parenthesized(_))
            // Expressions like (define (a x) (+ x x)) are parenthesized on second argument,
            // just take the "a" part from the first tree and pass it to FunctionBody parser
            => Any::Composed(Box::new(Composed::Function(Function::parse_define(args)?))),
        Any::Expression(Expr::Ident(fn_name)) if args[1].is_expression()
            => Any::Composed(Box::new(Composed::Function(Function::from_lambda(
            fn_name,
            unsafe {
                args[1].get_expression_unchecked().get_parenthesized()
                    .ok_or(InterpreterError::DeclaredFnError(DeclaredFunctionError::InvalidExpression))?
                    .clone()
            }
        )?))),
        other => other.clone()
    };

    cx
        .vars_mut()
        .insert(&ident, item);

    Ok(Any::Void(()))
}
