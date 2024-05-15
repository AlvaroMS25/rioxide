use std::{error::Error, io::Write};

use ast::Ast;
use cell::Cell;
use clap::Parser;
use interpreter::vars::{OwnedStorage, VarsStorage};
use primitives::DataType;

use crate::{cli::{Cli, SubCommands}, interpreter::Interpreter, lexer::Lexer};

mod ast;
mod interpreter;
mod lexer;
mod native;
mod primitives;
mod cell;
mod cli;
mod container;
mod display;
mod macros;
mod ext;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(SubCommands::Run { filename }) => run(&filename),
        Some(SubCommands::Repl) | None => repl()
    }
}

fn run(file: &str) -> Result<(), Box<dyn Error>> {
    let file = std::fs::read_to_string(file)?;
    let tokens = Lexer::new(&file).parse()?;
    let ast = Ast::try_from(tokens
        .into_iter()
        .map(|t| t.token)
        .collect::<Vec<_>>()
    )?;
    Interpreter::new(ast).run().map_err(From::from)
}

fn repl() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    let mut vars = Cell::new(OwnedStorage::new());
    let mut stdout = std::io::stdout();

    print!("> ");
    stdout.flush()?;

    while let Ok(_) = std::io::stdin().read_line(&mut buf) {
        let tokens = match Lexer::new(&buf).parse() {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Failed to parse tokens: {e}");
                buf.clear();
                continue;
            }
        };
        println!("{:?}", tokens);

        let ast_res = Ast::try_from(tokens
            .into_iter()
            .map(|t| t.token)
            .collect::<Vec<_>>()
        );

        println!("{:?}", ast_res);
        buf.clear();

        /*let ast = match ast_res {
            Ok(a) => a,
            Err(e) => {
                eprintln!("Error parsing abstract syntax tree, error: {e}");
                buf.clear();
                print!("> ");
                continue;
            }
        };

        let i = Interpreter::with_vars(ast, vars);
        if let Err(e) = i.run() {
            eprintln!("Runtime error: {e}");
        }
        vars = i.vars;
        println!("");
        print!("> ");
        stdout.flush()?;
        buf.clear();*/
    }

    Ok(())
}
