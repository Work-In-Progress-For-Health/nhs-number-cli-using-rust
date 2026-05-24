# Documentation

Welcome to the `nhs-number-cli` documentation.

`nhs-number-cli` is a small Unix-style filter that reads lines from
standard input, interprets each line as an NHS Number, and validates it.
Valid numbers are echoed to standard output; invalid or unparseable lines
are reported on standard error.

## Table of contents

1. [About NHS Numbers](./about-nhs-numbers/index.md) — what an NHS Number
   is, how the check digit works, and what "valid" means.
2. [Installation](./installation/index.md) — install from source, install
   from crates.io, and use the prebuilt Windows binary.
3. [Usage](./usage/index.md) — the complete command reference, input
   formats, exit codes, and stream semantics.
4. [Line validation](./line-validation/index.md) — the default
   subcommand: the stream-validator behaviour.
5. [Examples](./examples/index.md) — cookbook-style recipes. Longer
   working examples live under the top-level [`examples/`](../examples/)
   directory.
6. [Troubleshooting](./troubleshooting/index.md) — common error messages
   and how to resolve them.
7. [Architecture](./architecture/index.md) — how the program is put
   together and how it depends on the `nhs-number` crate.
8. [Development](./development/index.md) — how to build, test, release,
   and contribute.
9. [FAQ](./faq/index.md) — frequently asked questions.

## Quick start

```sh
# Build
cargo build --release

# Validate a file of NHS Numbers
cat input.txt | ./target/release/nhs-number-cli --line-validation
```

See [Usage](./usage/index.md) for the full reference.

## For contributors and agents

* [`../AGENTS.md`](../AGENTS.md) — conventions for AI coding agents.
* [`../AGENTS/`](../AGENTS/index.md) — topical agent guidance.
* [`../spec.md`](../spec.md) — living functional and non-functional
  specifications.

## Safety and privacy

NHS Numbers are patient identifiers. **Never** paste a real NHS Number
into a bug report, test fixture, or public example. Use the synthetic
test numbers documented in
[`examples/README.md`](../examples/README.md#about-the-test-numbers).
