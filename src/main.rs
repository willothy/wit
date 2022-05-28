use clap::{arg, command, SubCommand, Command, Arg };
use std::path::Path;

mod lib;
use lib::Repository;

/* .subcommand(
    Command::new("add")
    .about("") // TODO: Add description
    .arg(arg!([file]))
)*/

pub fn main() {
    let matches = Command::new("wit")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Will Hopkins <willothyh@gmail.com>")
        .about("'Write your self a git' implemented in Rust.")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init")
            .about("Creates a new git repository")
            .arg_required_else_help(true)
            .arg(
                arg!([path])
            )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            if let Err(e) = Repository::repo_create(sub_matches.value_of("path").unwrap()) {
                println!("{}", e);
                eprintln!("Could not create repo.");
            }
        },
        _ => {
            eprintln!("bruh");
        }
    }
}