# tolist for Raycast

### Install

If not already installed, please refer to the [installation](../README.md#from-a-release-no-rust-needed) step.

You do **not** need Rust, or anything else installed.

### Add the script to Raycast

1. Open Terminal and run:
   ```sh
   curl -fsSL https://raw.githubusercontent.com/lethib/tolist/main/raycast/install.sh | sh
   ```
   This downloads the two files the script needs into `~/.local/share/tolist/` and adds a `tolist`
   shortcut right in your home folder.
2. Open Raycast, then **Settings → Extensions → Script Commands →
   Add Script Directory**.
3. In the folder picker, go to your home folder, open **tolist**, then
   **raycast**.

*Clipboard to List* now shows up when you search in Raycast.

### Give it a shortcut

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
| Ruby | `[47658, 35367]` |
| CSV | `47658,35367` |

Values are written bare when the whole column is numeric. As soon as one value
is not a number, every value gets quoted, using the target's own escaping
rules — so a name like `O'Brien` comes out as `'O''Brien'` in SQL.

You can also assign a separate shortcut per format: duplicate the script file
under a new name, change its `@raycast.title`, and replace `"${1:-json}"` with
the format you want.

## Troubleshooting

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
