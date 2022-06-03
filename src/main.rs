use std::process::ExitCode;
use cli::CliExecute;

mod repository;
mod object;
mod blob;
mod commit;
mod tree;
mod reference;
mod tag;
mod index;
mod kvlm;
mod error;
mod util;
mod cli;

pub fn main() -> ExitCode {
    let app = cli::setup();
    match app.execute() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            println!("{}", e);
            ExitCode::FAILURE
        }
    }
}