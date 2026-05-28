# Changelog

All notable changes to `nhs-number-cli` are recorded here. The format
follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and
the project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

The numbered requirements referenced below (FR-…, NFR-…) live in
[`spec.md`](./spec.md). Work items (WI-…) are tracked in `spec.md` § 15.

## [Unreleased]

### Fixed

- `src/app/clap.rs`: bare URL `https://docs.rs/clap/` in module doc
  comment wrapped in angle brackets so `cargo +nightly rustdoc`
  produces zero warnings. `llms.txt` and `llms.json` regenerated.
- `src/app/clap.rs`: two stale doc comments on `test_verbose` and
  `test_check_lines` (left over from a template clone) rewritten to
  describe what the tests actually prove.

### Removed

- `src/app/run.rs`: empty `test_run` placeholder with a bare `//TODO`
  marker. Violated AGENTS/coding-style.md's "no TODO hints without an
  issue number" rule and added noise to `cargo test` output.

### Changed

- `spec.md` § 13: NFR-8 traceability cell now explicitly names the
  `examples/*/run.sh` byte-for-byte `diff -u` fixtures as the
  determinism gate (previously `(TBD: byte-for-byte fixture)`).

## [v0.4.0] — 2026-05-26

Three new feature flags and the infrastructure that supports them.
The no-flag default behaviour is byte-for-byte unchanged, so existing
callers do nothing and keep working. All planned work items from the
0.3.x cycle's roadmap (WI-1 through WI-11) shipped in this release.

### Added

- `--counts` / `-c` — summary subcommand. Emits a four-row summary
  (`valid`, `invalid`, `parse-error`, `blank`) on stdout instead of
  running the per-line filter. Mutually exclusive with
  `--line-validation` at the clap layer. (FR-17, WI-7)
- `--column N` — 1-based CSV-field selector. Splits each non-blank
  line on the literal byte `,` and treats the *N*-th field as the
  candidate NHS Number. Honoured by both subcommands. Header rows
  are not skipped automatically; pipe through `tail -n +2`. Quoted
  CSV is out of scope. (FR-18, WI-8)
- `--format text|json|tsv` — output format selector for diagnostics
  (line-validation, on stderr) and the counts summary (`--counts`,
  on stdout). `text` is the default and preserves the FR-10 stable
  contract. JSON is hand-rolled (no `serde_json` dependency per
  NFR-7). (FR-19, WI-9)
- `.github/workflows/ci.yml` — GitHub Actions CI gating every push
  and PR on `cargo fmt -- --check`, `cargo clippy --all-targets
  -- -D warnings`, `cargo test`, and `./examples/run-all.sh` over
  `ubuntu-latest`, `macos-latest`, and `windows-latest`. (WI-1)
- `.gitattributes` — keeps shell scripts and example fixtures LF on
  Windows checkouts, with an explicit `-text` override on the
  CRLF-bearing `examples/08-crlf-windows/input-crlf.txt`. (WI-11)
- Crate-integration tests for FR-2, FR-7, FR-9, FR-11, FR-12,
  FR-17, FR-18, FR-19 in `tests/test.rs`. The FR-9 case feeds
  invalid UTF-8 to exercise a real I/O error path. (WI-6)
- Runnable examples `examples/11-flag-demo/` (every public flag +
  `RUST_LOG`) and `examples/12-format-output/` (`--format json` and
  `--format tsv`). (WI-4, FR-19)

### Changed

- Examples are now Windows-compatible: each `run.sh` retries
  `target/debug/nhs-number-cli` with a `.exe` suffix when the bare
  path is not executable. CI runs `./examples/run-all.sh` on every
  matrix OS, not just Linux/macOS. (WI-11)
- `Error::*::line_number` widened from `i32` to a new
  `pub type LineIndex = usize` alias, matching `Iterator::enumerate`.
  Drops three `as i32` casts. User-visible stderr bytes unchanged.
  (WI-5)
