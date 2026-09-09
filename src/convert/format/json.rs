use crate::convert::format::Format;

pub(super) const JSON: Format = Format {
    name: "json",
    open: "[",
    close: "]",
    sep: ",",
    quote: quote_json,
};

fn quote_json(value: &str) -> String {
    let escaped = value
        .replace('\\', r"\\")
        .replace('"', r#"\""#)
        .replace('\t', r"\t");
    format!("\"{escaped}\"")
}
