
use clap::{Command, arg};
use crate::error::{
    builder::{ cli_unknown_command_err, cli_no_command_err },
    WitError
};

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
            Some(("rev-parse", args)) => commands::rev_parse(args),
            Some((invalid_cmd, _)) => {
                Err(cli_unknown_command_err(invalid_cmd))
            }
            None => {
                Err(cli_no_command_err())
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
    .subcommands(vec![
        // init
        Command::new("init")
        .display_order(0)
        .about("Create a new git repository")
        .arg(
            arg!([path])
            .required(false)
            .help("Directory location for the new repo. If omitted, defaults to pwd.")
            .display_order(0)
        ),
        // cat-file
        Command::new("cat-file")
        .display_order(1)
        .about("Provide content of repository objects")
        .arg_required_else_help(true)
        .arg(
            arg!([file_type])
            .required(true)
            .possible_values([
                "blob",
                "commit",
                "tag",
                "tree"
            ])
            .help("Specify the type.")
            .display_order(0)
        )
        .arg(
            arg!([object])
            .required(true)
            .help("The object to display")
            .display_order(1)
        ),
        // hash-object
        Command::new("hash-object")
        .display_order(2)
        .about("Compute object ID and optionally create a blob from file.")
        .arg_required_else_help(true)
        .arg(
            arg!(-t --type <name>)
            .required(false)
            .possible_values([
                "blob",
                "commit",
                "tag",
                "tree"
            ])
            .default_value("blob")
            .help("Specify the object type")
            .display_order(0)
        )
        .arg(
            arg!(-w --write)
            .required(false)
            .help("Actually write the object into the database")
            .display_order(1)
        )
        .arg(
            arg!([file])
            .required(true)
            .help("Read the object from <file>")
            .display_order(2)
        ),
        // log
        Command::new("log")
        .display_order(3)
        .about("Display history of a given commit")
        .arg(
            arg!([commit])
            .required(false)
            .default_value("HEAD")
            .display_order(0)
            //.multiple_occurrences(true)
            .help("Commit to start at")
        ),
        // ls-tree
        Command::new("ls-tree")
        .arg_required_else_help(true)
        .display_order(4)
        .about("List the contents of a tree object")
        .arg(
            arg!([object])
            .required(true)
            .help("The tree object to list")
            .display_order(0)
        ),
        // checkout
        Command::new("checkout")
        .arg_required_else_help(true)
        .display_order(5)
        .about("Checkout a commit, a branch, or a tag")
        .arg(
            arg!([commit])
            .required(true)
            .default_value("HEAD")
            .display_order(0)
            .help("The commit or tree to checkout.")
        )
        .arg(
            arg!([path])
            .required(true)
            .default_value("master")
            .display_order(1)
            .help("The EMPTY directory to checkout on.")
        ),
        // show-ref
        Command::new("show-ref")
        .display_order(6)
        .about("List references"),
        // tag
        Command::new("tag")
        .display_order(7)
        .about("Create or list a tag")
        .arg(
            arg!(create_tag_object: -a)
            .required(false)
            .requires("name")
            .id("create_tag_object")
            .help("Create a tag object")
            .display_order(0)
        )
        .arg(
            arg!([name])
            .required(false)
            .help("Name of the tag")
            .display_order(1)
        )
        .arg(
            arg!([object])
            .help("Object the new tag will point to")
            .display_order(2)
            .default_value("HEAD")
        ),
        // rev-parse
        Command::new("rev-parse")
        .display_order(8)
        .about("Parse revision (or other objects) identifiers")
        .arg(
            arg!(-t --type <type>)
            .id("type")
            .required(false)
            .help("The object to parse")
            .display_order(0)
            .forbid_empty_values(true)
            .number_of_values(1)
            .possible_values([
                "blob",
                "commit",
                "tag",
                "tree"
            ])
        )
        .arg(
            arg!([name])
            .required(true)
            .help("The object to parse")
            .display_order(1)
        ),
    ])
}

mod commands {
    use std::{
        env::current_dir,
        io::{ stdout, Write },
        fs,
        str::from_utf8,
        path::PathBuf
    };
    use clap::ArgMatches;
    use crate::{
        error::{ builder::*, WitError },
        repository::Repository,
        object::{ self, WitObject },
        tree::Tree,
        reference::{ self, Ref::* },
        tag
    };

    pub fn init(sub_matches: &ArgMatches) -> Result<(), Box<WitError>> {
        let pwd = match current_dir() {
            Ok(dir) => dir,
            Err(_) => Err(io_err(String::from("Could not find pwd")))?
        };
        let pwd = match pwd.to_str() {
            Some(string) => {
                String::from(string)
            },
            None => Err(io_err(String::from("Could not read pwd")))?
        };

        if let Err(e) = Repository::create(sub_matches.value_of("path").unwrap_or(pwd.as_str())) {
            println!("{}", e);
            eprintln!("Could not create repo.");
        }
        Ok(())
    }

