# Commits, branches, releases

Conventions for version control and release management. Agents should
follow these whenever they are asked to commit, branch, push, tag, or
release.

## Commits

* **One concept per commit.** A formatting pass, a behaviour change, and
  a doc update are three commits, not one. Tests for a behaviour change
  belong in the same commit as the change.
* **Imperative subject, ≤ 72 chars.** "Add `--strict` flag" not "Added"
  or "Adds".
* **Body explains the *why*.** Wrap at ~72 columns. Reference the issue
  number if one exists.
* **Sign-off / co-author lines** are optional. If an AI agent authored a
  commit, add a `Co-Authored-By:` trailer naming the model.
* **Do not commit:**
  * Real or potentially-real NHS Numbers (see `examples/README.md`).
  * `.env`, credentials, or any secret material.
  * Editor cruft (`.DS_Store`, `*.swp`, IDE configs not in `.gitignore`).
  * Generated artefacts other than the deliberately-tracked Windows
    binary at `target/x86_64-pc-windows-gnu/release/nhs-number-cli.exe`.

## Pre-commit checklist

Run before every push:

```sh
cargo fmt
cargo clippy -- -D warnings
cargo test
./examples/run-all.sh
```

If any of these fail, fix the underlying cause. Do not silence clippy,
do not `#[allow(dead_code)]` to dodge an unused-import warning, do not
edit a test to make it pass.

## Branches

* `main` is the default and always-buildable branch.
* Feature branches: `feat/<short-slug>`.
* Bug fixes: `fix/<short-slug>` or `fix/<issue-number>`.
* Releases: tags only — no long-lived release branches.

Force-pushing to `main` is forbidden. Force-pushing to your own
feature branch before opening a PR is fine; after opening a PR, prefer
additive commits so reviewers can follow the diff.

## Pull requests

PR description template (paste into the body):

```markdown
## Summary
- <one-line bullet per change>

## Behavioural contract
- [ ] No change to stdin/stdout/stderr behaviour, OR
- [ ] Major version bumped and `spec.md` + `AGENTS/behavioural-contract.md` updated.

## Tests
- [ ] `cargo fmt` clean
- [ ] `cargo clippy -- -D warnings` clean
- [ ] `cargo test` passes
- [ ] `./examples/run-all.sh` passes
- [ ] If observable behaviour changed: new example added under `examples/`

## Spec
- Linked requirements in `spec.md`: FR-…, NFR-…
```

A PR that changes behaviour without updating `spec.md` should not merge.

## Versioning

Semantic versioning, with one extra rule:

* **Any change to the stream contract is a major bump**, even when it
  *seems* like an addition (for example, adding a new line to stderr per
  invalid input changes what `wc -l < errors.txt` returns).
* **Adding a new flag is a minor bump** if the default behaviour is
  unchanged.
* **Adding a new subcommand** is a minor bump if the default behaviour
  (with no flags) is unchanged.

## Releases

1. Update `version` in `Cargo.toml`.
2. Update `spec.md`: bump the version header, mark any spec lines that
   changed status.
3. Run the full pre-commit checklist.
4. Commit: `Release vX.Y.Z`.
5. Tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`.
6. Push: `git push && git push --tags`.
7. Build the Windows GNU binary (if you have the toolchain):
   ```sh
   cargo build --release --target x86_64-pc-windows-gnu
   ```
   Commit the resulting `nhs-number-cli.exe`.
8. (Optional) `cargo publish` to crates.io once the crate is published.

## Don't do these

* Don't `git push --force` to `main`.
* Don't amend a published commit.
* Don't skip hooks (`--no-verify`). If a hook fails, fix the cause.
* Don't bypass clippy with `#[allow(clippy::…)]` without a one-line
  justification on the same line.
* Don't commit `Cargo.lock` changes from unrelated dependency drift in a
  feature PR. Either commit the drift alone first, or revert it.

<!-- cSpell:ignore clippy confy crates rustfmt udeps -->
