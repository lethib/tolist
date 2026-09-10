# tolist for Raycast

Turn a column of values copied from a spreadsheet into a list literal, with one
keyboard shortcut.

Copy this out of Excel or a CSV:

```
resource_id
47,658
35,367
40,368
35,605
```

Press the shortcut, and your clipboard now holds:

```
[47658,35367,40368,35605]
```

Ready to paste into a `WHERE id IN (...)`, a test fixture, or a script.

## Install

You do **not** need Rust, or anything else installed.

### 1. Install the `tolist` binary

Paste this into Terminal:

```sh
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/lethib/tolist/releases/latest/download/tolist-installer.sh | sh
```

It downloads the right binary for your Mac (Apple Silicon or Intel), puts it in
`~/.local/bin`, and adds that folder to your `PATH` if it is not there already.
The installer prints where it put things — if it says it edited a shell config,
open a new Terminal window before continuing.

Check it worked:

```sh
tolist --version
```

If that prints a version, you are done with this step.

### 2. Add the script to Raycast

1. Download this repository, or just this `raycast/` folder, somewhere
   permanent — Raycast reads the script from wherever you put it, so a folder
   you might delete later is a bad choice.
2. Open Raycast, then **Settings → Extensions → Script Commands →
   Add Script Directory**.
3. Pick the `raycast/` folder.

*Clipboard to List* now shows up when you search in Raycast.

### 3. Give it a shortcut

In the same Extensions list, find *Clipboard to List*, click the
**Record Hotkey** field on the right, and press the combination you want.

Something like `⌥⌘L` works well: it is close to `⌘C` and not taken by much.

## Use it

1. Select a column in your spreadsheet and copy it. The header row can be
   included — it is detected and dropped.
2. Press your shortcut.
3. Paste.

Nothing opens, no window appears. A small Raycast notification confirms what
was copied, truncated if the list is long.

### Choosing another format

Instead of pressing the shortcut, search for *Clipboard to List* in Raycast and
pick a format from the dropdown:

| Format | Result |
| --- | --- |
| JSON (default) | `[47658,35367]` |
| SQL | `(47658,35367)` |
| Python | `[47658, 35367]` |
| CSV | `47658,35367` |

Values are written bare when the whole column is numeric. As soon as one value
is not a number, every value gets quoted, using the target's own escaping
rules — so a name like `O'Brien` comes out as `'O''Brien'` in SQL.

You can also assign a separate shortcut per format: duplicate the script file
under a new name, change its `@raycast.title`, and replace `"${1:-json}"` with
the format you want.

## What it does to your data

- Thousands separators are cleaned: `47,658` becomes `47658`, and so do
  `1 234` and `1'234`.
- A French decimal like `1,5` is *not* touched, and stays a string. Only groups
  of exactly three digits are treated as separators.
- The header row is dropped when it is not a number and every value below it
  is. A column of text has no reliable header signal, so its first line is
  kept.
- Blank lines and surrounding whitespace go away, including the non-breaking
  spaces spreadsheets like to pad cells with.
- When several columns are copied, only the first one is kept.

## Troubleshooting

**"tolist not found"** — step 1 did not complete, or the binary is somewhere
unusual. The script looks in `~/.local/bin`, `~/.cargo/bin`,
`/opt/homebrew/bin` and `/usr/local/bin`. Run `which tolist` in Terminal; if it
prints a path outside that list, move the binary or add its folder to the list
at the top of `cols_to_list.sh`.

**"The clipboard contents were not available…"** — the clipboard holds
something that is not text, usually an image. Copy a column and retry.

**"no values found in input"** — the clipboard is empty, or held only blank
lines.

**The shortcut does nothing** — check that Raycast still points at the folder
where the script lives. If you moved or deleted it, remove the script directory
in Settings and add it again.

**Nothing was converted, the clipboard is unchanged** — the script exited with
an error that Raycast may have shown only briefly. Run it from Terminal to see
the message:

```sh
./cols_to_list.sh
```

## Not using Raycast?

The script is a five-line wrapper; the logic is all in the binary. `tolist`
reads your clipboard and writes back to it, so anything that can run a command
works the same way — Alfred, Hammerspoon, Karabiner, a shell alias, or just
typing `tolist` in a terminal.
