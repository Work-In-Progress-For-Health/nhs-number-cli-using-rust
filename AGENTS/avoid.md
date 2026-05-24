# Things to avoid

A short, prescriptive list. If you are about to do one of these,
**stop and ask first**.

## Behaviour and UX

* **No TUI, no interactive prompts.** The tool must compose with pipes
  and redirection. Reading from a `stdin` that is also a terminal is
  fine, but never block waiting for special input.
* **No colour or ANSI escapes on stdout.** Valid NHS Numbers go to
  stdout as plain ASCII; downstream tools consume them. Coloured stderr
  is acceptable if and only if it's disabled when stderr is not a TTY.
* **No progress bars or spinners.** Same reason.
* **No reading files from arguments.** The tool reads stdin. Callers use
  `< file` or `cat file |`.
* **No network access.** No HTTP, no DNS, no sockets. No "phone home",
  no update checks, no telemetry.
* **No swallowed errors.** Every input line either produces stdout, or
  produces exactly one stderr diagnostic, or is silently skipped (blank
  lines only). Never both, never neither.

## Data and privacy

* **No real NHS Numbers** in code, tests, fixtures, examples, commit
  messages, issue text, branch names, PR descriptions, agent prompts, or
  log lines. Use the synthetic range `999 000 0000`–`999 999 9999`.
* **No persistence of input lines.** Do not write input to disk, do not
  cache it, do not include it in crash reports.
* **No logging of input at info-or-higher levels.** The `debug!` /
  `trace!` macros are acceptable only under explicit `RUST_LOG` opt-in,
  and only with synthetic data.

## Architecture

* **No additional build systems.** Cargo only. No `make`, `just`, `npm`,
  `cmake`, `bazel`. Wrapping cargo in a shell script is fine.
* **No async runtimes** (tokio, async-std, smol). The tool is
  synchronous I/O over stdin.
* **No process forking or threading** in the main filter loop. A future
  subcommand may need threads; if so, gate it behind a flag.
* **No global mutable state.** Pass `Args` and `Config` down.
* **No re-implementation of `nhs-number`.** Parsing and check-digit
  validation live in the upstream crate. Reach for them, don't duplicate
  them.

## Code style

* **No `unwrap()` / `expect()` on the runtime path.** Acceptable inside
  `#[cfg(test)]` and never elsewhere.
* **No `println!` from `main` or `run`.** Only the `check_lines`
  subcommand prints to stdout, and only valid numbers.
* **No `eprintln!` on the success path.** Logging via `log::*` macros
  is the right tool for operational messages.
* **No `#[allow(dead_code)]` to silence warnings.** Either use the
  symbol or delete it. The exception is test-helper paths
  (`src/testing.rs`) which need it because not every test uses every
  helper.
* **No suppression of clippy warnings without a same-line justification.**

## Repository

* **No commit of `.DS_Store`** or other OS cruft. There's one tracked at
  `src/app/.DS_Store` that should be removed in a cleanup PR; do not
  add more.
* **No commit of `target/`** other than the deliberately-tracked
  Windows GNU binary.
* **No new top-level directories** without a one-line justification in
  the PR.

## Dependencies

* **No GPL-only crates.** See [`AGENTS/dependencies.md`](./dependencies.md).
* **No "kitchen-sink" crates** added speculatively. Each new dependency
  needs a concrete user story in the same PR.
* **No upgrade of `nhs-number`** outside a dedicated PR that runs every
  example and verifies output is byte-identical (or notes the diff in
  the release notes).

## Documentation

* **No `*.md` named after a transient task** (e.g. `FIX-456.md`,
  `MIGRATION-PLAN.md`). Use the issue tracker for transient context;
  the repo carries durable docs only.
* **No "we changed X for the Y flow" comments in code.** That belongs
  in the commit message and the PR description.
* **No duplicating `spec.md` content elsewhere.** Link to it.

<!-- cSpell:ignore bazel cmake confy crates smol -->
