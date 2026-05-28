# nhs-number-cli — project index

A small Rust command line tool that validates National Health Service
(NHS) Numbers from standard input. This page is a sitemap; for the
user guide start at [`docs/index.md`](./docs/index.md), and for the
behavioural specification start at [`spec.md`](./spec.md).

## For users

| Where                                                      | What                                          |
| ---------------------------------------------------------- | --------------------------------------------- |
| [`README.md`](./README.md)                                 | Quick overview and a worked example.          |
| [`docs/`](./docs/index.md)                                 | Full user documentation.                      |
| [`examples/`](./examples/README.md)                        | Runnable examples with expected output.       |
| [`docs/installation/`](./docs/installation/index.md)       | How to install.                               |
| [`docs/usage/`](./docs/usage/index.md)                     | Command reference, input formats, exit codes. |
| [`docs/line-validation/`](./docs/line-validation/index.md) | The default subcommand in depth.              |
| [`docs/troubleshooting/`](./docs/troubleshooting/index.md) | What error messages mean.                     |
| [`docs/faq/`](./docs/faq/index.md)                         | Frequently asked questions.                   |

## For contributors

| Where                                                | What                                       |
| ---------------------------------------------------- | ------------------------------------------ |
| [`CONTRIBUTING.md`](./CONTRIBUTING.md)               | How to propose a change.                   |
| [`CODE_OF_CONDUCT.md`](./CODE_OF_CONDUCT.md)         | Community guidelines.                      |
| [`docs/development/`](./docs/development/index.md)   | Build, test, lint, release.                |
| [`docs/architecture/`](./docs/architecture/index.md) | How the program is put together.           |
| [`spec.md`](./spec.md)                               | Living functional & non-functional specs.  |

## For AI coding agents

| Where                          | What                                            |
| ------------------------------ | ----------------------------------------------- |
| [`AGENTS.md`](./AGENTS.md)     | Entry point and non-negotiables.                |
| [`AGENTS/`](./AGENTS/index.md) | Topical conventions and rationale.              |
| [`spec.md`](./spec.md)         | What the program must do (the source of truth). |

## Repository metadata

| Where                              | What                                          |
| ---------------------------------- | --------------------------------------------- |
| [`Cargo.toml`](./Cargo.toml)       | Crate manifest. Pinned dependency versions.   |
| [`Cargo.lock`](./Cargo.lock)       | Locked transitive dependencies.               |
| [`CHANGELOG.md`](./CHANGELOG.md)   | What changed in each released version.        |
| [`CITATION.cff`](./CITATION.cff)   | How to cite this software.                    |
| [`cspell.json`](./cspell.json)     | Spell-check allow-list.                       |

## About the project

* **Status:** active, single-maintainer, small scope.
* **Licence:** `MIT OR Apache-2.0 OR GPL-2.0 OR GPL-3.0 OR BSD-3-Clause`.
* **Toolchain:** Rust edition 2024, MSRV 1.85.
* **Upstream domain crate:** [`nhs-number`](https://crates.io/crates/nhs-number).
* **Issues & releases:** <https://github.com/joelparkerhenderson/nhs-number-cli/issues>.

## Privacy note

NHS Numbers are patient identifiers. Use only the synthetic test range
`999 000 0000`–`999 999 9999` in any artefact stored in this
repository. See
[`examples/README.md`](./examples/README.md#about-the-test-numbers)
and [`spec.md`](./spec.md) § NFR-5.
