# Usage

`nhs-number-cli` is a filter in the Unix tradition. It reads lines from
standard input and writes results to standard output and standard error.

## Synopsis

```
nhs-number-cli [FLAGS] < INPUT > STDOUT 2> STDERR
```

All payload data is read from `stdin`; flags only select *which*
subcommand runs and how chatty the binary is. With no flags the binary
runs the line-validation subcommand (see [Line validation](../line-validation/index.md)),
which is the default and which most callers want.

## Flags

| Flag                       | Effect                                                                |
| -------------------------- | --------------------------------------------------------------------- |
| `-l`, `--line-validation`  | Run the line-validation subcommand explicitly. Same effect as no flag. Mutually exclusive with `--counts`. |
| `-c`, `--counts`           | Run the counts subcommand: read stdin, emit a four-row summary (`valid`, `invalid`, `parse-error`, `blank`) on stdout, write nothing to stderr. Mutually exclusive with `--line-validation`. See [`examples/09-counts-summary/`](../../examples/09-counts-summary/). |
| `--column N`               | Split each non-blank input line on `,` and use the N-th (1-based) field as the candidate NHS Number. Honoured by both `--line-validation` and `--counts`. Headers are not skipped automatically; pipe through `tail -n +2`. See [`examples/06-csv-column/`](../../examples/06-csv-column/). |
| `--test`                   | Print the parsed `Args` struct and resolved log level on `stdout` before running. Diagnostic only — do not pipe `--test` output into another tool. |
| `-v…`, `--verbose…`        | Increase log verbosity. Count → level: 1=error, 2=warn, 3=info, 4=debug, 5=trace. Logs go to `stderr` via `env_logger`. |
| `-V`, `--version`          | Print the crate version and exit.                                     |
| `-h`, `--help`             | Print help and exit.                                                  |

Flag names, short forms, and the count semantics of `--verbose` are
public API. See [`spec.md`](../../spec.md) FR-13.

## Subcommand dispatch

The binary supports two subcommands today: `check_lines` (the
default line-validation filter) and `counts` (`--counts`, summary
mode). They are mutually exclusive at the clap layer. Future
subcommands will each be opt-in via their own flag; the no-flag
default behaviour will not change. See
[Architecture § subcommands](../architecture/index.md) and
[`spec.md`](../../spec.md) FR-16 / FR-17.

## Input

* One NHS Number per line.
* Whitespace between digits is tolerated. Both `9999999999` and
  `999 999 9999` parse successfully.
* Blank lines are skipped.
* Lines with other content (letters, punctuation other than spaces,
  fewer or more than ten digits) are reported as parse errors.

## Output

* `stdout` — one line per **valid** NHS Number, in the canonical
  `NNN NNN NNNN` form.
* `stderr` — one line per input line that could not be validated, either
  because it failed to parse or because its check digit was wrong.

## Stream ordering

Stdout and stderr are separate streams and may be reordered by the
terminal or by downstream pipelines. Within each stream, output is in
input order. If you need a combined, ordered log, redirect both streams
into the same sink:

```sh
nhs-number-cli < input.txt > output.log 2>&1
```

## Exit status

The exit status is always **0**. Invalid input does not cause a non-zero
exit. If you need to fail a build or script when invalid numbers are
present, inspect the `stderr` output:

```sh
if nhs-number-cli < input.txt 2>&1 >/dev/null | grep -q '^Error '; then
  echo "Invalid NHS Numbers detected." >&2
  exit 1
fi
```

## Error message format

Errors on `stderr` have one of two forms:

```
Error invalid line <n>. Error: validate check digit failed. NHS Number: <nhs-number>
Error parsing line <n>. Error: <debug>. Line: <original-line>
```

Where `<n>` is the **zero-based** line index from the input. The format
is considered stable; scripts may grep for `^Error ` to detect problems.

## Reading from a file

`nhs-number-cli` does not take a filename argument. Redirect or pipe
the file:

```sh
cat input.txt | nhs-number-cli
nhs-number-cli < input.txt
```

On Windows:

```sh
type input.txt | nhs-number-cli.exe
```

## Separating valid from invalid output

Because the streams are already separated, you can redirect each to its
own file:

```sh
nhs-number-cli < input.txt > valid.txt 2> errors.txt
```

## Counting valid and invalid

```sh
cat input.txt | nhs-number-cli > valid.txt 2> errors.txt
echo "valid:   $(wc -l < valid.txt)"
echo "invalid: $(wc -l < errors.txt)"
```

## Integration with other tools

### Strip whitespace first

If your input file has trailing whitespace or CR line endings (common on
Windows), normalize before piping:

```sh
tr -d '\r' < windows-file.txt | nhs-number-cli
```

### Deduplicate before validating

```sh
sort -u input.txt | nhs-number-cli
```

### Extract NHS Numbers from a CSV

If the NHS Number is in column 3 of a comma-separated file:

```sh
cut -d, -f3 < patients.csv | tail -n +2 | nhs-number-cli
```

### Pipe directly from a query result

```sh
psql -At -c "SELECT nhs_number FROM patients" | nhs-number-cli
```

See [Examples](../examples/index.md) and the top-level
[`examples/`](../../examples/) directory for full working recipes.

## Configuration

On start-up the binary reads a TOML configuration file via the
[`confy`](https://crates.io/crates/confy) crate from the OS-appropriate
location:

| OS         | Path                                                       |
| ---------- | ---------------------------------------------------------- |
| Linux      | `~/.config/nhs-number-cli/nhs-number-cli.toml`             |
| macOS      | `~/Library/Application Support/rs.nhs-number-cli/nhs-number-cli.toml` |
| Windows    | `%APPDATA%\nhs-number-cli\config\nhs-number-cli.toml`      |

A missing file is not an error; the default config is used. A malformed
file produces a startup error on `stderr` and exit code `1`.

At the current version the only config field is `version` (schema
versioning). Future fields will be additive only. See
[`spec.md`](../../spec.md) FR-14.

## Logging and environment variables

Operational logs are routed by [`env_logger`](https://crates.io/crates/env_logger)
to `stderr`. The default level is off — no log lines appear unless you
opt in.

Two opt-in mechanisms:

```sh
# Repeat -v to raise the level: -v=error, -vv=warn, -vvv=info, -vvvv=debug, -vvvvv=trace
nhs-number-cli -vvv < input.txt

# Or use env_logger's standard RUST_LOG syntax.
RUST_LOG=debug nhs-number-cli < input.txt
RUST_LOG=nhs_number_cli=trace nhs-number-cli < input.txt
```

Log lines are written to `stderr` only. They never appear on `stdout`
and are distinguishable from per-line diagnostics because diagnostics
begin with `Error ` (see [Error message format](#error-message-format))
while log lines do not. See [`spec.md`](../../spec.md) FR-15.

No other environment variables affect the binary's behaviour.
