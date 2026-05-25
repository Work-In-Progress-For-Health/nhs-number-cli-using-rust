#!/usr/bin/env sh
# Exercises every public CLI flag plus RUST_LOG. See README.md.
set -eu

here="$(cd "$(dirname "$0")" && pwd)"
bin="$here/../../target/debug/nhs-number-cli"
[ -x "$bin" ] || { echo "Binary not found at $bin — run 'cargo build' first." >&2; exit 2; }

tmp_out="$(mktemp)"
tmp_err="$(mktemp)"
trap 'rm -f "$tmp_out" "$tmp_err"' EXIT

fail() { echo "FAIL: $1" >&2; exit 1; }

# 1. --version prints "nhs-number-cli <X.Y.Z>" on stdout.
"$bin" --version > "$tmp_out"
grep -Eq '^nhs-number-cli [0-9]+\.[0-9]+\.[0-9]+' "$tmp_out" \
  || { cat "$tmp_out"; fail "--version output does not match 'nhs-number-cli X.Y.Z'"; }

# 2. --help mentions every public flag by name.
"$bin" --help > "$tmp_out"
for flag in --line-validation --test --verbose --version --help; do
  grep -q -- "$flag" "$tmp_out" || fail "--help omits $flag"
done

# 3. --line-validation is equivalent to the no-flag default (FR-16).
"$bin" --line-validation < "$here/input.txt" > "$tmp_out" 2> "$tmp_err"
diff -u "$here/expected-stdout.txt" "$tmp_out"
diff -u "$here/expected-stderr.txt" "$tmp_err"

# 4. --test prints the parsed Args struct. With two --verbose flags
#    the log_level field should be Some(Warn) per the count mapping.
"$bin" --test --verbose --verbose < /dev/null > "$tmp_out" 2> "$tmp_err"
grep -q '^Args {'                "$tmp_out" || fail "--test did not print 'Args {' line"
grep -q 'log_level: Some(Warn)'  "$tmp_out" || fail "--test --verbose --verbose did not show log_level: Some(Warn)"
grep -q 'check_lines:'           "$tmp_out" || fail "--test did not print check_lines field"

# 5. RUST_LOG=trace routes log macros to stderr without polluting stdout.
#    Stdout must still equal the canonical line-validation output.
RUST_LOG=trace "$bin" --line-validation < "$here/input.txt" > "$tmp_out" 2> "$tmp_err"
diff -u "$here/expected-stdout.txt" "$tmp_out"
grep -q '^Error invalid line 1\.' "$tmp_err" \
  || fail "RUST_LOG=trace lost the per-line diagnostic on stderr"
grep -q 'TRACE nhs_number_cli'    "$tmp_err" \
  || fail "RUST_LOG=trace produced no log lines on stderr"

echo "OK"
