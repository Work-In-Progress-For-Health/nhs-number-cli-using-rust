#!/usr/bin/env sh
# Run this example and diff against the expected output files.
set -eu

here="$(cd "$(dirname "$0")" && pwd)"
bin="$here/../../target/debug/nhs-number-cli"

if [ ! -x "$bin" ]; then
  echo "Binary not found at $bin — run 'cargo build' first." >&2
  exit 2
fi

tmp_out="$(mktemp)"
tmp_err="$(mktemp)"
trap 'rm -f "$tmp_out" "$tmp_err"' EXIT

"$bin" < "$here/input.txt" > "$tmp_out" 2> "$tmp_err"

diff -u "$here/expected-stdout.txt" "$tmp_out"
diff -u "$here/expected-stderr.txt" "$tmp_err"

echo "OK"
