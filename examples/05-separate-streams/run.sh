#!/usr/bin/env sh
set -eu
here="$(cd "$(dirname "$0")" && pwd)"
bin="$here/../../target/debug/nhs-number-cli"
[ -x "$bin" ] || { echo "Binary not found at $bin — run 'cargo build' first." >&2; exit 2; }

valid="$here/valid.txt"
errors="$here/errors.txt"
trap 'rm -f "$valid" "$errors"' EXIT

"$bin" < "$here/input.txt" > "$valid" 2> "$errors"

diff -u "$here/expected-valid.txt"  "$valid"
diff -u "$here/expected-errors.txt" "$errors"
echo "OK"
