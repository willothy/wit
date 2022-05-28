use std::{env::current_dir, error::Error};

use clap::ArgMatches;

use crate::{error::builder::*, repository::Repository};

pub fn init(sub_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let pwd = match current_dir() {
        Ok(dir) => dir,
        Err(_) => return Err(Box::new(io_error(String::from("Could not find pwd"))))
    };
    let pwd = match pwd.to_str() {
        Some(string) => {
            String::from(string)
        },
        None => return Err(Box::new(io_error(String::from("Could not read pwd"))))
    };

    if let Err(e) = Repository::repo_create(sub_matches.value_of("path").unwrap_or(pwd.as_str())) {
        println!("{}", e);
        eprintln!("Could not create repo.");
    }
    Ok(())
}