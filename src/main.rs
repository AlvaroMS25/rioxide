use std::{error::Error, iter::Enumerate, str::Chars};

use crate::lexer::Lexer;

mod lexer;
mod primitives;
mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    let handle = wpool::builder::WorkerPoolBuilder::new()
        .build_owned().unwrap();

    let filename = "testfile.rkt";

    let file = std::fs::read_to_string(filename)?;

    let iter = file.lines().enumerate();

    let tokens = Lexer::new(iter).parse();

    println!("{:?}", tokens);
    
    handle.shutdown();

    Ok(())
}
