# Dependencies

Each entry in `Cargo.toml` should have a clear reason to exist. This file
records that reason, the surface area we actually use, and the alternatives
we considered. Add to it whenever you add, upgrade, or remove a crate.

## Licence policy

The crate is multi-licensed:

```
MIT OR Apache-2.0 OR GPL-2.0 OR GPL-3.0 OR BSD-3-Clause
```

Any new dependency must be **compatible with the intersection** of these
licences. In practice this means `MIT OR Apache-2.0` is always safe; pure
GPL-only dependencies are not acceptable.

Run a licence check before adding a dependency:

```sh
cargo install cargo-license   # one-time
cargo license
```

## Current dependencies

### Runtime

| Crate         | Why it's here                                                       |
| ------------- | ------------------------------------------------------------------- |
| `nhs-number`  | Parses NHS Numbers, validates Modulus 11 check digit, formats canonical `NNN NNN NNNN`. **Domain core.** Do not duplicate its logic in this crate. |
| `clap`        | Command line argument parsing. Features `string` and `wrap_help` enabled for cleaner help output. |
| `confy`       | Boilerplate-free TOML configuration loading at the OS-appropriate path. The CLI itself does not yet *require* persisted config, but the wiring is in place for forthcoming subcommands. |
| `serde`       | Derives `Serialize`/`Deserialize` on `Config`. Required by `confy`. |
| `thiserror`   | Derives `std::error::Error` on per-module `Error` enums. The user-visible diagnostic format flows out of `#[error("…")]`; treat it as observable. |
| `log`         | Logging facade. Used through macros (`trace!`, `debug!`, …). |
| `env_logger`  | Reads `RUST_LOG` and routes `log` macros to stderr. |

### Build / test only

| Crate         | Why it's here                                                       |
| ------------- | ------------------------------------------------------------------- |
| `assertables` | Process-aware assertion macros used in `#[cfg(test)]` blocks. Imported via `extern crate` from `main.rs` so the macros are available crate-wide for test discovery. (Listed as a runtime dependency in `Cargo.toml` because the `extern crate` is not gated, but its only use is in `#[cfg(test)]` blocks.) |

Other test helpers live in-crate (`src/testing.rs`); they are not crates.

### Previously included, now removed

The crates below were added speculatively for features that have not
materialised. They were removed in the NFR-7 audit (commit landing this
file). Re-add only with a concrete user story in the same PR:

| Crate        | Removed because                                                      |
| ------------ | -------------------------------------------------------------------- |
| `regex`      | No in-source reference. Reach for `.split_whitespace()` or the upstream `nhs-number` parser first. |
| `glob`       | No in-source reference; the binary reads `stdin`, not files.         |
| `walkdir`    | No in-source reference; the binary reads `stdin`, not directories.   |
| `strsim`     | No in-source reference; "did you mean…" hints are not yet a feature. |
| `rhai`       | No in-source reference; embedded scripting added the heaviest transitive closure of any candidate. |
| `serde_yaml` | No in-source reference; no subcommand emits YAML.                    |
| `toml`       | No in-source reference; `confy` handles TOML load/save on its own.   |

## Audit candidates

A few crates were added in anticipation of features that may or may not
ship. Before each release:

1. Run `cargo +nightly udeps` (or `cargo machete`) to find unused
   dependencies.
2. For each crate marked **Audit candidate** above, check whether any
   subcommand has started using it.
3. If a crate is still unused and no concrete user story exists for the
   feature it was reserved for, remove it. Smaller dependency closure =
   faster build, smaller binary, smaller attack surface.

## Adding a dependency

1. Search the existing closure first (`cargo tree`). If a dependency
   already exists transitively that does what you need, prefer it.
2. Confirm the licence is compatible (see above).
3. Confirm the crate is actively maintained: a release within the last
   twelve months and an issue response within the last six.
4. Pin to a minor version (`^X.Y`), not a major (`*` or `X`).
5. Enable only the features you need. `default-features = false` plus
   explicit `features = ["…"]` is preferred when defaults pull in extras.
6. Document the entry in this file in the same commit.

## Upgrading a dependency

* Patch upgrades (`X.Y.Z → X.Y.Z+1`) are non-breaking by SemVer; review
  the changelog anyway.
* Minor upgrades (`X.Y.Z → X.Y+1.0`): review the changelog for new
  features and any subtle behaviour changes. Run the full test suite and
  all examples.
* Major upgrades: open a PR titled `Upgrade <crate> to vX`. Run all
  examples; expect to update fixtures.

The `Cargo.lock` file is committed (this is a binary crate, not a
library). Upgrades go via `cargo update -p <crate>` so other crates stay
pinned.

## Pinning `nhs-number`

The domain crate is the only dependency whose behaviour can change the
*output bytes* of this binary. Pin it to a known-good version and bump
deliberately. A change in `nhs-number`'s `Display` impl or
`ParseError` `Debug` output is, by virtue of the
[behavioural contract](./behavioural-contract.md), a breaking change for
this binary.

<!-- cSpell:ignore assertables confy crates serde thiserror udeps walkdir Rhai strsim -->
