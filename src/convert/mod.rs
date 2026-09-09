use std::{error::Error, fmt::Display};

use crate::convert::{
    format::{Format, json::JSON},
    parse::parse,
};

pub mod format;
pub mod parse;
pub mod regex;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum ConvertError {
    EmptyInput,
    UnknownFormat(String),
}

impl Error for ConvertError {}

impl Display for ConvertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "no values found in input"),
            Self::UnknownFormat(name) => write!(f, "unknown format {name:?}"),
        }
    }
}

#[derive(Debug)]
pub struct Options {
    pub format: Format,
    pub keep_header: bool,
}

pub struct Field {
    val: String,
    is_number: bool,
}

pub fn convert(input: String, opts: Options) -> Result<String, ConvertError> {
    let fields = parse(input, &opts);
    if fields.is_empty() {
        return Err(ConvertError::EmptyInput);
    }

    let all_numbers = fields.iter().all(|f| f.is_number);

    Ok(opts.format.render(fields, all_numbers))
}

impl Default for Options {
    fn default() -> Self {
        Self {
            format: JSON,
            keep_header: false,
        }
    }
}
