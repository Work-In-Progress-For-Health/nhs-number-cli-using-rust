# 11-flag-demo

Exercises every public command-line flag and the `RUST_LOG`
environment variable. This is the runnable counterpart to
[`spec.md`](../../spec.md) **FR-13**, **FR-15**, and **FR-16**, and
closes the NFR-10 runnable-examples gap for the flag-related
requirements.

The example does *not* try to be a tutorial — for that, see
[`docs/usage/index.md`](../../docs/usage/index.md). It is a regression
test: each flag's contract should be enforced here so that an
accidental rename or removal fails `./run-all.sh`.

## What it exercises

1. **`--version`** prints `nhs-number-cli <X.Y.Z>` on stdout.
2. **`--help`** lists every public flag (`--line-validation`, `--test`,
   `--verbose`, `--version`, `--help`).
3. **`-l` / `--line-validation`** runs the line-validation subcommand
   explicitly. Stdout/stderr match the same fixture used by the
   no-flag default (FR-16: the explicit flag is equivalent to the
   default).
4. **`--test --verbose --verbose`** prints the parsed `Args` struct on
   stdout, with `log_level: Some(Warn)` confirming that the count of
   `--verbose` flags maps onto `::log::Level` (FR-13 verbose mapping).
5. **`RUST_LOG=trace …`** routes the `log` macros' output to stderr
   via `env_logger`. We assert that at least one trace line
   (`nhs_number_cli::app::run :: run`) appears, *and* that the
   per-line diagnostic still appears with its stable `Error ` prefix.

## Files

* `input.txt` — one valid + one invalid number, reused by the
  `--line-validation` and `RUST_LOG=trace` cases.
* `expected-stdout.txt`, `expected-stderr.txt` — exact bytes for the
  line-validation cases. Reused by both invocations.
* `run.sh` — the shell harness. Diff for deterministic streams; grep
  for help text and log lines (whose terminal-width-dependent or
  timestamped formatting is not byte-stable).

## Run

```sh
./run.sh
```

Prints `OK` and exits 0 on success.
