use std::{error::Error, iter::Enumerate, str::Chars};

use ast::Ast;
use primitives::DataType;

use crate::lexer::Lexer;

mod ast;
mod interpreter;
mod lexer;
mod native;
mod parser;
mod primitives;
mod cli;
mod macros;

fn main() -> Result<(), Box<dyn Error>> {
    let handle = wpool::builder::WorkerPoolBuilder::new()
        .build_owned().unwrap();

    let filename = "testfile.rkt";

    let file = std::fs::read_to_string(filename)?;

    let tokens = match Lexer::new(&file).parse() {
        Ok(t) => t,
        Err(_e) => {
            eprintln!("Failed to lex tokens");
            return Ok(()) // todo: impl error for lexererror
        }
    };

    let ast = Ast::try_from(tokens
        .into_iter()
        .map(|t| t.token)
        .collect::<Vec<_>>()
    ).unwrap();

    println!("{ast:#?}");
    
    handle.shutdown();

    Ok(())
}
