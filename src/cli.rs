
pub mod execution {
    use clap::{Command, arg};
    use crate::error::{builder::io_error, WitError};

    pub trait CliExecute<'a> {
        fn execute(&self) -> Result<(), Box<WitError>>;
    }

    impl<'a> CliExecute<'a> for Command<'a> {
        fn execute(&self) -> Result<(), Box<WitError>> {
            match self.get_matches().subcommand() {
                Some(("init", args)) => super::commands::init(args),
                _ => {
                    eprintln!("Unknown command");
                    return Err(io_error(String::from("Unknown command")))
                }
            }
        }
    }

    pub fn setup<'a>() -> Command<'a> {
        Command::new("wit")
            .version(env!("CARGO_PKG_VERSION"))
            .author("Will Hopkins <willothyh@gmail.com>")
            .about("'Write your self a git' implemented in Rust.")
            .propagate_version(true)
            .subcommand_required(true)
            .arg_required_else_help(true)
    }
}

pub mod commands {
    use std::{env::current_dir};
    use clap::ArgMatches;
    use crate::{error::{builder::*, WitError}, repository::Repository};

    pub fn init(sub_matches: &ArgMatches) -> Result<(), Box<WitError>> {
        let pwd = match current_dir() {
            Ok(dir) => dir,
            Err(_) => return Err(io_error(String::from("Could not find pwd")))
        };
        let pwd = match pwd.to_str() {
            Some(string) => {
                String::from(string)
            },
            None => return Err(io_error(String::from("Could not read pwd")))
        };

        if let Err(e) = Repository::repo_create(sub_matches.value_of("path").unwrap_or(pwd.as_str())) {
            println!("{}", e);
            eprintln!("Could not create repo.");
        }
        Ok(())
    }
}