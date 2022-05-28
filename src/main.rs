use clap::{arg, command, SubCommand, Command };
use std::path::Path;

mod lib;

pub fn main() {
    let matches = Command::new("wit")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Will Hopkins <willothyh@gmail.com>")
        .about("'Write your self a git' implemented in Rust.")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("") // TODO: Add description
                .arg(arg!([file]))
        ).get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => println!(
            "file: {}",
            sub_matches.value_of("file").unwrap()
        ),
        _ => {}
    }
}