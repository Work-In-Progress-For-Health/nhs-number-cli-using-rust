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
git clone https://github.com/GIG-Cymru-NHS-Wales/nhs-number-cli-using-rust.git
cd nhs-number-cli-using-rust
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
