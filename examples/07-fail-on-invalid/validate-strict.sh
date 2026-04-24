#!/usr/bin/env sh
# Read NHS Numbers on stdin and exit non-zero if any line is invalid.
#
# stdout passes through unchanged (one line per valid NHS Number).
# stderr passes through unchanged (one diagnostic per invalid line).
# Exit status is 0 when stderr is empty, 1 otherwise.
set -eu

here="$(cd "$(dirname "$0")" && pwd)"
bin="$here/../../target/debug/nhs-number-cli"
[ -x "$bin" ] || { echo "Binary not found at $bin — run 'cargo build' first." >&2; exit 2; }

err_file="$(mktemp)"
trap 'rm -f "$err_file"' EXIT

# Capture stderr so we can inspect it, but also print it to the real stderr.
"$bin" 2> "$err_file" || true
if [ -s "$err_file" ]; then
  cat "$err_file" >&2
  exit 1
fi
