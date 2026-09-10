# tolist

Turn a column of values copied from a spreadsheet or CSV file into a list
literal, without hand-editing it.

Copy this out of Excel:

```
resource_id
47,658
35,367
40,368
35,605
```

Run `tolist`, and the clipboard holds:

```
[47658,35367,40368,35605]
```

Ready to paste into a `WHERE id IN (...)`, a test fixture, or a script.

## Install

### From a release (no Rust needed)

```sh
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/lethib/tolist/releases/latest/download/tolist-installer.sh | sh
```

Downloads the right binary for your Mac — Apple Silicon or Intel — installs it
into `~/.local/bin`, and adds that folder to your `PATH` if it is not there
already. Open a new terminal afterwards if the installer says it edited a shell
config.

### From source

Needs Rust 1.85 or newer (the crate is on edition 2024).

```sh
git clone https://github.com/lethib/tolist
cd tolist
just install    # cargo install --path . --force, into ~/.cargo/bin
```

## Usage

```
tolist [flags]
```

| Flag | Meaning |
| --- | --- |
| `-f`, `--format <name>` | target format: `json`, `sql`, `python`, `csv` (default `json`) |
| `--keep-header` | treat the first line as a value, never as a column name |
| `-h`, `--help` | print help and exit |
| `-V`, `--version` | print the version and exit |

### Using with shortcuts

The point of the tool is to run on a keyboard shortcut. Some tools can properly run scripts using keyboard shortcuts.

#### Raycast

`raycast/` holds a thin script-command wrapper and its own [README](raycast/README.md) with the setup
steps.

#### more to come...

### Formats

For the column `47,658` / `35,367`:

| Format | Output |
| --- | --- |
| `json` | `[47658,35367]` |
| `sql` | `(47658,35367)` |
| `python` | `[47658, 35367]` |
| `csv` | `47658,35367` |

Values are written bare when every value in the column is a number. As soon as
one is not, all of them are quoted, so the list stays homogeneous and valid —
a mixed `(10,'foo')` cannot match a single typed SQL column, while
`('10','foo')` can.

Quoting follows each target's own rules: SQL doubles the apostrophe
(`'O''Brien'`), JSON and Python use a backslash, and CSV quotes a value only
when it would otherwise break the record.

## What it does to the input

- **Line endings.** Handles CRLF and lone-CR, as pasted by Excel.
- **Whitespace.** Blank lines are dropped and cells are trimmed, including the
  non-breaking spaces spreadsheet exports pad them with.
- **Columns.** When several columns are copied, only the first is kept.
- **Header.** The first line is dropped when it is not a number while every
  value below it is. A column of text has no reliable header signal, so its
  first line is kept. `--keep-header` disables the detection entirely.
- **Thousands separators.** `47,658` becomes `47658`. So do `1 234` (plain,
  non-breaking and narrow non-breaking spaces) and the Swiss `1'234`. A
  space-grouped number with a decimal comma is normalised too: `1 234,5`
  becomes `1234.5`.

Cleaning is deliberately conservative: it only fires on digit groups of exactly
three, so a French decimal like `1,5` is left alone as a string rather than
silently becoming `15`.

Numbers are never parsed into a numeric type — separators are stripped from the
text and nothing else. So `2,000.00` comes out as `2000.00`, not `2000`, and
large ids keep every digit.


## Development

```sh
just            # list the recipes
just test       # cargo test
just check      # fmt --check, clippy -D warnings, test
just build      # release build
just dist       # release binaries for both mac architectures, with checksums
```

## License

MIT.
