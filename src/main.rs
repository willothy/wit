use std::error::Error;

use clap::{arg, Command };

mod repository;
mod error;
mod cli;

use error::builder::*;

pub fn main() -> Result<(), Box<dyn Error>> {
    let command = Command::new("wit")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Will Hopkins <willothyh@gmail.com>")
        .about("'Write your self a git' implemented in Rust.")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init")
            .about("Create a new git repository")
            .arg(
                arg!([path])
            )
        )
        .get_matches();

    match command.subcommand() {
        Some(("init", sub_matches)) => cli::init(sub_matches),
        _ => {
            eprintln!("Unknown command");
            return Err(Box::new(io_error(String::from("Unknown command"))))
        }
    }
}