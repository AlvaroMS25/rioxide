use std::error::Error;

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

    let _tokens = Lexer::new(file.lines()).parse_tokens();
    
    handle.shutdown();

    Ok(())
}
