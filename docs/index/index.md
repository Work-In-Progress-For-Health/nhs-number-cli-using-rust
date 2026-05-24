# Documentation

Welcome to the `nhs-number-cli` documentation.

`nhs-number-cli` is a small Unix-style filter that reads lines from
standard input, interprets each line as an NHS Number, and validates it.
Valid numbers are echoed to standard output; invalid or unparseable lines
are reported on standard error.

## Table of contents

- [About NHS Numbers](../about-nhs-numbers) — what an NHS Number is, how
  the check digit works, and what "valid" means.
- [Installation](../installation) — install from source, install from
  crates.io, and use the prebuilt Windows binary.
- [Usage](../usage) — the complete command reference, input formats,
  exit codes, and stream semantics.
- [Line validation](../line-validation) — the default subcommand: the
  stream-validator behaviour.
- [Examples](../examples) — cookbook-style recipes. Longer working
  examples live under the top-level [`examples/`](../../examples/)
  directory.
- [Troubleshooting](../troubleshooting) — common error messages and how
  to resolve them.
- [Architecture](../architecture) — how the program is put together and
  how it depends on the `nhs-number` crate.
- [Development](../development) — how to build, test, release, and
  contribute.
- [FAQ](../faq) — frequently asked questions.

## Quick start

```sh
# Build
cargo build --release

# Validate a file of NHS Numbers
cat input.txt | ./target/release/nhs-number-cli --line-validation
```

See [Usage](../usage) for the full reference.

## For contributors and agents

- [`AGENTS.md`](../../AGENTS.md) — conventions for AI coding agents.
- [`AGENTS/`](../../AGENTS/index.md) — topical agent guidance.
- [`spec.md`](../../spec.md) — living functional and non-functional
  specifications.

## Safety and privacy

NHS Numbers are patient identifiers. **Never** paste a real NHS Number
into a bug report, test fixture, or public example. Use the synthetic
test numbers documented in
[`examples/README.md`](../../examples/README.md#about-the-test-numbers).
