use crate::convert::{Field, Options, regex::*};

pub(super) fn parse(mut input: String, opts: &Options) -> Vec<Field> {
    // Spreadsheets paste CRLF, and a lone CR still shows up from older Excel versions.
    input = input.replace("\r\n", "\n");
    input = input.replace("\r", "\n");

    let mut fields: Vec<Field> = input
        .split('\n')
        .map(|line| match line.split_once('\t') {
            Some((left, _)) => left,
            None => line,
        })
        .map(trim)
        .filter(|cell| !cell.is_empty())
        .map(classify)
        .collect();

    if !opts.keep_header {
        fields = drop_header(fields);
    }

    fields
}

fn trim(cell: &str) -> &str {
    cell.trim_matches(|c: char| c.is_whitespace() || c == '\u{feff}')
}

fn classify(cell: &str) -> Field {
    if PLAIN_NUMBER.is_match(cell) {
        return Field {
            val: cell.to_string(),
            is_number: true,
        };
    }

    if let Some(captures) = GROUPED_COMMA.captures(cell) {
        let integer = captures[1].replace(",", ""); // safe
        let decimal = captures.get(2).map_or("", |m| m.as_str());
        return Field {
            val: format!("{integer}{decimal}"),
            is_number: true,
        };
    }

    if let Some(captures) = GROUPED_SPACE.captures(cell) {
        let mut val = captures[1].to_string(); // safe
        val.retain(|c| !SPACE_SEPARATORS.contains(&c));
        if let Some(decimal) = captures.get(2) {
            // Decimal comma is not valid in any target format.
            val.push('.');
            val.push_str(decimal.as_str());
        }
        return Field {
            val,
            is_number: true,
        };
    }

    if let Some(captures) = GROUPED_QUOTE.captures(cell) {
        let integer = captures[1].replace("'", ""); // safe
        let decimal = captures.get(2).map_or("", |m| m.as_str());
        return Field {
            val: format!("{integer}{decimal}"),
            is_number: true,
        };
    }

    Field {
        val: cell.to_string(),
        is_number: false,
    }
}

fn drop_header(fields: Vec<Field>) -> Vec<Field> {
    if fields.len() == 0 {
        return fields;
    }

    if fields.len() < 2 || fields[0].is_number {
        return fields;
    }

    for f in fields[1..].iter() {
        if !f.is_number {
            return fields;
        }
    }

    fields.into_iter().skip(1).collect()
}
