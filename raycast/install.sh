#!/bin/bash
set -euo pipefail

DEST="${TOLIST_INSTALL_DIR:-$HOME/.local/share/tolist}"
RAW="https://raw.githubusercontent.com/lethib/tolist/main"

mkdir -p "$DEST/raycast" "$DEST/scripts"
curl -fsSL "$RAW/raycast/cols_to_list.sh" -o "$DEST/raycast/cols_to_list.sh"
curl -fsSL "$RAW/scripts/cols_to_list.sh" -o "$DEST/scripts/cols_to_list.sh"
chmod +x "$DEST/raycast/cols_to_list.sh" "$DEST/scripts/cols_to_list.sh"

echo "Installed to: $DEST/raycast"
echo
echo "Next steps:"
echo "1. Open Raycast > Settings > Extensions > Script Commands > Add Script Directory."
echo "2. Pick: $DEST/raycast"
echo "3. Search Raycast for \"Clipboard to List\" and assign it a hotkey."
