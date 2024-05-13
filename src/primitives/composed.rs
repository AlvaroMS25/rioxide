use std::collections::{HashMap, LinkedList};
use std::fmt::{self, Write};
use clap::arg;
use crate::ast::expr::{Expr, Tree};
use crate::display::InterpreterDisplay;
use crate::ext::StrExt;
use crate::interpreter::any::AnyEval;
use crate::interpreter::eval_tree::EvalTree;
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
pub struct FunctionBody<'a> {
    pub args: Vec<&'a str>,
    pub body: Vec<AnyEval<'a>>,
}

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

impl<'a> FunctionBody<'a> {
    pub fn evaluate(
        &self, 
        cx: &mut Context<'_, 'a>, 
        vars: &HashMap<&'a str, Option<&'a Any<'a>>>
    ) -> Result<Any<'a>, InterpreterError> {
        //let Some(paren) = self.body.get_parenthesized() else { return Ok(Any::from(&self.0)); };

        let mut iter = self.body.iter().enumerate().skip(1); // first child is the argument list;

        while let Some((idx, expr)) = iter.next() {
            if idx == self.body.len() - 1 {
                return Self::eval_expr(cx, expr, vars);
            } else {
                Self::eval_expr(cx, expr, vars)?;
            }
        }

        todo!()
    }

    fn eval_expr(
        cx: &mut Context<'_, 'a>,
        expr: &AnyEval<'a>,
        vars: &HashMap<&'a str, Option<&'a Any<'a>>>
    ) -> Result<Any<'a>, InterpreterError>
    {
        let Some(tree) = expr.get_expression() else {
            return Ok(Any::from(expr));
        };

        cx.eval_tree(tree)
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

impl Composed<'_> {
    pub fn make_static(self) -> Composed<'static> {
        use Composed::*;
        match self {
            List(l) => List(l.make_static()),
            Function(f) => Function(f.make_static()),
            Lambda(l) => Lambda(l.make_static()),
            Symbol(s) => Symbol(s.make_static()),
            Pair(p) => Pair(p.make_static()),
        }
    }
}

impl List<'_> {
    pub fn make_static(self) -> List<'static> {
        let mut new_list = LinkedList::new();

        for item in self.0 {
            new_list.push_back(item.make_static());
        }

        List(new_list)
    }
}

impl<'a> FunctionBody<'a> {
    pub fn make_static(self) -> FunctionBody<'static> {
        FunctionBody {
            args: self.args.into_iter().map(StrExt::make_static).collect(),
            body: self.body.into_iter().map(|e| e.make_static()).collect()
        }
    }

    fn parse(args: &Vec<AnyEval<'a>>, body: Vec<AnyEval<'a>>) -> Result<FunctionBody<'a>, InterpreterError> {
        let mut body_args = Vec::with_capacity(args.len());

        for arg in args.iter() {
            println!("Arg {arg:?}");
            body_args.push(*arg.get_ident().ok_or(DeclaredFunctionError::InvalidExpression)?);
        }

        Ok(Self {
            args: body_args,
            body
        })
    }
}

impl<'a> Function<'a> {
    pub fn make_static(self) -> Function<'static> {
        Function {
            name: self.name.make_static(),
            body: self.body.make_static(),
            arity: self.arity
        }
    }

    pub fn parse_from(left: &EvalTree<'a>, right: Vec<AnyEval<'a>>) -> Result<Function<'a>, InterpreterError> {
        let arity = left.children.len();
        let name = left.node.as_ref().unwrap().get_ident().unwrap();
        let body = FunctionBody::parse(&left.children, right)?;
        
        Ok(Function {
            name,
            arity: Some(arity as u8),
            body
        })
    }

    pub fn parse_define(args: &[AnyEval<'a>]) -> Result<Function<'a>, InterpreterError> {
        // we know is parenthesized, so just use the unchecked methods
        let first = unsafe { args[0].get_expression_unchecked() }.clone();

        Self::parse_from(&first, args.iter().skip(1).map(Clone::clone).collect::<Vec<_>>())
    }

    pub fn from_lambda(name: &'a str, tree: EvalTree<'a>) -> Result<Function<'a>, InterpreterError> {
        println!("Lamba tree: {tree:?}");
        if tree.children.len() < 2 {
            return Err(InterpreterError::DeclaredFnError(DeclaredFunctionError::InvalidExpression));
        }

        let arity = tree.children.len() + 1;

        assert_eq!(*tree.node.as_ref().unwrap().get_ident().unwrap(), "lambda");

         let args = (&tree.children[1..]).iter().map(Clone::clone).collect::<Vec<_>>();

        Ok(Function {
            name,
            arity: Some(arity as _),
            body: FunctionBody::parse(
                &tree.children[0].get_expression()
                    .ok_or(InterpreterError::DeclaredFnError(DeclaredFunctionError::InvalidExpression))?
                    .ident_vec(),
                args)?
        })
    }
}

impl LambdaFunction<'_> {
    pub fn make_static(self) -> LambdaFunction<'static> {
        LambdaFunction {
            arity: self.arity,
            body: self.body.make_static(),
        }
    }
}

impl Symbol<'_> {
    pub fn make_static(self) -> Symbol<'static> {
        Symbol(self.0.make_static())
    }
}

impl Pair<'_> {
    pub fn make_static(self) -> Pair<'static> {
        Pair {
            left: self.left.make_static(),
            right: self.right.make_static()
        }
    }
}
