# Behavioural contract

These are the observable behaviours that downstream tools, scripts, and
patient-data pipelines depend on. **Breaking any of them is a major
version bump** and must be flagged in the PR description, the release
notes, and [`spec.md`](../spec.md).

The corresponding numbered specifications live in [`spec.md`](../spec.md).
This file is the agent-facing summary.

## Stream contract

1. **Input source:** standard input only. The binary must not open files
   from arguments, environment variables, or configuration. Callers
   redirect (`< file`, `cat file |`) themselves.
2. **Line discipline:** input is processed one line at a time, in order,
   until EOF. Each line is independent; there is no cross-line state.
3. **Blank lines:** silently skipped. They produce nothing on either stream.
4. **Valid number → stdout:** a line that parses as an NHS Number and has
   a valid Modulus 11 check digit is printed to `stdout` in the canonical
   `NNN NNN NNNN` form produced by `nhs_number::NHSNumber`'s `Display`.
   Exactly one stdout line per valid input line.
5. **Failed check digit → stderr:** a line that parses but whose tenth
   digit does not match the Modulus 11 computation is reported on
   `stderr` and does not appear on `stdout`.
6. **Parse failure → stderr:** a line that fails to parse (wrong length,
   non-digit characters, etc.) is reported on `stderr` and does not appear
   on `stdout`.
7. **Read error → stderr:** an underlying I/O error on a line is reported
   on `stderr`. Processing continues with the next line.

## Error message format

Diagnostics on `stderr` follow a stable schema. Scripts grep `^Error `
to count or detect them.

Check-digit failure:

```
Error invalid line <n>. Error: validate check digit failed. NHS Number: <nhs-number>
```

Parse failure:

```
Error parsing line <n>. Error: <debug>. Line: <original-line>
```

I/O failure (rare):

```
Error reading line <n>. Error: <debug>.
```

The `thiserror`-rendered form may also include arrow-delimited fields
(`Error <Kind> ➡ field: value, …`); both forms are considered stable and
both are accepted by the test fixtures. Do not remove either.

`<n>` is the **zero-based** line index from `Iterator::enumerate`.
Changing it to 1-based is a breaking change.

## Exit code

* `0` — the program reached EOF on stdin and processed every line.
  Per-line failures do **not** change the exit code.
* `1` — the program could not start (configuration load failed, the OS
  refused stdin, an unrecoverable internal error). The reason is on
  `stderr`.

Callers that need to fail a pipeline on any invalid line should check for
output on `stderr`, not the exit code. See `examples/07-fail-on-invalid/`.

## Command-line interface

The CLI is built with `clap`. The currently-defined flags:

| Flag                       | Effect                                                                |
| -------------------------- | --------------------------------------------------------------------- |
| `-l`, `--line-validation`  | Run the line-validation subcommand (the default behaviour).           |
| `--test`                   | Print the parsed `Args` and log level to `stdout` (diagnostic).       |
| `-v…` / `--verbose…`       | Increase log verbosity: count maps to error/warn/info/debug/trace.    |
| `-V`, `--version`          | Print the crate version (handled by clap).                            |
| `-h`, `--help`             | Print help (handled by clap).                                         |

Flag names, short forms, and the count-based `--verbose` mapping are
public API. Adding a flag is non-breaking; removing or renaming one is
breaking. The `RUST_LOG` environment variable is honoured by
`env_logger` and is also considered public API.

## Determinism

For a given input byte sequence and a given crate version of
[`nhs-number`](https://crates.io/crates/nhs-number), the stdout and
stderr output streams are deterministic byte-for-byte (modulo the
unspecified interleaving between the two streams when both are captured
to the same sink).

The order of lines *within* each stream is the input order.

## Patient-data hygiene

* The binary must never log, cache, persist, or transmit any input line
  beyond echoing it to the stdout/stderr streams described above.
* The binary must never read configuration that could redirect output to
  the network or to a file outside the user's chosen redirection.
* All committed test data must be drawn from the NHS synthetic test
  range `999 000 0000`–`999 999 9999`.

## When you must break the contract

1. Open an issue describing the change and which downstream callers it
   may affect.
2. Bump the major version in `Cargo.toml`.
3. Update [`spec.md`](../spec.md): the relevant FR or NFR's status must
   change to `Changed in vX.0.0`.
4. Update [`docs/usage/index.md`](../docs/usage/index.md) and the example
   fixtures.
5. Note the change at the top of the release notes.

<!-- cSpell:ignore confy crates thiserror -->
