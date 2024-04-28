pub mod context;
pub mod vars;
pub mod any;

use crate::ast::Ast;

pub struct Interpreter<'a> {
    ast: Ast<'a>
}

impl<'a> Interpreter<'a> {
    pub fn new(ast: Ast<'a>) -> Self {
        Self {
            ast
        }
    }
}
