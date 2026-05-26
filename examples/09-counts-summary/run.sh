#!/usr/bin/env sh
# Pipe input.txt through `nhs-number-cli --counts` and diff the
# observed summary against expected-summary.txt.
set -eu
here="$(cd "$(dirname "$0")" && pwd)"
bin="$here/../../target/debug/nhs-number-cli"
[ -x "$bin" ] || bin="${bin}.exe"
[ -x "$bin" ] || { echo "Binary not found at $bin — run 'cargo build' first." >&2; exit 2; }

tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT

"$bin" --counts < "$here/input.txt" > "$tmp"
diff -u "$here/expected-summary.txt" "$tmp"
echo "OK"
