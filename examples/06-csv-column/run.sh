#!/usr/bin/env sh
set -eu
here="$(cd "$(dirname "$0")" && pwd)"
bin="$here/../../target/debug/nhs-number-cli"
[ -x "$bin" ] || { echo "Binary not found at $bin — run 'cargo build' first." >&2; exit 2; }

tmp_out="$(mktemp)"; tmp_err="$(mktemp)"
trap 'rm -f "$tmp_out" "$tmp_err"' EXIT

cut -d, -f3 < "$here/patients.csv" | tail -n +2 | "$bin" > "$tmp_out" 2> "$tmp_err"
diff -u "$here/expected-stdout.txt" "$tmp_out"
diff -u "$here/expected-stderr.txt" "$tmp_err"
echo "OK"
