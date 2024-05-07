use crate::{ast::expr::Expr, interpreter::{context::Context, error::InterpreterError}};

pub struct FunctionBody<'a>(pub Expr<'a>);

impl<'a> FunctionBody<'a> {
    /// Replaces expression items with variables from the current context, if this is later stored,
    /// subsequent calls to the function can get significantly faster.
    pub fn prepare(mut self, cx: &Context<'_, 'a>) -> Result<Self, InterpreterError> {
        todo!()
    }
}
