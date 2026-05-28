# Troubleshooting

## "validate check digit failed"

```
Error invalid line 2. Error: validate check digit failed. NHS Number: 999 123 4561
```

The input parsed as ten digits but the tenth digit is not the correct
Modulus 11 check digit for the first nine digits. Possible causes:

* A typo in the input. Re-enter the number.
* The wrong column is being piped in. Double-check your `cut`/`awk`
  command.
* The source system stores a non-NHS identifier that happens to be ten
  digits long.

Recalculate the check digit by hand with the algorithm in
[About NHS Numbers](../about-nhs-numbers/index.md#check-digit) or use
another tool to cross-check.

## "Error parsing line"

```
Error parsing line 3. Error: ParseError. Line: not-an-nhs-number
```

The input does not contain ten digits (ignoring whitespace). Common
causes:

* Extra columns (e.g. passing a whole CSV row instead of a single field
  — pick a field with `--column N`).
* Line endings included (rare, but possible with exotic encodings).
* A header row passed by mistake. Use `tail -n +2` to skip it.
* A comment or empty marker character such as `-` or `N/A`.

## "Error parsing line ... Error: ColumnMissing(N) ..."

```
Error parsing line 5. Error: ColumnMissing(3). Line: 6,short-row
```

The `--column N` flag asked for the *N*-th comma-separated field, but
the input row had fewer than *N* fields. Either the row is genuinely
short (a malformed CSV) or you picked the wrong column index. Note
that with `--column`, the `Line:` payload contains the *whole* input
row, not just the selected field — useful for tracing the row back to
its source.

## CRLF line endings

On Windows, files often have `\r\n` line endings. Rust's
`BufRead::lines()` strips both `\r` and `\n`, so a CRLF file generally
pipes through cleanly. If you see unexpected parse errors against a
Windows-authored file, strip the carriage returns explicitly and try
again:

```sh
tr -d '\r' < input.txt | nhs-number-cli
```

A more common Windows issue is **trailing whitespace** (a stray space
or tab at the end of a line). That is not stripped by the reader, so
the parser rejects the line. Normalise first:

```sh
sed 's/[[:space:]]*$//' < input.txt | nhs-number-cli
```

## No output at all

* Did you forget to pipe anything in? The tool reads from `stdin`; if
  `stdin` is empty, the tool exits immediately with no output.
* Did you redirect stderr somewhere you forgot? Try running without
  redirection first to see all diagnostics.

## The binary runs but exits with status 0 despite invalid numbers

This is by design. See
[Usage § exit status](../usage/index.md#exit-status). If you need a
non-zero exit on error, wrap the call in a script that inspects the
`stderr` stream — `examples/07-fail-on-invalid/validate-strict.sh`
is one such wrapper.

## `cargo build` fails with "edition = 2024" error

You are on an older toolchain. Upgrade:

```sh
rustup update stable
```

Edition 2024 requires Rust 1.85 or newer.

## Reporting a bug

Open an issue at
<https://github.com/joelparkerhenderson/nhs-number-cli/issues>
and include:

* The command you ran.
* A small, **synthetic** input file that reproduces the problem. Do not
  include real NHS Numbers.
* The complete `stdout` and `stderr` output.
* Output of `rustc --version` and `cargo --version`.
