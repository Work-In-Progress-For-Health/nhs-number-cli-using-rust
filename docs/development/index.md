# Development

## Toolchain

Install Rust via [rustup](https://rustup.rs/) and ensure the stable
toolchain is at least 1.85 (edition 2024):

```sh
rustup update stable
rustc --version
```

## Clone

```sh
git clone https://github.com/joelparkerhenderson/nhs-number-cli.git
cd nhs-number-cli
```

## Build

```sh
cargo build                # debug
cargo build --release      # release
```

## Test

```sh
cargo test
```

The integration test in `tests/test.rs` spawns the compiled debug binary
and pipes a small input through it. Cargo will build the binary before
running the test. See [Architecture § integration test](./architecture.md#integration-test).

## Lint and format

```sh
cargo fmt
cargo clippy -- -D warnings
```

CI (when added) should run both before merging.

## Cross-compiling for Windows

On macOS or Linux with the `mingw-w64` toolchain:

```sh
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

The resulting binary is committed to
`target/x86_64-pc-windows-gnu/release/nhs-number-cli.exe` for
convenience.

## Adding examples

New examples go under [`examples/`](../examples) at the repository root.
See [`examples/README.md`](../examples/README.md) for the layout. Each
example is a self-contained directory with its own `README.md`, an
`input.txt`, an `expected-stdout.txt`, and an `expected-stderr.txt`.

## Regenerating `llms.txt` and `llms.json`

The two top-level files [`llms.txt`](../../llms.txt) and
[`llms.json`](../../llms.json) are rustdoc snapshots aimed at LLM-style
consumers (and at human readers who want a single-file index of the
crate's docs). They are not hand-edited; regenerate them whenever
public-ish items, doc comments, or the crate version change.

Requirements:

* A nightly Rust toolchain (rustdoc's JSON output is unstable).
* The [`rustdoc-md`](https://crates.io/crates/rustdoc-md) tool:
  `cargo install rustdoc-md`.

Procedure:

```sh
# 1. Produce target/doc/nhs_number_cli.json.
cargo +nightly rustdoc -- --output-format json -Z unstable-options

# 2. Promote the JSON to the repo root.
cp target/doc/nhs_number_cli.json llms.json

# 3. Render Markdown from the JSON.
rustdoc-md --path llms.json --output llms.txt
```

Commit both files in the same change as the doc-comment or version
edit that prompted the regeneration. Do not edit them by hand.

## Releasing

1. Bump `version` in `Cargo.toml`.
2. Run `cargo test` and `cargo clippy -- -D warnings`.
3. Commit: `git commit -am "Release vX.Y.Z"`.
4. Tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`.
5. Push: `git push && git push --tags`.
6. (Optional) `cargo publish` to crates.io.

## Style

* Rust edition 2024.
* Four-space indentation (the Rust default).
* Follow `rustfmt` output verbatim; do not hand-format against it.
* Prefer `?` and `match` over `unwrap`.
* Keep the program's observable behaviour stable. See the "Behavioural
  contract" section of [`AGENTS.md`](../AGENTS.md).
