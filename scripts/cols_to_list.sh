#!/bin/bash

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
