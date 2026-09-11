#!/bin/bash
set -euo pipefail

DEST="${TOLIST_INSTALL_DIR:-$HOME/.local/share/tolist}"
LINK="$HOME/tolist"
RAW="https://raw.githubusercontent.com/lethib/tolist/main"

mkdir -p "$DEST/raycast" "$DEST/scripts"
curl -fsSL "$RAW/raycast/cols_to_list.sh" -o "$DEST/raycast/cols_to_list.sh"
curl -fsSL "$RAW/scripts/cols_to_list.sh" -o "$DEST/scripts/cols_to_list.sh"
chmod +x "$DEST/raycast/cols_to_list.sh" "$DEST/scripts/cols_to_list.sh"

ln -sfn "$DEST" "$LINK"

echo "Installed to: $LINK/raycast"
echo
echo "Next steps:"
echo "1. Open Raycast > Settings > Extensions > Script Commands > Add Script Directory."
echo "2. In the folder picker, go to your home folder, open \"tolist\", then \"raycast\"."
echo "3. Search Raycast for \"Clipboard to List\" and assign it a hotkey."
