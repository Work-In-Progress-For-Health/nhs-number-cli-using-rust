# 08-crlf-windows

Two related input issues you'll hit with files that come from other
systems:

1. **CRLF line endings** (`\r\n`) from Windows. The Rust standard
   library's `BufRead::lines()` strips both `\r` and `\n`, so a
   straight CRLF file pipes through cleanly. Nothing special is
   required.
2. **Trailing whitespace** on a line (spaces, tabs). These are *not*
   stripped, and the parser will reject a number such as
   `"999 000 0000 "` (trailing space). Normalise with `sed` or `awk`
   before piping.

This example exercises both cases.

## Files

* `input-crlf.txt` — CRLF-terminated valid numbers. The tool handles
  them as-is.
* `input-trailing.txt` — each line has a trailing space. The tool
  reports parse errors.
* `expected-stdout-crlf.txt` / `expected-stderr-crlf.txt` — outputs for
  the CRLF case.
* `expected-stdout-normalised.txt` / `expected-stderr-normalised.txt` —
  outputs after `sed 's/[[:space:]]*$//'` normalises the trailing
  whitespace from `input-trailing.txt`.

## Run

```sh
./run.sh
```

## Patterns

Strip trailing whitespace before piping:

```sh
sed 's/[[:space:]]*$//' < input-trailing.txt | nhs-number-cli
```

Strip all carriage returns (belt-and-braces, if the installed Rust is
older than 1.55):

```sh
tr -d '\r' < input-crlf.txt | nhs-number-cli
```
