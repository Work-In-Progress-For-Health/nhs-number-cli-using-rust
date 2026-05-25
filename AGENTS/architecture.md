# Architecture

This file explains how `nhs-number-cli` is wired together. The end-user
architecture document lives at
[`../docs/architecture/index.md`](../docs/architecture/index.md); this
file is the agent-facing twin and goes one level deeper into module
responsibilities and conventions.

For the numbered behavioural requirements that these modules satisfy,
see [`../spec.md`](../spec.md).

## Module graph

```
main.rs
  └── app::run::run()                         (top-level orchestration)
        ├── app::confy / app::config::Config  (load persisted config)
        ├── app::clap / app::args::Args       (parse CLI into a typed struct)
        └── subcommands::*                    (dispatch by subcommand flag)
              └── subcommands::check_lines    (the line-validation filter)
```

Everything outside `main.rs` is `pub(crate)` and not part of any
external API. Downstream consumers depend on the *binary*, not on the
library crate.

## Folders and what belongs where

```
src/
├── main.rs                     Binary entry point. Initialise env_logger,
│                               call app::run::run(), translate Result to
│                               process exit code.
├── testing.rs                  Test-only path helpers usable from any
│                               in-crate `#[cfg(test)] mod tests`.
├── app/
│   ├── mod.rs                  Module declarations.
│   ├── args.rs                 `pub struct Args` — typed view of the CLI.
│   │                           Add a field here whenever clap.rs grows a
│   │                           flag, and update `Default`.
│   ├── clap.rs                 `clap::Command` builder + argument matching
│   │                           into `Args`. Tests in this file drive the
│   │                           compiled binary via `assert_program_args_*`.
│   ├── config.rs               `pub(crate) struct Config` for persisted
│   │                           settings. Owns `serde` derive + a `Default`.
│   ├── confy.rs                Confy load/save tests for `Config`.
│   └── run.rs                  `run() -> Result<(), Error>`: initialise
│                               config, parse args, dispatch a subcommand.
├── subcommands/
│   ├── mod.rs                  Re-declare each subcommand module.
│   └── check_lines.rs          The default subcommand. Reads stdin,
│                               parses each line as an NHSNumber, validates
│                               the check digit, routes to stdout or stderr.
└── types/
    ├── mod.rs                  Re-export `list`, `map`, `queue`, `set`,
    │                           `stack` with `#[macro_use]`.
    ├── list.rs                 `pub type List<T> = Vec<T>;` + `list!` macro.
    ├── map.rs                  `pub type Map<K,V> = BTreeMap<K,V>;` + macro.
    ├── queue.rs                `pub type Queue<T> = VecDeque<T>;` + macro.
    ├── set.rs                  `pub type Set<T> = BTreeSet<T>;` + macro.
    └── stack.rs                `pub type Stack<T> = Vec<T>;` + macro.
```

External, in-repository:

```
tests/test.rs                   Cargo integration test. Spawns the compiled
                                binary, pipes a fixed input, asserts the
                                first stdout and stderr lines.
examples/NN-<slug>/             Runnable examples: input.txt, expected
                                stdout/stderr, and run.sh.
```

## Design conventions

* **Thin `main.rs`.** `main` should only initialise logging, call into
  `app::run::run()`, and translate a `Result` into an exit code. No
  business logic. Maps to `spec.md` § FR-12.
* **One subcommand per file.** Each file under `src/subcommands/` owns
  its own `Error` enum (via `thiserror`) and its own integration of any
  crates it needs. Cross-subcommand utilities go in `src/app/` or
  `src/types/`.
* **Args struct decoupled from clap.** `Args` is a plain Rust struct so
  another parser (or hand-built integration test) could populate it
  directly. `clap.rs` converts `clap::ArgMatches` into `Args`. Maps to
  `spec.md` § FR-13.
* **No global mutable state.** Pass `Args` and `Config` down the call
  stack. The `log` crate macros (`trace!`, `debug!`, etc.) are fine;
  they hit the global `env_logger` and stay on stderr.
* **Path constants via `LazyLock`.** Anywhere a test needs the compiled
  binary path, take it from `crate::testing::COMMAND_OS`. Do not
  reconstruct it inline.
* **Subcommand dispatch via `app::run::dispatch`.** A no-flag invocation
  must keep doing what it does today (run `check_lines`). New
  subcommands opt in via their own flag. Maps to `spec.md` § FR-16.

## Why a module split?

The original program was a single `fn main`. It grew flags,
configuration, logging, and (eventually) more subcommands. The split
exists to keep each concern testable in isolation:

* `clap.rs` tests verify CLI parsing without invoking domain logic.
* `subcommands/check_lines.rs` tests verify domain logic without going
  through clap.
* `tests/test.rs` verifies the wired-up binary as a process.

Resist re-collapsing the split. Resist further splitting until a
concrete concern needs its own home (for example: a second subcommand,
a non-stdin input source, a `--format` option that needs a formatter
trait).

## Where new code goes

| You want to…                              | Put it in                              | Spec link |
| ----------------------------------------- | -------------------------------------- | --------- |
| Add a CLI flag                            | `src/app/clap.rs` + `src/app/args.rs`  | FR-13     |
| Add a persisted setting                   | `src/app/config.rs`                    | FR-14     |
| Add a new subcommand                      | `src/subcommands/<name>.rs` + register | FR-16     |
|                                           | in `src/subcommands/mod.rs`            |           |
| Add a test helper used by ≥ 2 test modules| `src/testing.rs`                       | —         |
| Add a shared collection alias / macro     | `src/types/<name>.rs`                  | —         |
| Add a runnable example                    | `examples/NN-<slug>/`                  | NFR-10    |
| Add a stable behaviour you must not break | `spec.md` (new FR or NFR entry)        | —         |

<!-- cSpell:ignore confy clap LazyLock VecDeque BTreeMap BTreeSet -->