- `cargo fmt` applied across the whole tree as a prerequisite for
  the CI `fmt --check` gate. No behavioural changes.
- `examples/09-counts-summary/` switched from a hand-rolled
  `summary.sh` wrapper to the built-in `--counts` flag.
- `examples/06-csv-column/` switched from `cut -d, -f3 | tail -n +2`
  to `tail -n +2 | nhs-number-cli --column 3`.

### Removed

- Seven speculative crates that had no in-source reference:
  `regex`, `glob`, `walkdir`, `strsim`, `rhai`, `serde_yaml`,
  `toml`. `Cargo.lock` shrinks by ~380 lines. (NFR-7, WI-3 audit)
- Duplicate `src/app/testing.rs` test-helper module. The single
  source is now `src/testing.rs`. (WI-2)
- `examples/09-counts-summary/summary.sh` (superseded by `--counts`).

### Fixed

- Mathematically-invalid documented NHS Numbers (`999 000 0000`
  and `999 123 4560`, both yield Modulus-11 result 10 → no legal
  check digit) replaced project-wide with `999 999 9999` and
  `999 000 0069`. Required after `nhs-number` 1.0.1 corrected the
  upstream check-digit bug that previously accepted them.
- Per the behavioural contract (FR-10) every `stderr` diagnostic
  line begins with `Error ` — restored after the modular refactor
  briefly emitted `CheckDigitError ➡ …`.
- `subcommands::check_lines` was previously unreachable due to a
  `crate::subcommand::…` (singular) typo in `dispatch`; renamed to
  `crate::subcommands::…` and the binary now actually builds.
- `tests/test.rs`'s "valid" fixture was the mathematically-invalid
  `999 123 4560`; swapped to `999 999 9999`.
- `src/testing.rs` had a stray `"sita"` path from another project's
  template; corrected to `"nhs-number-cli"`.

### Documentation

- `spec.md` extended to cover FR-13 through FR-19 plus a full
  decision log; traceability matrix updated row-by-row.
- New top-level `AGENTS.md` plus topical files under `AGENTS/`
  (architecture, coding-style, testing, behavioural-contract,
  dependencies, commit-and-pr, avoid, index).
- `docs/development/index.md` documents how to regenerate
  `llms.txt` / `llms.json` from a nightly toolchain. (WI-3)
- Top-level `index.md` sitemap.

## [0.3.0] — 2026-05-25

### Changed

- Scope and authorship narrowed for open-source publication.
  Repository moved to `joelparkerhenderson/nhs-number-cli`; default
  description focuses on NHS England and the NHS Isle of Man.
- Modular refactor: monolithic `src/main.rs` split into
  `src/app/{args, clap, config, confy, run}.rs`,
  `src/subcommands/check_lines.rs`, and `src/types/{list, map,
  queue, set, stack}.rs`. The binary is now a `clap`-driven CLI
  with `--line-validation`, `--test`, and `--verbose` flags. The
  default behaviour (no flag) is unchanged.

## [0.2.0] — 2026-04-24

Initial public-facing release.

### Added

- Line-validation filter: read NHS Numbers from stdin, write valid
  numbers to stdout in canonical `NNN NNN NNNN` form, write
  diagnostics for invalid lines to stderr. (FR-1 through FR-12)
- Cross-compiled Windows GNU binary tracked at
  `target/x86_64-pc-windows-gnu/release/nhs-number-cli.exe`.
- Initial `docs/`, `examples/`, `tests/` layout.

[Unreleased]: https://github.com/joelparkerhenderson/nhs-number-cli/compare/v0.4.0...HEAD
[v0.4.0]: https://github.com/joelparkerhenderson/nhs-number-cli/compare/0.3.0...v0.4.0
[0.3.0]: https://github.com/joelparkerhenderson/nhs-number-cli/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/joelparkerhenderson/nhs-number-cli/releases/tag/0.2.0
