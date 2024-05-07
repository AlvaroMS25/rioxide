use crate::{ast::expr::Expr, interpreter::{context::Context, error::InterpreterError}};

use super::any::Any;

impl<'a> Expr<'a> {
    pub fn prepare(mut self, cx: &Context<'_, 'a>) -> Result<Any<'a>, InterpreterError> {
        todo!()
    }
}