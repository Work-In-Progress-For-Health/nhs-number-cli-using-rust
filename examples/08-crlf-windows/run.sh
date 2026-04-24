#!/usr/bin/env sh
set -eu
here="$(cd "$(dirname "$0")" && pwd)"
bin="$here/../../target/debug/nhs-number-cli"
[ -x "$bin" ] || { echo "Binary not found at $bin — run 'cargo build' first." >&2; exit 2; }

tmp_out="$(mktemp)"; tmp_err="$(mktemp)"
trap 'rm -f "$tmp_out" "$tmp_err"' EXIT

# 1. CRLF input passes through unchanged (stdout has canonical form).
"$bin" < "$here/input-crlf.txt" > "$tmp_out" 2> "$tmp_err"
diff -u "$here/expected-stdout-crlf.txt" "$tmp_out"
diff -u "$here/expected-stderr-crlf.txt" "$tmp_err"

# 2. Trailing whitespace after normalisation passes cleanly.
sed 's/[[:space:]]*$//' < "$here/input-trailing.txt" | "$bin" > "$tmp_out" 2> "$tmp_err"
diff -u "$here/expected-stdout-normalised.txt" "$tmp_out"
diff -u "$here/expected-stderr-normalised.txt" "$tmp_err"

echo "OK"
