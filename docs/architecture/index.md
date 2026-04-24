# Architecture

`nhs-number-cli` is intentionally small. This document explains how the
pieces fit together and why the design is the way it is.

## Components

```
stdin ──► main loop ──► NHSNumber::from_str ──► validate_check_digit
                                                 │
                                                 ├── true  ──► stdout
                                                 └── false ──► stderr
                           │
                           └── Err(parse) ──────────────────► stderr
```

1. `main` opens `stdin` and locks it to get a buffered reader.
2. Each line is matched. Read errors (rare; usually I/O) are reported on
   `stderr` with the line index.
3. Blank lines are skipped.
4. Non-blank lines are parsed with `nhs_number::NHSNumber::from_str`.
5. On parse success, `validate_check_digit()` is called on the value.
6. Output is written to `stdout` on success, `stderr` on any failure.

## Why a single file?

The entire program is about 30 lines of logic. Splitting it into modules
would not help the reader, and the observable contract (behavioural, not
API-level) is what the integration test exercises. If this grows into
something more than a thin wrapper — for example by accepting flags for
output formats or adding summary statistics — splitting becomes worth the
cost.

## Why stdin only?

A Unix filter composes with every other tool. Supporting a `--file`
option is redundant with `<` redirection and `cat`. It would also invite
the tool to start growing into an application: reading files implies
encoding decisions, error messages about missing files, globs, and so on.

## Why exit code 0 even on errors?

Downstream tools (`grep`, `awk`, `sort`) treat a zero exit code as "I
processed your stream successfully." The tool reports individual line
errors on `stderr`; the overall process of "read each line and report on
it" has not failed. If a caller needs a non-zero exit on any invalid
input, wrapping the tool in a five-line shell function is trivial; baking
that policy into the tool would surprise the many callers that prefer
the current behaviour.

## Dependencies

* [`nhs-number`](https://crates.io/crates/nhs-number) — parsing and check
  digit validation. All the domain logic lives here. Pinning this crate
  to a known-good version gives us reproducibility.
* [`serde`](https://crates.io/crates/serde) — pulled in transitively for
  `NHSNumber`'s derived serde implementations. The CLI itself does not
  serialise or deserialise anything, but removing the feature would
  complicate dependency pinning without shrinking the binary
  appreciably.

No runtime, no async framework, no argument parser, no logging library.

## Integration test

`tests/test.rs` spawns the compiled binary under `target/debug/` and
pipes a fixed input through it. It asserts the first `stdout` line and
the first `stderr` line. Because the test drives the binary as a real
process, it exercises the actual user-visible contract, not internal
functions. Adding more observable behaviours (e.g. new output formats)
should expand this test.

## Release artifacts

`cargo build --release` produces a static-ish binary at
`target/release/nhs-number-cli`. A cross-compiled Windows binary is
tracked at `target/x86_64-pc-windows-gnu/release/nhs-number-cli.exe`.
Additional targets can be added by using `rustup target add` and
`cargo build --release --target <triple>`.
