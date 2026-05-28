# Architecture

`nhs-number-cli` is a thin shell around the
[`nhs-number`](https://crates.io/crates/nhs-number) crate. The binary
itself does no domain logic; it wires together command-line parsing,
configuration loading, logging, and a small set of subcommands that
each consume `stdin` and emit `stdout` / `stderr`.

For the agent-facing twin of this document (one level deeper into
module responsibilities), see
[`../../AGENTS/architecture.md`](../../AGENTS/architecture.md). For the
numbered behavioural requirements that the architecture satisfies,
see [`../../spec.md`](../../spec.md).

## Module graph

```
main                                          src/main.rs
 └── env_logger::init
 └── app::run::run() -> Result<(), Error>     src/app/run.rs
       ├── app::confy / app::config::Config   src/app/{confy,config}.rs
       ├── app::clap / app::args::{Args, Format}  src/app/{clap,args}.rs
       └── dispatch(&Args)                    src/app/run.rs
             ├── subcommands::counts          src/subcommands/counts.rs  (FR-17)
             └── subcommands::check_lines     src/subcommands/check_lines.rs  (FR-1..FR-12)
```

`pick_column`, `json_escape`, and `tsv_escape` live in
`src/subcommands/mod.rs` as shared helpers used by both subcommands
(FR-18, FR-19).

## Per-line flow (default and `--line-validation`)

```
stdin
  │ (read line)
  ▼
BufRead::lines()
  │
  ├── Err(io)        ──► "Error reading line N. Error: <e>"            (FR-9)
  │
  └── Ok(line)
       │
       ├── line.is_empty()                ──► (silently skipped)        (FR-2)
       │
       ▼
   `--column N` set?
       │
       ├── yes, field N missing           ──► "Error parsing line N. Error: ColumnMissing(N). Line: <row>"  (FR-18)
       │
       └── candidate field selected
              │
              ▼
        NHSNumber::from_str(candidate)
              │
              ├── Err(parse)              ──► "Error parsing line N. Error: <e>. Line: <row>"  (FR-7)
              │
              └── Ok(nhs_number)
                     │
                     ▼
                validate_check_digit()
                     │
                     ├── true             ──► stdout: nhs_number (canonical NNN NNN NNNN form)  (FR-5)
                     │
                     └── false            ──► "Error invalid line N. Error: validate check digit failed. NHS Number: <nhs_number>"  (FR-6)
```

The `--format` flag swaps the stderr template for a JSON line or a
fixed five-column TSV row; stdout (valid NHS Numbers) is plain text
in every format (FR-19).

## Per-line flow (`--counts`)

Same classification, but instead of emitting per-line output the
subcommand accumulates four counters and prints a four-row summary at
EOF (FR-17). With `--format json` the summary is a single object;
with `--format tsv` it's a header row followed by an integer row.

## Why a module split?

The binary started life as a single `fn main`. It grew CLI flags,
configuration loading, logging, and (now) multiple subcommands. The
split exists so each concern is testable in isolation:

* `src/app/clap.rs` tests verify CLI parsing without invoking the
  domain layer.
* `src/subcommands/check_lines.rs` tests (and the `#[cfg(test)] mod
  tests` block in `src/subcommands/mod.rs` covering `pick_column`,
  `json_escape`, `tsv_escape`) verify domain logic without going
  through clap.
* `tests/test.rs` verifies the wired-up binary as a process.

Resist re-collapsing the split. Resist further splitting until a
concrete concern needs its own home.

## Why stdin only?

A Unix filter composes with every other tool. Supporting a `--file`
option is redundant with `<` redirection and `cat`. It would also
invite the tool to start growing into an application: reading files
implies encoding decisions, error messages about missing files,
globs, and so on. `--column N` is the closest the binary gets to
file-format awareness, and even then it only operates on whatever
arrives on `stdin`.

## Why exit code 0 even on errors?

Downstream tools (`grep`, `awk`, `sort`) treat a zero exit code as
"I processed your stream successfully." The tool reports individual
line errors on `stderr`; the overall process of "read each line and
report on it" has not failed. Callers that need a non-zero exit on
any invalid input can wrap the tool in a five-line shell function
(see [`examples/07-fail-on-invalid/`](../../examples/07-fail-on-invalid/));
baking that policy into the tool would surprise the many callers
that prefer the current behaviour. See FR-12.

## Dependencies

Eight runtime crates, each with a stated reason in
[`../../AGENTS/dependencies.md`](../../AGENTS/dependencies.md):

* [`nhs-number`](https://crates.io/crates/nhs-number) — parsing and
  Modulus 11 check-digit validation. All domain logic lives here.
* [`clap`](https://crates.io/crates/clap) — command-line argument
  parsing.
* [`confy`](https://crates.io/crates/confy) — boilerplate-free TOML
  configuration loading from the OS-appropriate path.
* [`serde`](https://crates.io/crates/serde) — derives
  `Serialize`/`Deserialize` on `Config`.
* [`thiserror`](https://crates.io/crates/thiserror) — derives
  `std::error::Error` on per-module `Error` enums.
* [`log`](https://crates.io/crates/log) — logging facade.
* [`env_logger`](https://crates.io/crates/env_logger) — routes `log`
  macros to `stderr` via the `RUST_LOG` variable.
* [`assertables`](https://crates.io/crates/assertables) — process-aware
  assertion macros for `#[cfg(test)]` blocks.

No async runtime. No HTTP client. No serialization framework beyond
what `serde` and the hand-rolled JSON in `src/subcommands/mod.rs`
provide.

## Integration tests

`tests/test.rs` spawns the compiled binary under `target/debug/` (or
`target/debug/nhs-number-cli.exe` on Windows, via the `.exe`
fallback) and exercises every observable requirement:

* `test` — the round-trip baseline (FR-3, FR-4, FR-5, FR-6, FR-10).
* `fr_2_blank_lines_skipped` — blank-line handling.
* `fr_7_parse_failure_to_stderr` — parse-error diagnostics.
* `fr_9_read_error_to_stderr` — feeds invalid UTF-8 to provoke a
  real `BufRead::lines()` error.
* `fr_11_stream_separation` — valid-invalid-valid input cleanly splits.
* `fr_12_exit_zero_on_per_line_failures` — exit code is 0 on bad input.
* `fr_17_counts_summary` — `--counts` produces the four-row summary.
* `fr_18_column_extraction` — `--column N` extracts and reports
  `ColumnMissing(N)`.
* `fr_19_format_json`, `fr_19_format_tsv`,
  `fr_19_counts_format_json`, `fr_19_default_text_unchanged` —
  output-format coverage including the FR-10 byte-for-byte guard.

Plus 19 in-crate unit and in-crate-integration tests under
`src/**/`, including the `src/app/clap.rs::tests` module that
spawns the binary to verify CLI parsing.

The 12 runnable examples under `examples/` provide a third coverage
layer: each `run.sh` diffs observed stdout/stderr against committed
expected files, byte-for-byte. They double as living documentation
and as the FR-10 / NFR-8 regression bar.

## Release artifacts

`cargo build --release` produces an optimized binary at
`target/release/nhs-number-cli`. A cross-compiled Windows binary is
tracked at `target/x86_64-pc-windows-gnu/release/nhs-number-cli.exe`.
Additional targets can be added with `rustup target add` and a
matching `cargo build --release --target <triple>`; the new
artefact path needs an explicit negation in `.gitignore` to be
tracked.

Versions are recorded in [`../../CHANGELOG.md`](../../CHANGELOG.md).
