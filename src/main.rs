pub mod cli;
pub mod editor;
pub mod note;
pub mod search;
pub mod traversal;

use std::io;

use clap::Parser;
use noteorg::cli::list_files;

use crate::cli::{edit_file, generate_completions, show_search_results_realtime};

fn main() -> io::Result<()> {
    let args = cli::Args::parse();
    match args.cmd {
        cli::Commands::List { path } => list_files(path),
        cli::Commands::Edit { search_value } => edit_file(search_value),
        cli::Commands::Search => show_search_results_realtime(),
        cli::Commands::Completions { shell } => {
            generate_completions(shell);
            Ok(())
        }
        _ => {
            println!("Not implemented!");
            Ok(())
        }
    }
}
