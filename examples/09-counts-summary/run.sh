#!/usr/bin/env sh
set -eu
here="$(cd "$(dirname "$0")" && pwd)"
chmod +x "$here/summary.sh"

tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT

"$here/summary.sh" < "$here/input.txt" > "$tmp"
diff -u "$here/expected-summary.txt" "$tmp"
echo "OK"
