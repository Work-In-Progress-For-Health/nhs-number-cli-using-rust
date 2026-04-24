# Documentation

Welcome to the `nhs-number-cli` documentation.

`nhs-number-cli` is a small Unix-style filter that reads lines from standard
input, interprets each line as an NHS Number, and validates it. Valid
numbers are echoed to standard output; invalid or unparseable lines are
reported on standard error.

## Table of contents

- [FAQ](./faq) — frequently asked questions.
- [About NHS Numbers](./about-nhs-numbers) — what an NHS Number is, how
   the check digit works, and what "valid" means.
- [Installation](./installation) — install from source, install from
   crates.io, and use the prebuilt Windows binary.
- [Usage](./usage) — the complete command reference, input formats,
   exit codes, and stream semantics.
- [Examples](./examples) — cookbook-style recipes. Longer working
   examples live under the top-level [`examples/`](../examples/) directory.
- [Architecture](./architecture) — how the program is put together and
   how it depends on the `nhs-number` crate.
- [Development](./development) — how to build, test, release, and
   contribute.
- [Troubleshooting](./troubleshooting) — common error messages and how
   to resolve them.

## Quick start

```sh
# Build
cargo build --release

# Validate a file of NHS Numbers
cat input.txt | ./target/release/nhs-number-cli
```

See [Usage](./usage.md) for the full reference.

## Safety and privacy

NHS Numbers are patient identifiers. **Never** paste a real NHS Number into
a bug report, test fixture, or public example. Use the synthetic test
numbers documented in
[examples](../examples/README.md#about-the-test-numbers).
