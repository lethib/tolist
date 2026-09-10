use std::fmt::Display;

use crate::convert::ConvertError;

#[derive(Debug)]
pub(super) enum CLIError {
    MissingValue(&'static str),
    UnknownArg(String),
    Convert(ConvertError),
    Clipboard(arboard::Error),
}

impl Display for CLIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingValue(command) => write!(f, "missing value for command {command}"),
            Self::UnknownArg(arg) => write!(f, "unknown arg {arg}"),
            Self::Convert(err) => write!(f, "{err}"),
            Self::Clipboard(err) => write!(f, "{err}"),
        }
    }
}

impl From<ConvertError> for CLIError {
    fn from(value: ConvertError) -> Self {
        Self::Convert(value)
    }
}

impl From<arboard::Error> for CLIError {
    fn from(value: arboard::Error) -> Self {
        Self::Clipboard(value)
    }
}
