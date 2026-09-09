use crate::convert::format::Format;

pub(super) const PYTHON: Format = Format {
    name: "python",
    open: "[",
    close: "]",
    sep: ", ",
    quote: quote_python,
};

fn quote_python(value: &str) -> String {
    let escaped = value.replace('\\', r"\\").replace('\'', r"\'");
    format!("'{escaped}'")
}
