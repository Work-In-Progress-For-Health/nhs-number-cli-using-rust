# spec.md — nhs-number-cli

**Status:** Living document. **Version:** tracks the crate version in
[`Cargo.toml`](./Cargo.toml) (currently **0.2.0**). **Last reviewed:**
2026-05-24.

This file is the project's source of truth for *what* the binary must
do. The code in `src/` is the source of truth for *how*. When the two
disagree, the spec is authoritative and the code is a bug; raise an issue
and either change the code or amend the spec deliberately.

---

## 1. Purpose

`nhs-number-cli` is a Unix-filter command line tool that validates
National Health Service (NHS) Numbers, one per line, from standard
input. It is the operational front-end to the
[`nhs-number`](https://crates.io/crates/nhs-number) crate, packaged so
that data-engineering and clinical-informatics pipelines can call it
from any shell.

The binary deliberately exposes a small, stable surface so that
downstream callers (shell scripts, ETL jobs, CI checks, PowerShell
pipelines) can rely on it for years.

## 2. Methodology — how this spec is used

This project follows lightweight **spec-driven development (SDD)**:

1. **Specify first.** Every observable behaviour starts as a numbered
   requirement here (`FR-…` or `NFR-…`). No code or test is written
   until the relevant entry exists and has acceptance criteria.
2. **Trace.** Each requirement links to the test(s) that prove it and
   the example(s) that demonstrate it.
3. **Living.** This file is updated in the same commit as any change to
   observable behaviour. A PR that changes behaviour without amending
   this file is incomplete.
4. **Versioned.** Status fields move from `Planned` → `In progress` →
   `Implemented` → optionally `Deprecated in vX.Y.Z`. Status changes are
   visible in `git log` for this file.
5. **Audited.** Before every release, walk the spec top to bottom and
   confirm each `Implemented` line still matches the code.

## 3. Stakeholders

| Stakeholder                    | Interest                                                           |
| ------------------------------ | ------------------------------------------------------------------ |
| NHS data engineers             | Bulk validation of NHS Numbers in ETL pipelines.                   |
| Clinical informatics teams     | Spot-checking and triaging suspect identifiers.                    |
| Software developers            | Lightweight validator usable from any shell or CI environment.     |
| Information governance         | Patient-data hygiene; absence of telemetry and persistence.        |
| Project maintainer             | Small, stable surface; small dependency closure.                   |
| AI coding agents               | Predictable rules, complete enough to act on without guessing.     |

## 4. Glossary

| Term                | Definition                                                                                   |
| ------------------- | -------------------------------------------------------------------------------------------- |
| **NHS Number**      | A ten-digit identifier issued by NHS England, NHS Wales, and the NHS Isle of Man.            |
| **Check digit**     | The tenth digit of an NHS Number, computed from the first nine via the Modulus 11 algorithm. |
| **Canonical form**  | `NNN NNN NNNN` — three space-separated groups of three, three, four digits.                  |
| **Synthetic range** | `999 000 0000`–`999 999 9999`, reserved by the NHS for testing.                              |
| **Stream contract** | The set of guarantees about what appears on `stdout` and `stderr`. See § 7.                  |
| **Filter**          | A program that reads `stdin` and writes `stdout`/`stderr`; classical Unix sense.             |
| **Subcommand**      | A discrete mode of operation selectable by a flag. The default is line validation.           |

## 5. Scope

### 5.1 In scope

* Reading NHS Numbers one per line from `stdin`.
* Validating syntactic correctness and the Modulus 11 check digit.
* Reporting valid numbers on `stdout` and invalid lines on `stderr`.
* Configuration via flags and `confy`-loaded TOML.
* Logging via `env_logger` (controlled by `RUST_LOG` and `--verbose`).
* Distribution as source, via `cargo install`, and as prebuilt binaries.

### 5.2 Out of scope (explicit non-goals)

* Generating NHS Numbers, synthetic or otherwise.
* Verifying that an NHS Number belongs to a registered patient (no
  Personal Demographics Service / PDS lookup, no network).
* Validating CHI numbers (Scotland), H&C numbers (Northern Ireland), or
  other ten-digit healthcare identifiers from outside NHS England,
  NHS Wales, or the NHS Isle of Man.
* Reading files named on the command line (use `<` or `cat | …`).
* Reading or writing patient data over the network.
* Interactive use (TUI, prompts, colours, progress bars).

## 6. Functional requirements

Each requirement has an **ID**, a **statement**, **acceptance criteria**,
a **status**, and a **traceability** block.

---

### FR-1 — Read input from standard input

**Statement.** The binary reads its input from standard input only,
line by line, until end-of-file.

**Acceptance criteria.**

* No filename argument is accepted on the command line.
* Lines are terminated by `\n` (LF) or `\r\n` (CRLF). Both are handled
  identically because `BufRead::lines()` strips both.
* The binary terminates when `stdin` reaches EOF, not before.

**Status.** Implemented.

**Traceability.** `src/subcommands/check_lines.rs` ::
`check_lines()`; `tests/test.rs` :: `test`.

---

### FR-2 — Skip blank lines

**Statement.** A blank line in the input produces no output on any
stream.

**Acceptance criteria.**

* `""` after stripping `\r?\n` is treated as blank.
* No `stdout` line is produced.
* No `stderr` diagnostic is produced.
* Line numbering continues to advance: the next non-blank line's index
  is one greater than the blank line's index.

**Status.** Implemented.

**Traceability.** `src/subcommands/check_lines.rs` (the `if line ==
""` short-circuit); `examples/04-blank-lines/`.

---

### FR-3 — Parse each non-blank line as an NHS Number

**Statement.** Each non-blank input line is passed to
`nhs_number::NHSNumber::from_str` for parsing.

**Acceptance criteria.**

* Whitespace within a line of digits is tolerated, exactly as the
  upstream crate tolerates it (so `9991234560` and `999 123 4560` parse
  identically).
* Lines that cannot be parsed produce exactly one diagnostic on
  `stderr` and nothing on `stdout`. See FR-7.
* The binary does not re-implement parsing in any form.

**Status.** Implemented.

**Traceability.** `src/subcommands/check_lines.rs`;
`examples/10-parse-errors/`; `examples/03-mixed-formats/`.

---

### FR-4 — Validate the Modulus 11 check digit

**Statement.** A successfully parsed `NHSNumber` is validated by calling
`validate_check_digit()`.

**Acceptance criteria.**

* The check is the upstream crate's Modulus 11 algorithm — not a
  re-implementation.
* A `true` result routes the number to FR-5.
* A `false` result routes the line to FR-6.

**Status.** Implemented.

**Traceability.** `src/subcommands/check_lines.rs`;
[`docs/about-nhs-numbers/index.md`](./docs/about-nhs-numbers/index.md).

---

### FR-5 — Emit valid numbers on stdout in canonical form

**Statement.** A valid NHS Number is printed to `stdout` in the
canonical `NNN NNN NNNN` form produced by `NHSNumber`'s `Display` impl,
followed by `\n`.

**Acceptance criteria.**

* Exactly one `stdout` line per valid input line.
* The output line is the `Display` form, regardless of the input form
  (e.g. `9991234560` in → `999 123 4560` out).
* Order is preserved: outputs appear in the same order as their inputs.
* No additional whitespace, prefix, suffix, colour, or formatting is
  applied.

**Status.** Implemented.

**Traceability.** `src/subcommands/check_lines.rs`;
`tests/test.rs`; `examples/01-basic/expected-stdout.txt`.

---

### FR-6 — Emit failed-check-digit diagnostics on stderr

**Statement.** A line that parses but fails the check-digit validation
is reported on `stderr`.

**Acceptance criteria.**

* Exactly one `stderr` line per such input line.
* The line includes the zero-based input index and the parsed NHS
  Number (in canonical form).
* The line begins with `Error ` and includes the phrase
  `validate check digit failed`.
* Nothing is written to `stdout` for the same input line.

**Status.** Implemented.

**Traceability.** `src/subcommands/check_lines.rs` (`Error::CheckDigitError`);
`tests/test.rs`; `examples/01-basic/expected-stderr.txt`.

---

### FR-7 — Emit parse-failure diagnostics on stderr

**Statement.** A line that fails to parse as an NHS Number is reported
on `stderr`.

**Acceptance criteria.**

* Exactly one `stderr` line per such input line.
* The line includes the zero-based input index and the original line
  text.
* The line begins with `Error ` and includes the substring `parsing`.
* Nothing is written to `stdout` for the same input line.

**Status.** Implemented.

**Traceability.** `src/subcommands/check_lines.rs` (`Error::ParseError`);
`examples/10-parse-errors/`.

---

### FR-8 — Continue processing after a per-line failure

**Statement.** A bad input line (parse failure, check-digit failure, or
read error) does not stop the program. Processing continues with the
next line.

**Acceptance criteria.**

* In an input of N lines containing K failures and N-K valid numbers,
  the program produces N-K stdout lines and K stderr lines.
* The exit code is `0` regardless of K (see FR-12).

**Status.** Implemented.

**Traceability.** `src/subcommands/check_lines.rs` (loop continues);
`examples/01-basic/`.

---

### FR-9 — Read-error diagnostics on stderr

**Statement.** An I/O error encountered while reading a line is reported
on `stderr` and processing continues.

**Acceptance criteria.**

* The diagnostic includes the zero-based input index and the
  `std::io::Error` debug form.
* The diagnostic begins with `Error ` (matching the project's stable
  prefix).

**Status.** Implemented.

**Traceability.** `src/subcommands/check_lines.rs` (`Error::IOError`).

---

### FR-10 — Stable error-message format

**Statement.** The error format produced on `stderr` is observable
public API. Downstream scripts grep for `^Error ` and for the
field-bearing phrases.

**Acceptance criteria.**

* Every `stderr` diagnostic line begins with `Error `.
* Check-digit failures contain the substring `validate check digit
  failed` and the canonical NHS Number.
* Parse failures contain the substring `parsing` and the offending
  line.
* Line numbering is **zero-based** (it comes from
  `Iterator::enumerate`).
* Changes to any of the above are major-version breaks.

**Status.** Implemented.

**Traceability.** [`AGENTS/behavioural-contract.md`](./AGENTS/behavioural-contract.md);
`src/subcommands/check_lines.rs` (`#[error("…")]` annotations).

---

### FR-11 — Stream separation

**Statement.** `stdout` and `stderr` are independent streams. Within
each stream, output is in input order. The interleaving *between*
streams is not specified.

**Acceptance criteria.**

* Redirecting `stdout` does not affect what appears on `stderr`, and
  vice versa.
* A consumer that captures only `stdout` sees only valid numbers in
  input order.
* A consumer that captures only `stderr` sees only diagnostics in
  input order.

**Status.** Implemented.

**Traceability.** Behaviour of the standard `println!` / `eprintln!`
macros; `examples/05-separate-streams/`.

---

### FR-12 — Exit codes

**Statement.** The process exit code communicates *whether the program
ran*, not *whether the input was clean*.

**Acceptance criteria.**

* Exit code `0` after reaching EOF on `stdin`, regardless of how many
  per-line failures were reported.
* Exit code `1` if the program could not start (configuration load
  failure, panic, signal). The reason appears on `stderr`.
* Callers needing a non-zero exit on bad input wrap the call (see
  `examples/07-fail-on-invalid/`).

**Status.** Implemented.

**Traceability.** `src/main.rs` (the `Ok`/`Err` arms);
`examples/07-fail-on-invalid/`.

---

### FR-13 — CLI flags

**Statement.** The binary supports the flags listed below. Flag names,
short forms, and the count semantics of `--verbose` are public API.

| Flag                       | Effect                                                                |
| -------------------------- | --------------------------------------------------------------------- |
| `-l`, `--line-validation`  | Run the line-validation subcommand (the default behaviour).           |
| `--test`                   | Print the parsed `Args` and log level to `stdout` for diagnostics.    |
| `-v…`, `--verbose…`        | Increase log verbosity. Count → level: 1=error, 2=warn, 3=info, 4=debug, 5=trace. |
| `-V`, `--version`          | Print the crate version. Handled by clap.                             |
| `-h`, `--help`             | Print help. Handled by clap.                                          |

**Acceptance criteria.**

* `--version` prints the version from `CARGO_PKG_VERSION`.
* `--help` prints a non-empty help text that includes each documented
  flag and its short form (if any).
* `-vv` and `--verbose --verbose` set the same log level.
* `--test` output begins with the literal `Args {` so it can be
  asserted.

**Status.** Implemented.

**Traceability.** `src/app/clap.rs`; in-file `#[cfg(test)] mod tests`
in the same file.

---

### FR-14 — Configuration loading via confy

**Statement.** On start-up the binary loads a `Config` from the
OS-appropriate `confy` location. A missing file is not an error; the
default config is used.

**Acceptance criteria.**

* Configuration is read once, before any subcommand runs.
* A malformed file produces a startup error on `stderr` and exit code
  `1` (FR-12).
* The default config is usable without any file being present.

**Status.** Implemented (loading wired up; no fields currently affect
behaviour beyond `version`).

**Traceability.** `src/app/confy.rs`; `src/app/config.rs`; `src/app/run.rs`.

---

### FR-15 — Logging via env_logger and RUST_LOG

**Statement.** Operational logging is performed via the `log` macros
and routed by `env_logger` to `stderr`. Default level is off.

**Acceptance criteria.**

* `RUST_LOG=…` selects the level following `env_logger`'s standard
  semantics.
* `--verbose` flags raise the level above whatever `RUST_LOG` selects.
* Log lines are never printed to `stdout`.
* Log lines never include real or test NHS Numbers above `debug`
  level.

**Status.** Implemented.

**Traceability.** `src/main.rs` (`env_logger::init()`); `src/app/run.rs`;
[`AGENTS/coding-style.md`](./AGENTS/coding-style.md).

---

### FR-16 — Subcommand dispatch

**Statement.** The binary dispatches to a subcommand selected by flag.
At present there is one subcommand (`check_lines`); the architecture
admits more.

**Acceptance criteria.**

* When no subcommand flag is given, the line-validation behaviour runs.
  (Default behaviour preservation; the binary is useful as a bare
  filter.)
* When `--line-validation` is given explicitly, the same behaviour
  runs.
* Future subcommands must be opt-in via their own flag and must not
  change the no-flag default.

**Status.** Implemented.

**Traceability.** `src/subcommands/mod.rs`; `src/app/run.rs` (`dispatch`);
`tests/test.rs` (no-flag default); `src/app/clap.rs` `test_check_lines`
(explicit `--line-validation`).

---

## 7. Non-functional requirements

### NFR-1 — Streaming, constant-memory I/O

The binary processes input line by line and does not buffer the entire
input. Memory use is bounded by the longest single line.

**Status.** Implemented (`BufRead::lines()` iterator).

### NFR-2 — Portability

The binary builds and runs on Linux, macOS, and Windows. A prebuilt
Windows GNU binary is tracked at
`target/x86_64-pc-windows-gnu/release/nhs-number-cli.exe`.

**Status.** Implemented for x86_64 Linux/macOS/Windows. Other targets
build cleanly given the matching toolchain.

### NFR-3 — Composability

The binary is a Unix-style filter: stdin → stdout/stderr, no flags
required for the default behaviour, stable error format. It composes
with `cat`, `grep`, `sort`, `awk`, `cut`, `tr`, `psql`, PowerShell, and
similar.

**Status.** Implemented.

### NFR-4 — No network, no telemetry, no persistence

The binary does not open sockets, does not read environment variables
for endpoints, and does not write input lines to disk. Configuration
loaded via `confy` is read-only from the binary's perspective.

**Status.** Implemented; enforced by code review.

### NFR-5 — Patient-data hygiene

All committed test data uses the synthetic range `999 000 0000`–
`999 999 9999`. No real NHS Numbers appear in code, tests, fixtures,
commit messages, issues, or PR descriptions.

**Status.** Implemented; enforced by [`AGENTS/avoid.md`](./AGENTS/avoid.md)
and `examples/README.md`.

### NFR-6 — Licence compatibility

Every direct and transitive dependency must be compatible with
`MIT OR Apache-2.0 OR GPL-2.0 OR GPL-3.0 OR BSD-3-Clause`.

**Status.** Implemented; audited per release.

### NFR-7 — Small dependency closure

Each crate in `Cargo.toml` must have a stated reason in
[`AGENTS/dependencies.md`](./AGENTS/dependencies.md). Audit candidates
are removed at each release unless a concrete user story has materialised.

**Status.** Partially implemented (several "reserved for future use"
crates are currently flagged for audit).

### NFR-8 — Reproducible output

For a given input byte sequence and a pinned `nhs-number` crate
version, the stdout/stderr byte sequences are deterministic.

**Status.** Implemented (no nondeterministic sources — no time, no
random, no hash iteration on user-visible paths).

### NFR-9 — Minimum supported Rust version

Rust 1.85 (the minimum for edition 2024). Builds on older toolchains
are not supported.

**Status.** Implemented (`edition = "2024"` in `Cargo.toml`).

### NFR-10 — Documentation parity

Every observable behaviour is documented in three places: this file
(`spec.md`), the user docs (`docs/`), and a runnable example
(`examples/`).

**Status.** Implemented for FR-1 through FR-12; FR-13 through FR-16
have spec entries here but partial coverage in `docs/` and `examples/`.

## 8. Interface specifications

### 8.1 Standard input

* **Encoding:** UTF-8. ASCII-clean input is the common case.
* **Line terminator:** `\n` or `\r\n`. Stripped by the reader.
* **No record separator** other than line breaks. A line may contain
  whitespace within it (handled by the parser); leading/trailing
  whitespace is not stripped before parsing.
* **EOF** terminates the binary.

### 8.2 Standard output

* **Encoding:** UTF-8. In practice, ASCII.
* **Format:** zero or more lines, each `NNN NNN NNNN\n`.
* **Order:** input order of the corresponding valid lines.

### 8.3 Standard error

* **Encoding:** UTF-8.
* **Format:** zero or more diagnostic lines, each beginning with
  `Error ` and ending with `\n`. Additional log lines from
  `env_logger` may be interleaved when `RUST_LOG` selects them; log
  lines do not begin with `Error `.
* **Order:** input order of the corresponding bad lines.

### 8.4 Command-line arguments

See FR-13 for the canonical flag table. The clap `Command` is built in
[`src/app/clap.rs`](./src/app/clap.rs); its `--help` output is the
authoritative human-readable summary.

### 8.5 Environment variables

| Variable     | Effect                                                       |
| ------------ | ------------------------------------------------------------ |
| `RUST_LOG`   | `env_logger` filter expression. Controls log verbosity.       |
| `NO_COLOR`   | Honoured by `env_logger` if a future colour mode is enabled.  |

The binary defines no environment variables of its own.

### 8.6 Configuration file

Loaded via `confy` from the OS-appropriate location (`~/.config/nhs-number-cli/`
on Linux, etc.). Schema is `Config` in
[`src/app/config.rs`](./src/app/config.rs). At v0.2.0 the only field is
`version: u8 = 1`; future fields will be additive only.

### 8.7 Exit codes

See FR-12.

## 9. Data specifications

### 9.1 NHS Number

* Ten digits, conventionally `NNN NNN NNNN`.
* Tenth digit is a Modulus 11 check digit over the first nine
  (weights `10..=2`).
* If the Modulus-11 result is 10, the number is invalid (no legal NHS
  Number maps to that residue).
* Synthetic test range: `999 000 0000`–`999 999 9999`.

See [`docs/about-nhs-numbers/index.md`](./docs/about-nhs-numbers/index.md).

### 9.2 Error record schema (stderr)

| Field           | Type                | Source                                            |
| --------------- | ------------------- | ------------------------------------------------- |
| Prefix          | literal `Error `    | Every diagnostic line.                            |
| Kind            | `CheckDigitError` \| `ParseError` \| `IOError` | Internal enum variant. |
| `line_number`   | i32 (zero-based)    | `Iterator::enumerate` over stdin.                 |
| Payload         | NHS Number \| line text \| io::Error | Depends on Kind.                |

## 10. Test specifications

### 10.1 Test layers

| Layer                | Lives in                                  | Covers                                  |
| -------------------- | ----------------------------------------- | --------------------------------------- |
| Unit                 | `#[cfg(test)] mod tests` in `.rs` files   | Pure logic.                             |
| In-crate integration | `src/app/clap.rs` `#[cfg(test)]` block    | CLI parsing of the compiled binary.     |
| Crate integration    | `tests/test.rs`                           | End-to-end stdin→stdout/stderr.         |
| Runnable examples    | `examples/NN-<slug>/run.sh`               | Living documentation + regression bar.  |

See [`AGENTS/testing.md`](./AGENTS/testing.md).

### 10.2 Acceptance for "Implemented" status

A requirement may carry `Status: Implemented` only if **all** of the
following hold:

1. A test exists in at least the crate-integration or in-crate
   integration layer that exercises the behaviour.
2. The behaviour appears in the relevant `docs/` topic page.
3. If the behaviour is observable per input line, a runnable example
   under `examples/` demonstrates it.

## 11. Known gaps and TODOs

These are deliberate divergences between the spec and the code at the
time of writing. Each is tracked toward closure.

* **NFR-7 dependency audit.** `regex`, `glob`, `walkdir`, `strsim`,
  `rhai`, `serde_yaml`, `toml` are reserved for forthcoming features.
  Before v0.3.0, run `cargo machete` and remove any that no
  subcommand uses.
* **NFR-10 documentation parity.** FR-13–FR-16 have partial coverage
  in `docs/` and `examples/`. Track in the next docs PR.
* **CI.** No `.github/workflows/ci.yml` is currently committed. Before
  v0.3.0, add one that runs `cargo fmt`, `cargo clippy -- -D warnings`,
  `cargo test`, and `./examples/run-all.sh` on Linux, macOS, and
  Windows.

## 12. Change management

* **Adding a requirement:** append at the end of the relevant section
  with the next free ID. Do not renumber existing IDs.
* **Changing a requirement:** edit in place; bump the document version
  header; note the change in release notes; bump the crate version per
  the rules in [`AGENTS/commit-and-pr.md`](./AGENTS/commit-and-pr.md).
* **Removing a requirement:** mark `Status: Deprecated in vX.Y.Z` and
  leave the entry in place for one major version, then delete.
* **Status transitions:** `Planned` → `In progress` → `Implemented`,
  and optionally to `Deprecated in vX.Y.Z` or `Changed in vX.Y.Z`.

## 13. Traceability matrix

| Requirement | Code                                            | Tests                                     | Examples                              | Docs                                                          |
| ----------- | ----------------------------------------------- | ----------------------------------------- | ------------------------------------- | ------------------------------------------------------------- |
| FR-1        | `src/subcommands/check_lines.rs`                | `tests/test.rs`                           | `examples/01-basic/`                  | `docs/usage/index.md`                                         |
| FR-2        | `check_lines.rs` (blank-line guard)             | (TBD)                                     | `examples/04-blank-lines/`            | `docs/usage/index.md`                                         |
| FR-3        | `check_lines.rs` (`NHSNumber::from_str`)        | `tests/test.rs`                           | `examples/03-mixed-formats/`          | `docs/about-nhs-numbers/index.md`                             |
| FR-4        | `check_lines.rs` (`validate_check_digit`)       | `tests/test.rs`                           | `examples/01-basic/`                  | `docs/about-nhs-numbers/index.md`                             |
| FR-5        | `check_lines.rs` (`println!`)                   | `tests/test.rs`                           | `examples/02-valid-only/`             | `docs/usage/index.md`                                         |
| FR-6        | `check_lines.rs` (`Error::CheckDigitError`)     | `tests/test.rs`                           | `examples/01-basic/`                  | `docs/troubleshooting/index.md`                               |
| FR-7        | `check_lines.rs` (`Error::ParseError`)          | (TBD)                                     | `examples/10-parse-errors/`           | `docs/troubleshooting/index.md`                               |
| FR-8        | `check_lines.rs` (loop continues)               | `tests/test.rs`                           | `examples/01-basic/`                  | `docs/usage/index.md`                                         |
| FR-9        | `check_lines.rs` (`Error::IOError`)             | (TBD)                                     | (TBD)                                 | `docs/troubleshooting/index.md`                               |
| FR-10       | `check_lines.rs` (`#[error("…")]`)              | `tests/test.rs`                           | `examples/01-basic/expected-stderr.txt` | `AGENTS/behavioural-contract.md`                            |
| FR-11       | (stdlib I/O)                                    | (TBD)                                     | `examples/05-separate-streams/`       | `docs/usage/index.md`                                         |
| FR-12       | `src/main.rs`                                   | (TBD)                                     | `examples/07-fail-on-invalid/`        | `docs/usage/index.md`                                         |
| FR-13       | `src/app/clap.rs`, `src/app/args.rs`            | `src/app/clap.rs` `#[cfg(test)]`          | (TBD)                                 | `docs/usage/index.md`                                         |
| FR-14       | `src/app/config.rs`, `src/app/confy.rs`         | `src/app/confy.rs` `#[cfg(test)]`         | (TBD)                                 | (TBD)                                                         |
| FR-15       | `src/main.rs` (`env_logger::init`)              | (TBD)                                     | (TBD)                                 | `AGENTS/coding-style.md`                                      |
| FR-16       | `src/app/run.rs`, `src/subcommands/mod.rs`      | (TBD)                                     | all                                   | `AGENTS/architecture.md`                                      |
| NFR-1       | `BufRead::lines` iterator                       | (constant-memory not currently asserted)  | n/a                                   | `docs/architecture/index.md`                                  |
| NFR-2       | `Cargo.toml` (no platform-specific deps)        | manual cross-build                        | n/a                                   | `docs/installation/index.md`                                  |
| NFR-3       | overall design                                  | `tests/test.rs`                           | `examples/06-csv-column/`             | `docs/usage/index.md`                                         |
| NFR-4       | absence of network/persistence code             | code review                               | n/a                                   | `AGENTS/avoid.md`                                             |
| NFR-5       | `examples/README.md`                            | code review                               | all                                   | `AGENTS/avoid.md`                                             |
| NFR-6       | `Cargo.toml` `license`                          | `cargo license` (manual)                  | n/a                                   | `AGENTS/dependencies.md`                                      |
| NFR-7       | `Cargo.toml`                                    | `cargo machete` (manual)                  | n/a                                   | `AGENTS/dependencies.md`                                      |
| NFR-8       | absence of clock/random/hash on output path     | (TBD: byte-for-byte fixture)              | all                                   | `AGENTS/behavioural-contract.md`                              |
| NFR-9       | `edition = "2024"`                              | build                                     | n/a                                   | `docs/installation/index.md`, `docs/development/index.md`     |
| NFR-10      | this file                                       | n/a                                       | n/a                                   | every doc                                                     |

---

<!-- cSpell:ignore CARGO confy clap crates iechyd machete Modulus thiserror walkdir Rhai strsim PowerShell PDS subcommand subcommands -->
