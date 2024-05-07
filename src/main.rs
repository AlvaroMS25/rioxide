use std::error::Error;

use ast::Ast;
use cell::Cell;
use interpreter::vars::VarsStorage;
use primitives::DataType;

use crate::{cli::Cli, interpreter::Interpreter, lexer::Lexer};

mod ast;
mod interpreter;
mod lexer;
mod native;
mod primitives;
mod cell;
mod cli;
mod display;
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

    println!("{tokens:?}");

    let ast = Ast::try_from(tokens
        .into_iter()
        .map(|t| t.token)
        .collect::<Vec<_>>()
    ).unwrap();

    println!("{ast:#?}");

    let interpreter = Interpreter::new(ast);

    interpreter.run()?;
    
    handle.shutdown();

    Ok(())
}

fn repl() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    let mut vars = Cell::new(VarsStorage::new());

    while let Ok(_) = std::io::stdin().read_line(&mut buf) {
        let l = buf.clone();
        let tokens = Lexer::new(&l).parse()?;
        let ast = Ast::try_from(tokens
            .into_iter()
            .map(|t| t.token)
            .collect::<Vec<_>>()
        )?;

        let i = Interpreter::with_vars(ast, vars);
        i.run()?;
        vars = i.vars;
    }

    Ok(())
}