    pub fn cat_file(args: &ArgMatches) -> Result<(), Box<WitError>> {
        let repo: Repository = Repository::find(".", true)?.ok_or(pwd_not_repo_err())?;

        let data = object::find(
            &repo,
            args.value_of("object").ok_or(io_err(format!("No object specified")))?,
            Some(args.value_of("file_type").ok_or(io_err(format!("No file type specified")))?),
            true
        )?;
        let obj = object::read(
            &repo,
            data.as_str()
        )?;

        let mut out = stdout();
        out.write(obj.serialize()?.as_slice())?;
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
                    args.value_of("file").ok_or(
                        cli_argument_err("file")
                    )?
                )?[..]
            )?,
            args.value_of("type").ok_or(
                cli_argument_err("type")
            )?,
            repo
        );
        println!("{sha:?}");
        Ok(())
    }

    pub fn log(args: &ArgMatches) -> Result<(), Box<WitError>> {
        let commit = args.value_of("commit").ok_or(cli_argument_err("commit"))?;
        let repo = Repository::find(".", true)?.ok_or(pwd_not_repo_err())?;
        println!("digraph log {{\n");
        object::graphviz(
            &repo,
            object::find(&repo, commit, None, true)?,
            &mut Vec::new()
        )?;
        println!("}}");
        Ok(())
    }

    pub fn ls_tree(args: &ArgMatches) -> Result<(), Box<WitError>> {
        let repo = Repository::find(".", true)?.ok_or(pwd_not_repo_err())?;
        let obj_name = args.value_of("object").ok_or(
            cli_argument_err("object")
        )?;
        let obj = match object::read(&repo, &object::find(&repo, obj_name, Some("tree"), true)?)? {
            WitObject::TreeObject(tree) => tree,
            _ => Err(malformed_object_err(format!("Object {} is not a tree", obj_name)))?
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
                String::from_utf8(object::read(&repo, leaf.sha())?.serialize()?)?,
                leaf.sha(),
                leaf.path().to_str().unwrap()
            );
        }
        Ok(())
    }

    pub fn checkout(args: &ArgMatches) -> Result<(), Box<WitError>> {
        let repo = Repository::find(".", true)?.ok_or(pwd_not_repo_err())?;
        let obj_name = args.value_of("commit").ok_or(cli_argument_err("commit"))?;
        let obj: Tree = match object::read(&repo, &object::find(&repo, obj_name, Some("tree"), true)?)? {
            WitObject::CommitObject(commit) => {
                match object::read(
                    &repo,
                    &commit.kvlm().get("tree").ok_or(
                        missing_data_err(format!("No tree in commit {}", obj_name))
                    )?[0]
                )? {
                    WitObject::TreeObject(tree) => tree,
                    _ => Err(malformed_object_err(format!("Could not find tree from object {}", obj_name)))?
                }
            },
            WitObject::TreeObject(tree) => tree,
            other => Err(malformed_object_err(format!("Expected a commit or tree object, got {}", from_utf8(&other.fmt()).unwrap_or("?"))))?
        };

        let path = PathBuf::from(args.value_of("path").ok_or(
            cli_argument_err("path")
        )?);
        if path.exists() {
            if !path.is_dir() {
                Err(not_a_directory_err(&path))?
            }
            if !path.read_dir()?.next().is_none() {
                Err(dir_not_empty_err(&path))?
            }
        } else {
            fs::create_dir_all(&path)?;
        }

        object::checkout(&repo, &obj, &path.canonicalize()?)?;
        Ok(())
    }

    pub fn show_ref() -> Result<(), Box<WitError>> {
        let repo = Repository::find(".", true)?.ok_or(pwd_not_repo_err())?;
        let refs = reference::list(&repo, None)?;
        reference::show(&repo, &refs, true, "refs")?;
        Ok(())
    }

    pub fn tag(args: &ArgMatches) -> Result<(), Box<WitError>> {
        let repo = Repository::find(".", true)?.ok_or(pwd_not_repo_err())?;
        if args.is_present("name") {
            tag::create(
                &repo,
                args.value_of("name").ok_or(
                    cli_argument_err("name")
                )?,
                args.value_of("object").ok_or(
                    cli_argument_err("object")
                )?,
                args.is_present("create_tag_object")
            )
        } else {
            let refs = reference::list(&repo, None)?;
            let tags = match refs.get("tags").unwrap() {
                Indirect(refs) => refs,
                Direct(direct) => Err(
                    unknown_reference_err(
                        format!("Expected an indirect reference, got direct reference {}", direct)
                    )
                )?
            };
            reference::show(&repo, tags, true, "")
        }
    }

    pub fn rev_parse(args: &ArgMatches) -> Result<(), Box<WitError>> {
        let mut fmt = None;
        if args.is_present("type") {
            fmt = Some(args.value_of("type").ok_or(
                cli_argument_err("type")
            )?);
        }
        let repo = Repository::find(".", true)?.ok_or(pwd_not_repo_err())?;
        println!(
            "{}",
            object::find(&repo, args.value_of("name").ok_or(
                cli_argument_err("name")
            )?, fmt, true)?
        );
        Ok(())
    }
}