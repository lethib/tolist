use crate::convert::{ConvertError, Options, convert, format::Format};

/// One table-driven case. `format` is a name resolved through Format::parse,
/// so the tests never need access to the private format constants.
struct Case {
    name: &'static str,
    input: &'static str,
    format: &'static str,
    keep_header: bool,
    want: &'static str,
}

impl Case {
    fn new(name: &'static str, input: &'static str, want: &'static str) -> Self {
        Self {
            name,
            input,
            format: "json",
            keep_header: false,
            want,
        }
    }

    fn format(mut self, format: &'static str) -> Self {
        self.format = format;
        self
    }

    fn keep_header(mut self) -> Self {
        self.keep_header = true;
        self
    }
}

#[test]
fn convert_renders_a_column_as_a_list_literal() {
    let cases = vec![
        Case::new(
            "spreadsheet column with header and thousands separators",
            "resource_id\n47,658\n35,367\n40,368\n35,605",
            "[47658,35367,40368,35605]",
        ),
        Case::new("plain integers", "1\n2\n3", "[1,2,3]"),
        Case::new("CRLF line endings", "id\r\n10\r\n20\r\n", "[10,20]"),
        Case::new("lone CR line endings", "10\r20", "[10,20]"),
        Case::new("blank lines are dropped", "\n10\n\n  \n20\n\n", "[10,20]"),
        Case::new(
            "tab separated keeps first column",
            "id\tname\n10\tfoo\n20\tbar",
            "[10,20]",
        ),
        Case::new(
            "non breaking space padding is trimmed",
            "\u{a0}10\u{a0}\n20",
            "[10,20]",
        ),
        Case::new(
            "french decimal comma is not a thousands separator",
            "1,5\n2,25",
            r#"["1,5","2,25"]"#,
        ),
        Case::new(
            "space grouped number with decimal comma",
            "1\u{a0}234,5\n2\u{a0}000",
            "[1234.5,2000]",
        ),
        Case::new("swiss apostrophe grouping", "1'234\n2'000", "[1234,2000]"),
        Case::new(
            "comma grouping with decimals",
            "1,234.56\n2,000.00",
            "[1234.56,2000.00]",
        ),
        Case::new("negative numbers", "-1,500\n-20", "[-1500,-20]"),
        Case::new(
            "keep header retains the first line",
            "resource_id\n10\n20",
            r#"["resource_id","10","20"]"#,
        )
        .keep_header(),
        Case::new(
            "header is kept when values are not numbers",
            "name\nfoo\nbar",
            r#"["name","foo","bar"]"#,
        ),
        Case::new(
            "single value is never treated as a header",
            "foo",
            r#"["foo"]"#,
        ),
        Case::new(
            "mixed types quote every value",
            "10\nfoo\n20",
            r#"["10","foo","20"]"#,
        ),
        Case::new("sql numbers", "id\n10\n20", "(10,20)").format("sql"),
        Case::new(
            "sql escapes single quotes by doubling",
            "O'Brien\nfoo",
            "('O''Brien','foo')",
        )
        .format("sql"),
        Case::new("python uses a spaced separator", "10\n20", "[10, 20]").format("python"),
        Case::new(
            "python escapes quotes and backslashes",
            "a'b\nc\\d",
            r"['a\'b', 'c\\d']",
        )
        .format("python"),
        Case::new("ruby uses a spaced separator", "10\n20", "[10, 20]").format("ruby"),
        Case::new(
            "ruby escapes quotes and backslashes",
            "a'b\nc\\d",
            r"['a\'b', 'c\\d']",
        )
        .format("ruby"),
        Case::new(
            "json escapes quotes and backslashes",
            "a\"b\nc\\d",
            r#"["a\"b","c\\d"]"#,
        ),
        Case::new("csv has no wrapper", "10\n20", "10,20").format("csv"),
        Case::new(
            "csv only quotes values that need it",
            "a,b\nplain",
            r#""a,b",plain"#,
        )
        .format("csv"),
    ];

    // Every case is run before reporting, so one failure does not hide the
    // others the way an early assert would.
    let mut failures = Vec::new();
    for case in cases {
        let opts = Options {
            // An unknown name here is a bug in the table above, not a case
            // failure, so it panics instead of being collected.
            format: Format::parse(case.format)
                .unwrap_or_else(|err| panic!("{}: {err:?}", case.name)),
            keep_header: case.keep_header,
        };

        match convert(case.input.to_string(), opts) {
            Ok(got) if got == case.want => {}
            Ok(got) => failures.push(format!("{}: got {}, want {}", case.name, got, case.want)),
            Err(err) => failures.push(format!("{}: {err:?}", case.name)),
        }
    }

    assert!(
        failures.is_empty(),
        "{} case(s) failed:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

#[test]
fn convert_rejects_input_without_a_value() {
    for input in ["", "   ", "\n\n", "\t\n \n"] {
        let opts = Options {
            format: Format::parse("json").unwrap(),
            keep_header: false,
        };

        match convert(input.to_string(), opts) {
            Err(ConvertError::EmptyInput) => {}
            other => panic!("convert({input:?}) = {other:?}, want ConvertError::EmptyInput"),
        }
    }
}

#[test]
fn format_parse_resolves_every_name_and_alias() {
    let cases = [
        ("json", "json"),
        ("JSON", "json"),
        (" js ", "json"),
        ("javascript", "json"),
        ("go", "json"),
        ("sql", "sql"),
        ("python", "python"),
        ("py", "python"),
        ("csv", "csv"),
    ];

    for (input, want) in cases {
        let got =
            Format::parse(input).unwrap_or_else(|err| panic!("Format::parse({input:?}): {err:?}"));
        // The fn pointer field keeps Format from deriving PartialEq, so the
        // resolved format is identified by its name.
        assert_eq!(got.name, want, "Format::parse({input:?})");
    }
}

#[test]
fn format_parse_rejects_an_unknown_name() {
    match Format::parse("ekljee") {
        Err(ConvertError::UnknownFormat(name)) => assert_eq!(name, "ekljee"),
        other => panic!("Format::parse(\"ekljee\") = {other:?}, want UnknownFormat"),
    }
}
