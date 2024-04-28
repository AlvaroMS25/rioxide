use std::marker::PhantomData;
use crate::{ast::expr::{Expr, Tree}, native::NativeStorage};

use super::vars::VarsStorage;

pub struct Context<'a> {
    pub storage: NativeStorage<'a>,
    pub vars: VarsStorage<'a>,
    marker: PhantomData<&'a ()>
}

impl<'a> Context<'a> {
    pub fn eval_tree(&self, tree: &Tree<'a>) -> Expr<'a> {
        todo!()
    }
}
