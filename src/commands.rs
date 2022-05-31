use std::{env::current_dir, io::{stdout, Write}, fs, str::from_utf8};
use clap::ArgMatches;
use crate::{
    error::{
        builder::*,
        WitError
    },
    repository::Repository,
    object
};

pub fn init(sub_matches: &ArgMatches) -> Result<(), Box<WitError>> {
    let pwd = match current_dir() {
        Ok(dir) => dir,
        Err(_) => Err(io_error(String::from("Could not find pwd")))?
    };
    let pwd = match pwd.to_str() {
        Some(string) => {
            String::from(string)
        },
        None => Err(io_error(String::from("Could not read pwd")))?
    };

    if let Err(e) = Repository::create(sub_matches.value_of("path").unwrap_or(pwd.as_str())) {
        println!("{}", e);
        eprintln!("Could not create repo.");
    }
    Ok(())
}

pub fn cat_file(args: &ArgMatches) -> Result<(), Box<WitError>> {
    let repo: Repository = Repository::find(".", true)?
        .ok_or(io_error(format!("No such repository {}.", args.value_of("object").unwrap_or(""))))?;
    
    let data = object::find(
        &repo,
        args.value_of("object").ok_or(io_error(format!("No object specified")))?,
        Some(args.value_of("file_type").ok_or(io_error(format!("No file type specified")))?),
        true
    );
    let obj = object::read(
        &repo,
        data.as_str()
    )?;

    let mut out = stdout();
    out.write(obj.serialize().as_slice())?;
    out.flush()?;
    Ok(())
}

pub fn hash_object(args: &ArgMatches) -> Result<(), Box<WitError>> {
    let r = Repository::new(".", false)?;
    let repo = if args.is_present("write") {
        Some(&r)
    } else {
        None
    };

    let sha = object::hash(
        from_utf8(
            &fs::read(
                args.value_of("file").ok_or(debug_error())?
            )?[..]
        )?,
        args.value_of("type").ok_or(debug_error())?,
        repo
    );
    println!("{sha:?}");
    Ok(())
}

pub fn log(args: &ArgMatches) -> Result<(), Box<WitError>> {
    let commit = args.value_of("commit").ok_or(cli_argument_error("No commit specified".to_owned()))?;
    let repo = Repository::find(".", true)?.unwrap();
    println!("digraph log {{\n");
    object::graphviz(
        &repo,
        object::find(&repo, commit, None, true),
        &mut Vec::new()
    )?;
    println!("}}");
    Ok(())
}