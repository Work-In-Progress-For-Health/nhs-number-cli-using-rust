# Coding style

Rust style for `nhs-number-cli`. Applies to every `.rs` file in the
repository. See [`AGENTS/architecture.md`](./architecture.md) for *where*
code goes; this file is *how* it should look.

## Edition and toolchain

* **Edition:** 2024. Do not change `edition` in `Cargo.toml` without a
  ticket.
* **MSRV:** Rust 1.85 (the minimum that supports edition 2024).
* **Formatter:** `cargo fmt`. Run it before every commit. Do not hand-format
  against `rustfmt`.
* **Linter:** `cargo clippy -- -D warnings`. Treat any clippy warning as a
  failure to fix, not a comment to suppress. Allowlist a lint only with a
  one-line justification.

## File-level conventions

* Begin each file with a `//!` doc comment that names the file's purpose
  in one sentence and, where applicable, points to the partner file
  (e.g. `clap.rs` ↔ `args.rs`, `config.rs` ↔ `confy.rs`).
* Group `use` statements: `std` first, then external crates
  (alphabetised), then in-crate `use crate::…` paths. A blank line
  between groups is fine; not required.
* `#[cfg(test)] mod tests { … }` goes at the end of the file. Inside, use
  `use super::*;` if needed, never `use crate::module::*` for the module
  under test.

## Naming

| Item              | Convention                                                  |
| ----------------- | ----------------------------------------------------------- |
| Modules / files   | `snake_case`. Match the file name to the module name.       |
| Types / traits    | `UpperCamelCase`.                                           |
| Functions / vars  | `snake_case`.                                               |
| Constants         | `SCREAMING_SNAKE_CASE`.                                     |
| Lifetimes         | Short and lowercase (`'a`, `'src`). Avoid `'static_` etc.   |
| Error enums       | `Error` (one per module). Variants are nouns or noun phrases. |

## Error handling

* Use [`thiserror`](https://crates.io/crates/thiserror) for in-crate error
  enums. One `enum Error` per module that produces errors, with one
  variant per failure mode.
* Each variant carries the data needed to make the message useful: a line
  number, the offending string, the underlying error. See
  `subcommands/check_lines.rs` for the canonical pattern.
* The `#[error("…")]` message **is** the user-facing message. Treat the
  format as observable; downstream scripts grep it. The current format is:
  `Error <reason> ➡ <field>: <value>, <field>: <value>`. Stay close to
  this format when adding new variants.
* Never `unwrap()` or `expect()` on the runtime path. Both are acceptable
  inside `#[cfg(test)]` code and in `build.rs` (if one is ever added).
* `Result<T, Error>` propagates with `?`. Convert at module boundaries with
  `#[from]` so the call site stays clean.

## Stdout / stderr discipline

* `println!` → only valid NHS Numbers (i.e. user-requested output).
* `eprintln!` → only diagnostics about a single input line.
* `trace!` / `debug!` / `info!` / `warn!` / `error!` → operational logging
  via `env_logger`. Lands on `stderr`. Off by default; turn on with
  `--verbose` flags or `RUST_LOG=…`.

Never use `println!` to log; never use `eprintln!` for normal output.

## Comments and documentation

* Prefer `///` doc comments on public-ish items (`pub`, `pub(crate)`).
* Default to no inline comments. Add one only when the *why* is
  non-obvious: a hidden constraint, a workaround for a specific bug, a
  format that scripts grep.
* Do not write comments that restate the code (`// loop over lines`).
* Do not leave `// TODO: refactor` style hints unless paired with an issue
  number.

## Importing collection types

Prefer the project-local aliases when readability benefits:

```rust
use crate::types::{list::*, map::*, set::*};

let mut seen: Set<String> = Set::new();
let counts: Map<&str, i32> = map!("valid" => 0, "invalid" => 0);
```

`Vec` and `HashMap` directly from `std` are fine when no macro is needed.
Don't mix `Vec<T>` and `List<T>` in the same function — pick one.

## Macros

* In-crate macros (`list!`, `map!`, etc.) are exposed via `#[macro_use]`
  on `mod types`. Macro definitions live next to the type alias they
  produce.
* New macros require a clear win over a function. If a function can do it,
  write a function.

## Tests inside source files

* Unit tests live in `#[cfg(test)] mod tests { … }` blocks at the bottom
  of each file.
* Use the [`assertables`](https://crates.io/crates/assertables) macros for
  process-style assertions (`assert_program_args_stdout_string_contains!`,
  etc.). Plain `assert!`, `assert_eq!`, and `assert_matches!` are fine for
  in-process checks.
* See [`AGENTS/testing.md`](./testing.md) for the full testing strategy.

## Performance

This is not a hot-path tool, but:

* Process input line-by-line. Do not buffer the entire input into memory.
* Avoid `to_string()` / `clone()` in the per-line loop unless you need an
  owned value (e.g., to attach to an `Error` variant).
* Do not introduce async runtimes. The program is synchronous I/O over
  stdin/stdout.

## What not to do

See [`AGENTS/avoid.md`](./avoid.md) for the project-wide "do not"
list. The most-violated:

* No TUIs, no colours, no progress bars.
* No reading files directly (callers redirect into stdin).
* No swallowed errors. Every bad input line must produce exactly one
  diagnostic on stderr.

<!-- cSpell:ignore allowlist assertables clippy confy crates ScreamingSnakeCase thiserror upper -->
