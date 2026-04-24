# AGENTS.md

Guidance for AI coding agents (Claude Code, GitHub Copilot, Cursor, Aider, and
similar) working in this repository. Follow these conventions in addition to
any instructions in the active conversation.

## Project overview

`nhs-number-cli` is a small Rust command line tool that validates National
Health Service (NHS) Numbers, one per line, from standard input.

* Valid NHS Numbers are printed to standard output.
* Invalid or unparseable lines produce a diagnostic on standard error.
* Blank lines are skipped.

The program is a thin shell around the
[`nhs-number`](https://crates.io/crates/nhs-number) crate. The binary is
designed to compose with Unix pipelines (`cat`, `grep`, `sort`, `awk`, etc.).

## Repository layout

```
.
├── AGENTS.md              This file
├── CITATION.cff           Citation metadata
├── CODE_OF_CONDUCT.md     Contributor Covenant
├── CONTRIBUTING.md        How to contribute
├── Cargo.lock             Locked dependency versions
├── Cargo.toml             Crate manifest
├── README.md              User-facing documentation
├── docs/                  Long-form documentation
├── examples/              Sample inputs, scripts, and expected outputs
├── releases/              Prebuilt binaries (by target triple)
├── src/
│   └── main.rs            Entry point and the entire program
└── tests/
    └── test.rs            Integration test that invokes the compiled binary
```

The program is deliberately small. There are no library modules and no
additional binaries. Resist the urge to split `src/main.rs` into modules
unless the scope of the program changes substantially.

## Build, run, and test

Use Cargo for every task. Do not introduce alternative build systems.

```sh
cargo build                 # debug build
cargo build --release       # optimized build
cargo run                   # run the debug binary against stdin
cargo test                  # run the integration test
cargo fmt                   # format the code
cargo clippy -- -D warnings # lint with warnings denied
```

The integration test in `tests/test.rs` spawns the compiled debug binary via
`std::process::Command`. That means `cargo test` implicitly requires that
`cargo build` succeed first; Cargo handles this for you, but note that the
test relies on the binary existing at
`target/debug/nhs-number-cli`. Do not change that path without also updating
the test.

## Cross-compilation

A Windows GNU binary lives at
`target/x86_64-pc-windows-gnu/release/nhs-number-cli.exe` and is tracked in
git for convenience. To rebuild it on macOS or Linux with `mingw-w64`
installed:

```sh
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

Other target triples are welcome as prebuilt releases under `releases/`.

## Code style

* Rust edition 2024. Keep it that way unless a dependency forces a change.
* Format with `cargo fmt` before committing.
* Prefer the standard library over new dependencies. New crates need a good
  reason and a compatible license (see "Licensing" below).
* The program is a single `fn main`. Keep it readable top-to-bottom. If you
  need to add a helper, put it in `src/main.rs` above `main`.
* Error messages go to `stderr` with the format:
  `Error <reason> line <n>. Error: <detail>. <context>`
  Keep this format stable; downstream scripts grep for it.
* Avoid `unwrap()` and `expect()` in `main`. Pattern match and report a clear
  message on `stderr` instead.
* Do not introduce `println!` on the error path or `eprintln!` on the success
  path. Valid NHS Numbers are stdout; everything else is stderr.

## Behavioural contract

These behaviours are public API. Breaking them is a breaking change and
requires a major version bump.

1. Input is read one line at a time from `stdin` until EOF.
2. A blank line is silently skipped (no output on either stream).
3. A line that parses as an NHS Number and has a valid check digit is
   printed to `stdout` in the canonical `NNN NNN NNNN` format produced by
   `NHSNumber::Display`.
4. A line that parses but fails check-digit validation is reported on
   `stderr` and does not appear on `stdout`.
5. A line that fails to parse is reported on `stderr` and does not appear on
   `stdout`.
6. Line numbering in error messages starts at 0 (it comes from
   `Iterator::enumerate`). Do not silently change it to 1-based; scripts
   depend on it. If you must change it, bump the major version and note it
   in the release notes.
7. The exit code is always 0. Invalid lines do not cause a non-zero exit.
   Callers that need to detect failure should check whether anything was
   written to `stderr` or compare line counts.

## Licensing

The crate is multi-licensed: `MIT OR Apache-2.0 OR GPL-2.0 OR GPL-3.0 OR
BSD-3-Clause`. Any new dependency must be compatible with all of these.
When in doubt, prefer crates dual-licensed `MIT OR Apache-2.0`.

## Testing additions

When you add a feature, add a test that drives the compiled binary over a
pipe, in the style of `tests/test.rs`. Unit tests inside `src/main.rs` are
acceptable but not preferred, because the contract of this program is
observable behaviour, not internal structure.

## Things to avoid

* Do not add a TUI, interactive prompts, colours, or progress bars. The
  tool must behave well in pipelines and under redirection.
* Do not add network access, telemetry, or logging frameworks.
* Do not read files directly; read from `stdin`. Callers redirect files in
  themselves (`cat input.txt | nhs-number-cli`).
* Do not swallow parse errors. Every invalid line must produce exactly one
  diagnostic on `stderr`.
* Do not commit patient data, real NHS Numbers, or anything that could
  identify a real person. Use the documented test numbers (see
  `examples/`).

## Contact

For anything that is not covered here, open a GitHub issue or email
<joel.henderson@wales.nhs.uk>.
