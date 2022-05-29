use clap::{ Command, arg };

mod repository;
mod object;
mod error;
mod cli;

use self::{error::WitError, cli::execution::CliExecute};

pub fn main() -> Result<(), Box<WitError>> {
    cli::execution::setup()
        .subcommand(
            Command::new("init")
            .about("Create a new git repository")
            .arg(
                arg!([path])
            )
        )
        .execute()
}