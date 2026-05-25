# AGENTS/ index

Topical guidance for AI coding agents working in this repository. The
entry point is [`../AGENTS.md`](../AGENTS.md); the files below expand
each section.

| File                                                   | Read when you are…                                  |
| ------------------------------------------------------ | --------------------------------------------------- |
| [`architecture.md`](./architecture.md)                 | Choosing where new code belongs.                    |
| [`coding-style.md`](./coding-style.md)                 | Writing Rust in this repo.                          |
| [`testing.md`](./testing.md)                           | Adding or running tests.                            |
| [`behavioural-contract.md`](./behavioural-contract.md) | Touching anything that prints to stdout / stderr.   |
| [`dependencies.md`](./dependencies.md)                 | Adding, upgrading, or removing a crate.             |
| [`commit-and-pr.md`](./commit-and-pr.md)               | Committing, branching, releasing.                   |
| [`avoid.md`](./avoid.md)                               | Tempted to do anything fancy.                       |

See also:

* [`../spec.md`](../spec.md) — living functional and non-functional
  specifications. The source of truth for *what* the binary must do.
* [`../docs/index.md`](../docs/index.md) — end-user documentation.
* [`../examples/README.md`](../examples/README.md) — runnable examples.
* [`../README.md`](../README.md) — the project's public README.

## How these files relate

`AGENTS.md` is the entry point. The topical files in this directory
expand each section in depth. `spec.md` is the cross-cutting source of
truth: every observable behaviour is numbered there and traced back to
code, tests, examples, and docs. When this directory disagrees with
`spec.md`, treat `spec.md` as authoritative and fix the AGENTS file.
