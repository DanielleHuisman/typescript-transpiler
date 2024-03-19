use std::{error::Error, path::PathBuf};

use clap::{
    command, error::ErrorKind, value_parser, Arg, ArgMatches, Command, CommandFactory,
    FromArgMatches, Parser,
};
use typescript_transpiler::{parse_rust_file, parse_typescript_file, transpile_file};

fn main() -> Result<(), Box<dyn Error>> {
    let command = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("parse-rs").about("Parse Rust").arg(
                Arg::new("input")
                    .required(true)
                    .value_parser(value_parser!(PathBuf)),
            ),
        )
        .subcommand(
            Command::new("parse-ts").about("Parse TypeScript").arg(
                Arg::new("input")
                    .required(true)
                    .value_parser(value_parser!(PathBuf)),
            ),
        )
        .subcommand(
            Command::new("transpile")
                .about("Transpile TypeScript to Rust")
                .arg(
                    Arg::new("input")
                        .required(true)
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(
                    Arg::new("output")
                        .required(false)
                        .value_parser(value_parser!(PathBuf)),
                ),
        );

    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("parse-rs", sub_matches)) => parse_rs(sub_matches),
        Some(("parse-ts", sub_matches)) => parse_ts(sub_matches),
        Some(("transpile", sub_matches)) => transpile(sub_matches),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct ParseArgs {
    input: PathBuf,
}

fn parse_rs(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let args = ParseArgs::from_arg_matches(matches)?;

    if !args.input.exists() {
        let mut cmd = ParseArgs::command();
        cmd.error(
            ErrorKind::ValueValidation,
            format!("Input file `{}` doesn't exist.", args.input.display()),
        )
        .exit();
    }

    let module = parse_rust_file(args.input.as_path())?;
    println!("{:#?}", module);

    Ok(())
}

fn parse_ts(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let args = ParseArgs::from_arg_matches(matches)?;

    if !args.input.exists() {
        let mut cmd = ParseArgs::command();
        cmd.error(
            ErrorKind::ValueValidation,
            format!("Input file `{}` doesn't exist.", args.input.display()),
        )
        .exit();
    }

    let module = parse_typescript_file(args.input.as_path())?;
    println!("{:#?}", module);

    Ok(())
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct TranspileArgs {
    input: PathBuf,
    output: Option<PathBuf>,
}

fn transpile(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let args = TranspileArgs::from_arg_matches(matches)?;

    if !args.input.exists() {
        let mut cmd = TranspileArgs::command();
        cmd.error(
            ErrorKind::ValueValidation,
            format!("Input file `{}` doesn't exist.", args.input.display()),
        )
        .exit();
    }

    let output = args.output.unwrap_or_else(|| {
        let mut output = args.input.clone();
        output.set_extension("rs");
        output
    });

    let output_directory = output.parent();
    if output_directory.is_none() || !output_directory.unwrap().exists() {
        let mut cmd = TranspileArgs::command();
        cmd.error(
            ErrorKind::ValueValidation,
            format!(
                "Output directory `{}` doesn't exist.",
                output_directory.unwrap().display()
            ),
        )
        .exit();
    }

    transpile_file(args.input.as_path(), output.as_path()).map_err(Box::from)
}
