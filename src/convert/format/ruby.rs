use crate::convert::format::Format;

pub(super) const RUBY: Format = Format {
    name: "ruby",
    open: "[",
    close: "]",
    sep: ", ",
    quote: quote_ruby,
};

fn quote_ruby(value: &str) -> String {
    let escaped = value.replace('\\', r"\\").replace('\'', r"\'");
    format!("'{escaped}'")
}
