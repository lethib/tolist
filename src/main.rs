use std::{env, process::ExitCode};

use crate::{
    cli_error::CLIError,
    convert::{Options, format::Format},
};

pub mod cli_error;
pub mod clipboard;
pub mod convert;

const USAGE: &str = "\
tolist turns a copied spreadsheet column into a list literal.

Usage:
  tolist [flags]

Reads the clipboard, converts it, and writes the result back to the clipboard.

Flags:
  -f, --format <name>  target format: json, sql, python, csv (default: json)
      --keep-header    treat the first line as a value, never as a column name
  -h, --help           print this help and exit
  -V, --version        print the version and exit

Format aliases:
  json    js, javascript, go
  python  py

Examples:
  tolist                # clipboard to clipboard, as JSON
  tolist -f sql         # (47658,35367)
  tolist --keep-header  # keep the first line as a value
";

enum Command {
    Run(Options),
    Help,
    Version,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), CLIError> {
    match parse_args(env::args().skip(1))? {
        Command::Help => print!("{USAGE}"),
        Command::Version => println!("tolist {}", env!("CARGO_PKG_VERSION")),
        Command::Run(options) => {
            let input = clipboard::read();
            let output = convert::convert(input, options)?;

            clipboard::write(&output);
            eprintln!("{}", preview(&output, 50));
        }
    }

    Ok(())
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Command, CLIError> {
    let mut options = Options::default();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => return Ok(Command::Help),
            "--version" | "-V" => return Ok(Command::Version),
            "--keep-header" => options.keep_header = true,
            "--format" | "-f" => {
                let targeted_format = args.next().ok_or(CLIError::MissingValue("--format"))?;
                options.format = Format::parse(&targeted_format)?
            }
            _ => return Err(CLIError::UnknownArg(arg)),
        }
    }

    Ok(Command::Run(options))
}

fn preview(text: &str, max: usize) -> String {
    match text.char_indices().nth(max) {
        Some((byte_index, _)) => format!("{}…", &text[..byte_index]),
        None => text.to_string(),
    }
}
