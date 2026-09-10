#!/bin/bash

# Raycast script command wrapper around tolist.
# The conversion logic lives in the tolist binary; this file only exists to
# expose it to a Raycast hotkey.
#
# Setup:
#   1. Install tolist, either from a release (no Rust needed):
#        curl --proto '=https' --tlsv1.2 -LsSf \
#          https://github.com/lethib/tolist/releases/latest/download/tolist-installer.sh | sh
#      or from a clone of this repo: just install
#   2. Raycast > Extensions > Script Commands > Add Script Directory
#      and point it at this raycast/ folder
#   3. Assign a hotkey to "Clipboard to List"

# @raycast.schemaVersion 1
# @raycast.title Clipboard to List
# @raycast.mode silent
# @raycast.packageName Clipboard
# @raycast.icon 🔢
# @raycast.author lethib
# @raycast.authorURL https://github.com/lethib
# @raycast.argument1 { "type": "dropdown", "placeholder": "Format", "optional": true, "data": [{ "title": "JSON", "value": "json" }, { "title": "SQL", "value": "sql" }, { "title": "Python", "value": "python" }, { "title": "Ruby", "value": "ruby" }. { "title": "CSV", "value": "csv" }] }
# @raycast.description Convert the copied spreadsheet column into a list literal

for candidate in \
	"$HOME/.local/bin/tolist" \
	"$HOME/.cargo/bin/tolist" \
	/opt/homebrew/bin/tolist \
	/usr/local/bin/tolist; do
	if [ -x "$candidate" ]; then
		TOLIST="$candidate"
		break
	fi
done
TOLIST="${TOLIST:-$(command -v tolist)}"

if [ -z "$TOLIST" ]; then
	echo "tolist not found; install it from https://github.com/lethib/tolist/releases"
	exit 1
fi

output=$("$TOLIST" --format "${1:-json}" 2>&1)
status=$?

if [ $status -ne 0 ]; then
	echo "$output"
	exit $status
fi

echo "Copied: $output"
