# Examples

Runnable, self-contained examples for `nhs-number-cli`. Each example
directory contains:

* `README.md` — what the example shows and how to run it.
* `input.txt` — the input piped into the tool.
* `expected-stdout.txt` — the standard-output bytes expected.
* `expected-stderr.txt` — the standard-error bytes expected.
* `run.sh` — a shell script that runs the example and diffs against the
  expected files. Exit code 0 means the observed output matched.

## Index

1. [01-basic](./01-basic/) — validate one valid and one invalid number.
2. [02-valid-only](./02-valid-only/) — a file of numbers that all pass.
3. [03-mixed-formats](./03-mixed-formats/) — spaced, unspaced, and mixed
   whitespace.
4. [04-blank-lines](./04-blank-lines/) — blank lines are skipped.
5. [05-separate-streams](./05-separate-streams/) — redirect valid and
   invalid to separate files.
6. [06-csv-column](./06-csv-column/) — extract an NHS Number column from
   a CSV and validate it.
7. [07-fail-on-invalid](./07-fail-on-invalid/) — wrapper script that
   exits non-zero when any line is invalid.
8. [08-crlf-windows](./08-crlf-windows/) — handle CRLF line endings from
   Windows-authored input files.
9. [09-counts-summary](./09-counts-summary/) — print a count of valid and
   invalid inputs.
10. [10-parse-errors](./10-parse-errors/) — examples of lines that fail
    to parse (versus fail validation).

## Running an example

From the repository root, first build the binary:

```sh
cargo build
```

Then run an example. Each example's `run.sh` assumes the debug binary is
at `../../target/debug/nhs-number-cli` relative to the example
directory.

```sh
cd examples/01-basic
./run.sh
```

If the output matches the expected files, `run.sh` prints `OK` and
exits 0.

## Running all examples

Use the orchestrator at `examples/run-all.sh`:

```sh
./examples/run-all.sh
```

It exits non-zero if any example fails. Under the hood it runs each
`run.sh` in turn and collects the results.

## About the test numbers

All examples use NHS Numbers from the `999 000 0000` to `999 999 9999`
synthetic test range. These are reserved for testing by the NHS and are
never issued to a real patient.

Valid test numbers used in these examples (all have correct Modulus 11
check digits):

```
999 000 0000
999 000 0018
999 000 0026
999 000 0034
999 000 0042
999 000 0069
999 123 4560
999 555 0016
999 555 0024
999 888 1005
999 888 1013
999 999 9999
```

Intentionally invalid numbers used for error cases:

```
999 123 4561    # correct shape, wrong check digit
999 000 0001    # correct shape, wrong check digit
not-an-nhs-no   # unparseable
1234            # too short
```

**Never** put a real NHS Number into a test fixture, bug report, commit
message, or example. Patient identifiers are personal data under the
UK GDPR and NHS information governance policies.
