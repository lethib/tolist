#!/bin/bash

# @raycast.schemaVersion 1
# @raycast.title Clipboard to List
# @raycast.mode silent
# @raycast.packageName Clipboard
# @raycast.icon 🔢
# @raycast.author lethib
# @raycast.authorURL https://github.com/lethib
# @raycast.argument1 { "type": "dropdown", "placeholder": "Format", "optional": true, "data": [{ "title": "JSON", "value": "json" }, { "title": "SQL", "value": "sql" }, { "title": "Python", "value": "python" }, { "title": "Ruby", "value": "ruby" }, { "title": "CSV", "value": "csv" }] }
# @raycast.description Convert the copied spreadsheet column into a list literal

exec "$(dirname "$0")/../scripts/cols_to_list.sh" "${1:-json}"
