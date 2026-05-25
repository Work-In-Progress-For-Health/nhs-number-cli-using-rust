# AGENTS.md

Guidance for AI coding agents (Claude Code, GitHub Copilot, Cursor, Aider,
Codex, Continue, and similar) working in this repository. Follow these
conventions in addition to any instructions in the active conversation,
and defer to the user when a conversational instruction conflicts with
this file.

> If you only read one document, read this one. The topical files under
> [`AGENTS/`](./AGENTS/) expand each section in depth, and
> [`spec.md`](./spec.md) is the living source of truth for *what* the
> binary must do (the *how* lives in the code).

## Project at a glance

`nhs-number-cli` is a small Rust command line tool. Its core feature is
a Unix-style filter that validates National Health Service (NHS) Numbers
from standard input:

* Valid NHS Numbers are printed to standard output in canonical
  `NNN NNN NNNN` form.
* Invalid or unparseable lines produce a diagnostic on standard error.
* Blank lines are skipped.
* Exit code is `0` on EOF regardless of how many lines were invalid.

The validation logic lives in the
[`nhs-number`](https://crates.io/crates/nhs-number) crate; this binary
is the thin shell that composes that crate with stdin/stdout. The
binary is designed to compose with Unix pipelines (`cat`, `grep`,
`sort`, `awk`, `cut`, `psql`, PowerShell, etc.).

## Where to look

| Concern                                  | Read                                                                 |
| ---------------------------------------- | -------------------------------------------------------------------- |
| Functional & non-functional requirements | [`spec.md`](./spec.md)                                               |
| Module layout and dependency graph       | [`AGENTS/architecture.md`](./AGENTS/architecture.md)                 |
| Rust style, formatting, error handling   | [`AGENTS/coding-style.md`](./AGENTS/coding-style.md)                 |
| How to write and run tests               | [`AGENTS/testing.md`](./AGENTS/testing.md)                           |
| Public behaviour you must not break      | [`AGENTS/behavioural-contract.md`](./AGENTS/behavioural-contract.md) |
| Why each dependency is in `Cargo.toml`   | [`AGENTS/dependencies.md`](./AGENTS/dependencies.md)                 |
| Commit, branch, release conventions      | [`AGENTS/commit-and-pr.md`](./AGENTS/commit-and-pr.md)               |
| Things to avoid                          | [`AGENTS/avoid.md`](./AGENTS/avoid.md)                               |
| End-user documentation                   | [`docs/index.md`](./docs/index.md)                                   |
| Runnable example fixtures                | [`examples/README.md`](./examples/README.md)                         |

## Quick start for an agent

```sh
cargo build                  # debug build (required before integration tests)
cargo test                   # run unit + integration tests
cargo fmt                    # format
cargo clippy -- -D warnings  # lint with warnings denied
cargo run -- --line-validation < examples/01-basic/input.txt
./examples/run-all.sh        # exercise every runnable example
```

The integration test in `tests/test.rs` and the per-`mod tests` blocks
in `src/app/clap.rs` both spawn the compiled debug binary via
`std::process::Command`. They depend on the binary existing at
`target/debug/nhs-number-cli`. Do **not** change that path without
also updating every test that hardcodes it (search `COMMAND_OS` and
`nhs-number-cli` literals).

## Spec-driven workflow

This project follows lightweight **spec-driven development**. Before
you write or change code:

1. Find or add the matching `FR-…` / `NFR-…` entry in
   [`spec.md`](./spec.md).
2. Make sure its acceptance criteria are concrete and observable.
3. Implement, write tests, and (if behaviour is observable per input
   line) add a runnable example under `examples/`.
4. Update the requirement's traceability block in the same commit.

A PR that changes observable behaviour without amending `spec.md` is
incomplete and should not merge. See
[`AGENTS/commit-and-pr.md`](./AGENTS/commit-and-pr.md).

## Repository layout

```
.
├── AGENTS.md              This file: agent guidance entry point
├── AGENTS/                Topical agent guidance (read on demand)
├── CITATION.cff           Citation metadata
├── CODE_OF_CONDUCT.md     Contributor Covenant
├── CONTRIBUTING.md        How to contribute
├── Cargo.lock             Locked dependency versions
├── Cargo.toml             Crate manifest
├── README.md              User-facing documentation entry point
├── cspell.json            Spell-check word list
├── docs/                  Long-form end-user documentation (topic per dir)
├── examples/              Runnable examples with expected stdout/stderr
├── releases/              Prebuilt binaries (by target triple)
├── spec.md                Living specifications (spec-driven development)
├── src/
│   ├── main.rs            Binary entry point (thin: delegates to app::run)
│   ├── testing.rs         Integration-test helpers (paths to compiled binary)
│   ├── app/               Application wiring: clap, confy, args, config, run
│   ├── subcommands/       One module per subcommand (e.g. check_lines)
│   └── types/             Project-local aliases for std collections + macros
└── tests/
    └── test.rs            Integration test that pipes input into the binary
```

## Non-negotiable rules

1. **Stream contract is public API.** The split between `stdout` (valid
   numbers) and `stderr` (diagnostics, one per bad line) is observable
   behaviour that downstream scripts rely on. Changing it is a breaking
   change. See
   [`AGENTS/behavioural-contract.md`](./AGENTS/behavioural-contract.md)
   and [`spec.md`](./spec.md) (FR-5 through FR-11).
2. **No real patient data, ever.** Use only NHS-published synthetic
   test ranges (`999 000 0000`–`999 999 9999`) in code, tests, fixtures,
   commit messages, issues, PR descriptions, and AI prompts. See
   [`examples/README.md`](./examples/README.md) and `spec.md` § NFR-5.
3. **Cargo is the build system.** Do not introduce `make`, `just`,
   `npm`, `cmake`, or shell-script orchestrators alongside it. Scripts
   that *use* Cargo are fine.
4. **Edition 2024, MSRV Rust 1.85.** Keep both unless a dependency
   forces a change. See `spec.md` § NFR-9.
5. **Multi-licence compatibility.** Any new dependency must be
   compatible with
   `MIT OR Apache-2.0 OR GPL-2.0 OR GPL-3.0 OR BSD-3-Clause`. When in
   doubt, prefer crates dual-licensed `MIT OR Apache-2.0`. See
   `spec.md` § NFR-6.
6. **No network, no telemetry, no persistence.** The binary must not
   phone home, read environment for endpoints, or open sockets.
   Logging via `env_logger` writes to `stderr` only. See `spec.md`
   § NFR-4.

## When in doubt

* Prefer the smaller change. This is a small tool; resist refactors
  that inflate the surface area without a concrete behaviour to
  support.
* Add a runnable example under `examples/` for any new observable
  behaviour. Update [`spec.md`](./spec.md) in the same commit.
* Open a GitHub issue if a change would alter the behavioural contract
  or the licence set.

## Contact

For anything that is not covered here or in the topical files, open a
GitHub issue or email <joel@joelparkerhenderson.com>.

<!-- cSpell:ignore Aider Codex confy clippy crates -->
