use std::collections::{HashMap, LinkedList};
use std::fmt::{self, Write};
use clap::arg;
use crate::ast::expr::Expr;
use crate::display::InterpreterDisplay;
use crate::interpreter::context::Context;
use crate::interpreter::error::InterpreterError;
use crate::interpreter::vars::VarsStorage;
use crate::interpreter::Interpreter;
use crate::macros::get_enum;
use crate::native::error::DeclaredFunctionError;
use crate::primitives::any::Any;

#[derive(Clone, Debug)]
pub struct List<'a>(pub LinkedList<Any<'a>>);

#[derive(Clone, Debug)]
pub struct FunctionBody<'a>(pub Expr<'a>);

#[derive(Clone, Debug)]
pub struct Function<'a> {
    pub name: &'a str,
    pub body: FunctionBody<'a>,
    pub arity: Option<u8>
}

#[derive(Clone, Debug)]
pub struct LambdaFunction<'a> {
    pub arity: Option<u8>,
    pub body: FunctionBody<'a>
}

#[derive(Clone, Debug)]
pub struct Symbol<'a>(pub &'a str);

#[derive(Clone, Debug)]
pub struct Pair<'a> {
    pub left: Any<'a>,
    pub right: Any<'a>
}

get_enum! {
    /// Data types composed by more of a single item
    #[derive(Clone, Debug)]
    pub enum Composed<'a> {
        List(List<'a>),
        Function(Function<'a>),
        Lambda(LambdaFunction<'a>),
        Symbol(Symbol<'a>),
        Pair(Pair<'a>)
    }
}

impl<'a> Function<'a> {        
    pub fn evaluate(&self, cx: &mut Context<'_, 'a>) -> Result<Any<'a>, InterpreterError> {
        let Some(body) = self.body.0.get_parenthesized() else { return Ok(Any::from(&self.body.0)) };

        if body.children.len() < 1 {
            return Err(InterpreterError::DeclaredFnError(DeclaredFunctionError::InvalidExpression));
        }

        let mut parameters = HashMap::with_capacity(body.children.len() + (body.node.is_some() as usize));
        let _ = &mut parameters;

        self.body.evaluate(cx, parameters)
    }
}

impl<'a> FunctionBody<'a> {
    pub fn evaluate(
        &self, 
        cx: &mut Context<'_, 'a>, 
        vars: HashMap<&'a str, Option<&'a Any<'a>>>
    ) -> Result<Any<'a>, InterpreterError> {
        todo!()
    }
}

impl InterpreterDisplay for Composed<'_> {
    fn fmt(&self, f: &mut dyn Write, interpreter: &Interpreter<'_>) -> fmt::Result {
        match self {
            Self::List(l) => l.fmt(f, interpreter),
            Self::Pair(p) => p.fmt(f, interpreter),
            _ => Ok(())
        }
    }
}
