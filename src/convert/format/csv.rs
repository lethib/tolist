use crate::convert::format::Format;

pub(super) const CSV: Format = Format {
    name: "csv",
    open: "",
    close: "",
    sep: ",",
    quote: quote_csv,
};

fn quote_csv(value: &str) -> String {
    // RFC 4180: only quote when the value would otherwise break the record.
    if !value.contains([',', '"', '\n', '\r']) {
        return value.to_string();
    }
    format!("\"{}\"", value.replace('"', "\"\""))
}
