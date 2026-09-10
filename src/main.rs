use std::{env, process::ExitCode};

use crate::{
    cli_error::CLIError,
    convert::{Options, format::Format},
};

pub mod cli_error;
pub mod clipboard;
pub mod convert;

fn main() -> ExitCode {
    match run() {
        Ok(output) => {
            eprintln!("{}", preview(&output, 50));
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<String, CLIError> {
    let options = parse_args(env::args().skip(1))?; // first arg is the binary path

    let input = clipboard::read();
    let output = convert::convert(input, options)?;

    clipboard::write(&output);
    Ok(output)
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Options, CLIError> {
    let mut options = Options::default();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--keep-header" => options.keep_header = true,
            "--format" | "-f" => {
                let targeted_format = args.next().ok_or(CLIError::MissingValue("--format"))?;
                options.format = Format::parse(&targeted_format)?
            }
            _ => return Err(CLIError::UnknownArg(arg)),
        }
    }

    Ok(options)
}

fn preview(text: &str, max: usize) -> String {
    match text.char_indices().nth(max) {
        Some((byte_index, _)) => format!("{}…", &text[..byte_index]),
        None => text.to_string(),
    }
}
