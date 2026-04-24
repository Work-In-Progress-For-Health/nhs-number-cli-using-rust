# Installation

There are three supported ways to install `nhs-number-cli`:

1. Build from source with Cargo (recommended).
2. Install from crates.io (once published).
3. Use a prebuilt binary from the `releases/` or `target/` directory.

## 1. Build from source

Prerequisites:

* [Rust toolchain](https://www.rust-lang.org/tools/install) 1.85 or newer
  (edition 2024 is required).
* A working C toolchain for linking. macOS and most Linux distributions
  have this already; on Windows use the MSVC or GNU toolchain that matches
  your Rust target.

Clone and build:

```sh
git clone https://github.com/GIG-Cymru-NHS-Wales/nhs-number-cli-using-rust.git
cd nhs-number-cli-using-rust
cargo build --release
```

The binary is placed at `target/release/nhs-number-cli` (or
`target/release/nhs-number-cli.exe` on Windows).

To put it on your `PATH`, copy or symlink it somewhere appropriate:

```sh
sudo install -m 0755 target/release/nhs-number-cli /usr/local/bin/
```

## 2. Install from crates.io

Once the crate is published to crates.io you can install it with:

```sh
cargo install nhs-number-cli
```

This puts the binary in `$CARGO_HOME/bin` (by default `~/.cargo/bin`),
which should already be on your `PATH` if you installed Rust via
`rustup`.

## 3. Prebuilt binaries

Prebuilt binaries, when available, live under `releases/` and inside
`target/<triple>/release/` in the repository. For example, a Windows GNU
build is tracked at:

```
target/x86_64-pc-windows-gnu/release/nhs-number-cli.exe
```

Copy the appropriate binary to a directory on your `PATH`. On Windows you
can drop it into a folder listed in `%PATH%`.

### Verify the installation

```sh
echo "999 123 4560" | nhs-number-cli
```

You should see `999 123 4560` echoed back on standard output.

## Uninstall

* If installed via `cargo install`, run `cargo uninstall nhs-number-cli`.
* If you copied the binary manually, delete it from the directory you
  placed it in.
