# NHS Number command line interface

A National Health Service (NHS) Number is a unique ten-digit identifier
allocated to registered patients of NHS England and the NHS Isle of Man.

`nhs-number-cli` is a Unix-style filter that reads lines from standard
input, validates each one as an NHS Number, and routes it:

* Valid NHS Numbers are printed to standard output in canonical
  `NNN NNN NNNN` form.
* Invalid or unparseable lines produce a diagnostic on standard error,
  beginning with the stable `Error ` prefix.
* Blank lines are silently skipped.
* The exit code is `0` on EOF, regardless of how many lines were
  invalid.

References:

* [National Health Service (NHS)](https://en.wikipedia.org/wiki/National_Health_Service)
* [NHS Number](https://en.wikipedia.org/wiki/NHS_number)

## Quick example

Suppose `input.txt` contains one NHS Number per line, valid or invalid:

```txt
999 999 9999
999 123 4561
```

Linux or macOS:

```sh
cat input.txt | nhs-number-cli
```

Windows:

```sh
type input.txt | nhs-number-cli.exe
```

The command prints the valid NHS Number to stdout:

```
999 999 9999
```

and the invalid one to stderr:

```
Error invalid line 1. Error: validate check digit failed. NHS Number: 999 123 4561
```

## Flags

| Flag                       | Effect                                                                |
| -------------------------- | --------------------------------------------------------------------- |
| `-l`, `--line-validation`  | Run the per-line filter explicitly. Same effect as no flag.           |
| `-c`, `--counts`           | Emit a four-row summary (`valid`, `invalid`, `parse-error`, `blank`) on stdout instead of filtering. |
| `--column N`               | Treat each line as a comma-separated row and pick the *N*-th (1-based) field as the candidate. |
| `--format text\|json\|tsv` | Output format for diagnostics and the counts summary. `text` is the default and the stable contract. |
| `-v…`, `--verbose…`        | Increase log verbosity. Count → level: 1=error, …, 5=trace.           |
| `-V`, `--version`          | Print the crate version and exit.                                     |
| `-h`, `--help`             | Print help and exit.                                                  |

See [`docs/usage/`](./docs/usage/index.md) for full semantics.

## Releases

Build a release for your own platform:

```sh
cargo build --release
```

A prebuilt Windows GNU binary is tracked in this repository at
[`target/x86_64-pc-windows-gnu/release/nhs-number-cli.exe`](./target/x86_64-pc-windows-gnu/release/nhs-number-cli.exe).
Additional prebuilt targets will follow as they're requested.

Version history is recorded in [`CHANGELOG.md`](./CHANGELOG.md).

## Documentation

Full documentation lives under [`docs/`](./docs/):

* [About NHS Numbers](./docs/about-nhs-numbers/index.md)
* [Installation](./docs/installation/index.md)
* [Usage](./docs/usage/index.md)
* [Line validation](./docs/line-validation/index.md)
* [Examples](./docs/examples/index.md)
* [Troubleshooting](./docs/troubleshooting/index.md)
* [Architecture](./docs/architecture/index.md)
* [Development](./docs/development/index.md)
* [FAQ](./docs/faq/index.md)

Runnable examples with expected-output fixtures and `run.sh` scripts
live under [`examples/`](./examples/). Run them all with:

```sh
cargo build
./examples/run-all.sh
```

## For contributors

* [`CONTRIBUTING.md`](./CONTRIBUTING.md) — how to propose a change.
* [`CODE_OF_CONDUCT.md`](./CODE_OF_CONDUCT.md) — community guidelines.
* [`spec.md`](./spec.md) — living functional and non-functional
  specifications. The source of truth for what the binary must do.

## For AI coding agents

See [`AGENTS.md`](./AGENTS.md) for conventions, the behavioural
contract, and things to avoid when modifying this repository with the
help of AI tools. Topical guidance lives under [`AGENTS/`](./AGENTS/).
