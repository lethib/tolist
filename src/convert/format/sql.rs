use crate::convert::format::Format;

pub(super) const SQL: Format = Format {
    name: "sql",
    open: "(",
    close: ")",
    sep: ",",
    quote: quote_sql,
};

fn quote_sql(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}
