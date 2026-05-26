# Testing

How tests are organised, what each layer covers, and how to add a new
one. See [`coding-style.md`](./coding-style.md) for the syntax-level
conventions; this file is the strategy. The numbered requirements that
each layer is proving live in [`../spec.md`](../spec.md).

## Test taxonomy

| Layer                | Lives in                                                   | Drives               | Purpose                                              |
| -------------------- | ---------------------------------------------------------- | -------------------- | ---------------------------------------------------- |
| Unit                 | `#[cfg(test)] mod tests` blocks inside the file under test | In-process functions | Verify pure logic (types, macros, parsing helpers).  |
| In-crate integration | `#[cfg(test)] mod tests` in `src/app/clap.rs` etc.         | The compiled binary  | Verify CLI parsing, exit, and `--test` output.       |
| Crate integration    | `tests/test.rs`                                            | The compiled binary  | Verify the end-to-end stdin → stdout/stderr contract.|
| Runnable examples    | `examples/NN-<slug>/run.sh`                                | The compiled binary  | Living documentation + regression bar.               |

All four layers run from `cargo test` plus `./examples/run-all.sh`. A
change that adds an observable behaviour should produce a new entry in
at least the **crate integration** layer and the **runnable examples**
layer (see `spec.md` § NFR-10).

## Running tests

```sh
cargo build              # required: the binary must exist at target/debug/
cargo test               # unit + in-crate integration + tests/test.rs
./examples/run-all.sh    # all runnable examples
```

`cargo test` builds the binary automatically before running tests that
spawn it, but the path `target/debug/nhs-number-cli` is hardcoded in
several places. Do not change the binary name without updating every
hardcoded reference (search `COMMAND_OS` and `nhs-number-cli`
literals).

## Test helpers

There is **one** in-crate test-helper module: `src/testing.rs`. It is
`#[cfg(test)]`-gated by `mod testing;` in `src/main.rs`, and exposes
`LazyLock` paths to the compiled binary
(`COMMAND_OS`, `COMMAND_FILE`) and to the project's standard
directories (`CARGO_MANIFEST_DIR`, `LOG_DIR`, `TESTS_DIR`, `TMP_DIR`,
`TARGET_DIR`, `DEBUG_DIR`).

**Convention:** every `#[cfg(test)] mod tests { … }` block in `src/`
imports via `use crate::testing::*;`. Do not add a parallel
helper module under `src/app/` or elsewhere; if you find a third copy
appearing, delete it and route callers through `crate::testing`.

`tests/test.rs` keeps its own copy of `COMMAND_OS` because it is a
separate crate-integration binary and cannot reach `crate::testing`
across the crate boundary. If the path expression there ever drifts,
harmonise it to match `src/testing.rs`.

## Patterns

### Spawning the compiled binary

```rust
use crate::testing::COMMAND_OS;
use assertables::*;

#[test]
fn rejects_bad_check_digit() {
    assert_program_args_stdout_string_contains!(
        &*COMMAND_OS,
        &["--line-validation"],
        "999 999 9999",
    );
}
```

`assertables` macros wrap `std::process::Command` so the assertion
failure message includes the program, args, and observed output.

### Piping stdin

For tests that need to write to the spawned process's stdin (the
typical case for the line-validation filter), build the `Command` by
hand. See `tests/test.rs` for the canonical example:
`.stdin(Stdio::piped())`, `stdin.write_all(...)`, then
`wait_with_output()`.

### Test data

Use synthetic NHS Numbers from the `999 000 0000`–`999 999 9999`
range. The canonical pair used by the project-wide tests is:

* **Valid:** `999 999 9999`
* **Invalid (fails check digit):** `999 123 4561`

A longer curated list lives in
[`../examples/README.md`](../examples/README.md#about-the-test-numbers).

**Never** put a real or potentially-real NHS Number into a fixture,
test input, or assertion message — even as a placeholder, even in a
comment. See `spec.md` § NFR-5.

## Writing a new test

1. **Pick the right layer.** If the change is pure logic (a parser, a
   helper), write a unit test in the same file. If the change alters
   stdin/stdout/stderr behaviour, write a crate-integration test plus
   a runnable example.
2. **Use synthetic data.** See above.
3. **Assert on observable output, not internals.** Even unit tests
   should assert what a user could observe (return value, formatted
   string), not internal state.
4. **Update [`../spec.md`](../spec.md).** Every new acceptance test
   should trace back to a numbered FR or NFR. Add the requirement (and
   its traceability block) if it does not yet exist.

## Adding a runnable example

Examples are both documentation and tests. Each example is a directory
under `examples/`:

```
examples/NN-<slug>/
├── README.md             What the example demonstrates.
├── input.txt             Bytes piped into the binary on stdin.
├── expected-stdout.txt   Exact stdout bytes the test expects.
├── expected-stderr.txt   Exact stderr bytes the test expects.
└── run.sh                Runs the binary; diffs observed vs expected.
```

`run.sh` should exit 0 on match, non-zero on diff. The orchestrator
[`../examples/run-all.sh`](../examples/run-all.sh) runs every example
and collects exit codes.

## CI

GitHub Actions runs the same pre-commit checks on every push and PR.
The workflow is [`.github/workflows/ci.yml`](../.github/workflows/ci.yml);
it matrices over `ubuntu-latest`, `macos-latest`, and `windows-latest`
and runs:

* `cargo fmt -- --check`
* `cargo clippy --all-targets -- -D warnings`
* `cargo test`
* `./examples/run-all.sh` (under `shell: bash` on every OS — Git
  Bash on Windows runners; each `run.sh` retries `target/debug/
  nhs-number-cli` with a `.exe` suffix; `.gitattributes` keeps the
  scripts and fixtures LF-only on Windows checkouts).

Run the same four commands locally before pushing — relying on CI
to find drift slows reviews.

<!-- cSpell:ignore assertables LazyLock -->
