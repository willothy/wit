
use clap::Command;
use crate::error::{builder::io_err, WitError};
use crate::commands;

pub trait CliExecute<'a> {
    fn execute(self) -> Result<(), Box<WitError>>;
}

impl<'a> CliExecute<'a> for Command<'a> {
    fn execute(self) -> Result<(), Box<WitError>> {
        match self.get_matches().subcommand() {
            Some(("init", args)) => commands::init(args),
            Some(("cat-file", args)) => commands::cat_file(args),
            Some(("hash-object", args)) => commands::hash_object(args),
            Some(("log", args)) => commands::log(args),
            Some(("ls-tree", args)) => commands::ls_tree(args),
            Some(("checkout", args)) => commands::checkout(args),
            Some(("show-ref", _)) => commands::show_ref(),
            Some(("tag", args)) => commands::tag(args),
            Some((invalid_cmd, _)) => {
                Err(io_err(format!("Unknown command {}", invalid_cmd)))
            }
            None => {
                Err(io_err(format!("No command specified")))
            }
        }
    }
}

pub fn setup<'a>() -> Command<'a> {
    Command::new("wit")
    .version(env!("CARGO_PKG_VERSION"))
    .author("Will Hopkins <willothyh@gmail.com>")
    .about("'Write Yourself a Git' implemented in Rust.")
    .propagate_version(true)
    .subcommand_required(true)
    .arg_required_else_help(true)
}