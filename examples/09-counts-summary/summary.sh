#!/usr/bin/env sh
# Print a summary of valid, invalid, and blank lines on stdin.
set -eu
here="$(cd "$(dirname "$0")" && pwd)"
bin="$here/../../target/debug/nhs-number-cli"
[ -x "$bin" ] || { echo "Binary not found at $bin — run 'cargo build' first." >&2; exit 2; }

input="$(mktemp)"
out="$(mktemp)"
err="$(mktemp)"
trap 'rm -f "$input" "$out" "$err"' EXIT

cat > "$input"

"$bin" < "$input" > "$out" 2> "$err"

valid=$(wc -l < "$out" | tr -d ' ')
invalid=$(wc -l < "$err" | tr -d ' ')
blank=$(awk '/^[[:space:]]*$/ { n++ } END { print n+0 }' "$input")

printf 'valid:   %s\ninvalid: %s\nblank:   %s\n' "$valid" "$invalid" "$blank"
