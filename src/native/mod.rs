pub mod error;
pub mod function;
mod r#impl;
use r#impl::*;

use error::NativeFnError;

use std::collections::HashMap;

use crate::{ast::expr::{Expr, Tree}, interpreter::{context::Context, Interpreter}};
use crate::primitives::any::Any;
use crate::macros::map_native_hashmap;
use crate::native::function::NativeFunction;

pub struct NativeStorage {
    table: HashMap<&'static str, NativeFunction>
}

impl NativeStorage {
    pub fn new() -> Self {
        Self {
            table: map_native_hashmap! {
                "cons" => compose::cons,
                "define" => common::define,
                "list" => compose::list,
                "d/ast" => debug::ast,
                "d/clear" => debug::clear_terminal,
                "exit" => debug::exit,
                "d/ast-with" => debug::ast_with,
                "d/show-memory" => debug::show_memory,
                "=" => comparison::eq,
                ">" => comparison::gt,
                "<" => comparison::lt,
                ">=" => comparison::ge,
                "<=" => comparison::le,
                "+" => math::add,
                "-" => math::sub,
                "*" => math::mul,
                "/" => math::div,
                "map" => fos::map,
                "foldr" => fos::foldr,
                "foldl" => fos::foldl,
                "filter" => fos::filter,
                "string?" => string::is_string,
                "string-append" => string::string_append,
                "make-string" => string::make_string,
                "string-length" => string::len,
                "string-ref" => string::string_ref,
                "substring" => string::substring,
                "string->list" => string::string_to_list,
                "list->string" => string::list_to_string
            }
        }
    }

    pub fn get(&self, item: &str) -> Option<&NativeFunction> {
        self.table.get(item)
    }
}
