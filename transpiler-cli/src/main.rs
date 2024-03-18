use std::{io, path::PathBuf};

use clap::{error::ErrorKind, CommandFactory, Parser};
use typescript_transpiler::transpile_file;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    input: PathBuf,
    output: PathBuf,
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();

    if !args.input.exists() {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::ValueValidation,
            format!("Input file `{}` doesn't exist.", args.input.display()),
        )
        .exit();
    }

    let output_directory = args.output.parent();
    if output_directory.is_none() || !output_directory.unwrap().exists() {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::ValueValidation,
            format!(
                "Output directory `{}` doesn't exist.",
                output_directory.unwrap().display()
            ),
        )
        .exit();
    }

    transpile_file(args.input.as_path(), args.output.as_path())
}
