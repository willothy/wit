use std::process::ExitCode;

use clap::{ Command, arg };

mod repository;
mod object;
mod blob;
mod commit;
mod tree;
mod refs;
mod tag;
mod kvlm;
mod commands;
mod error;
mod cli;

use cli::CliExecute;

pub fn main() -> ExitCode {
    let app = cli::setup()
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
                arg!(-a)
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
            )
        ]);

    match app.execute() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            println!("{}", e);
            ExitCode::FAILURE
        }
    }
}