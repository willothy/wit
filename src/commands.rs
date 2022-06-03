use std::{env::current_dir, io::{stdout, Write}, fs, str::from_utf8, path::PathBuf};
use clap::{ArgMatches};
use crate::{
    error::{
        builder::*,
        WitError
    },
    repository::Repository,
    object::{self, WitObject}, tree::{Tree}, refs::{self, IndirectRef}
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

pub fn ls_tree(args: &ArgMatches) -> Result<(), Box<WitError>> {
    let repo = Repository::find(".", true)?.ok_or(debug_error())?;
    let obj_name = args.value_of("object").ok_or(debug_error())?;
    let obj = match object::read(&repo, &object::find(&repo, obj_name, Some("tree"), true))? {
        WitObject::TreeObject(tree) => tree,
        _ => Err(malformed_object_error(format!("Object {} is not a tree", obj_name)))?
    };

    let mut mode_str = String::new();
    for leaf in obj.leaves() {
        mode_str.clear();
        for _ in 0..(6 - leaf.mode().len()) {
            mode_str.push('0');
        }
        mode_str.push_str(leaf.mode());

        println!(
            "{} {} {}\t{}",
            mode_str,
            String::from_utf8(object::read(&repo, leaf.sha())?.serialize())?,
            leaf.sha(),
            leaf.path().to_str().unwrap()
        );
    }
    Ok(())
}


pub fn checkout(args: &ArgMatches) -> Result<(), Box<WitError>> {
    let repo = Repository::find(".", true)?.ok_or(debug_error())?;
    let obj_name = args.value_of("commit").ok_or(debug_error())?;
    let obj: Tree = match object::read(&repo, &object::find(&repo, obj_name, Some("tree"), true))? {
        WitObject::CommitObject(commit) => {
            match object::read(
                &repo,
                &commit.kvlm.get("tree").ok_or(debug_error())?[0]
            )? {
                WitObject::TreeObject(tree) => tree,
                _ => Err(malformed_object_error(format!("Could not find tree from object {}", obj_name)))?
            }
        },
        WitObject::TreeObject(tree) => tree,
        other => Err(malformed_object_error(format!("Expected a commit or tree object, got {}", from_utf8(&other.fmt()).unwrap_or("?"))))?
    };

    let path = PathBuf::from(args.value_of("path").ok_or(debug_error())?);
    if path.exists() {
        if !path.is_dir() {
            Err(debug_error())?
        }
        if !path.read_dir()?.next().is_none() {
            Err(debug_error())?
        }
    } else {
        fs::create_dir_all(args.value_of("path").ok_or(debug_error())?)?;
    }

    crate::object::checkout(&repo, &obj, &path.canonicalize()?)?;
    Ok(())
}

pub fn show_ref() -> Result<(), Box<WitError>> {
    let repo = Repository::find(".", true)?.ok_or(debug_error())?;
    let refs: IndirectRef = refs::list(&repo, None)?;
    refs::show(&repo, &refs, true, "refs")?;
    Ok(())
}