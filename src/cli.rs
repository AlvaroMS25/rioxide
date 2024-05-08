use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version = "0.1")]
#[command(author = "Alvaro")]
#[command(about = "A simple racket interpreter")]
#[command(long_about = "A simple racket interpreter made to learn to write interpreters")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<SubCommands>
}

#[derive(Subcommand)]
pub enum SubCommands {
    Repl,
    Run {
        filename: String
    }
}
