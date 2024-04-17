use std::{error::Error, iter::Enumerate, str::Chars};

use crate::lexer::Lexer;

mod lexer;
mod primitives;
mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let handle = wpool::builder::WorkerPoolBuilder::new()
        .build_owned().unwrap();

    let filename = "juan";

    let file = std::fs::read_to_string(filename)?;

    let iter = file.lines().enumerate();

    let _tokens = Lexer::new(iter);
    
    handle.shutdown();

    Ok(())
}
