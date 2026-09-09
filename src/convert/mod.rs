use crate::convert::{format::Format, parse::parse};

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
