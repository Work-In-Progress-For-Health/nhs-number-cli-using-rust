# Frequently asked questions

## Does this tool generate NHS Numbers?

No. It only validates numbers you already have. If you need to generate
synthetic test numbers, use the
[`nhs-number`](https://crates.io/crates/nhs-number) crate directly or
one of the NHS-published test data generators.

## Does it support CHI numbers (Scotland)?

No. CHI numbers are ten digits like NHS Numbers but use a different
check digit algorithm. Use a CHI-specific validator.

## Does it connect to NHS systems to check whether a number is *real*?

No. The tool only checks syntactic validity (ten digits, correct Modulus
11 check digit). A number that passes this check may still not belong to
any registered patient.

## Why is the line numbering zero-based?

It comes from Rust's `Iterator::enumerate`. Changing it would break
downstream scripts that grep the error output. It is documented as part
of the [behavioural contract](../../AGENTS/behavioural-contract.md).

## Why does the exit code stay zero on bad input?

Because this is a filter, not a validator. "I processed your lines" is
the exit-0 contract; per-line results are on `stderr`. See the
[architecture doc](../architecture/index.md#why-exit-code-0-even-on-errors)
for the full rationale and a wrapper script if you need a different
policy.

## How do I get a count summary instead of per-line output?

Use `--counts` (or `-c`):

```sh
nhs-number-cli --counts < input.txt
```

prints a four-row summary on stdout:

```
valid:       <n>
invalid:     <n>
parse-error: <n>
blank:       <n>
```

`--counts` is mutually exclusive with `--line-validation`; clap will
reject the combination. See
[`examples/09-counts-summary/`](../../examples/09-counts-summary/).

## How do I validate the NHS Number column in a CSV?

Use `--column N` to pick the *N*-th (1-based) comma-separated field.
Skip the header row first:

```sh
tail -n +2 < patients.csv | nhs-number-cli --column 3
```

Splitting is on the literal byte `,`; quoted-CSV (RFC 4180) is out of
scope. For files with quoted fields or embedded commas, use a real CSV
parser (`xsv`, `csvkit`, `miller`) upstream. See
[`examples/06-csv-column/`](../../examples/06-csv-column/).

## Can the diagnostics come out as JSON or TSV?

Yes — `--format json` or `--format tsv` switches the wire format on
stderr (and the `--counts` summary on stdout). The default `text`
format is the FR-10 stable contract and is byte-for-byte unchanged.
See [`examples/12-format-output/`](../../examples/12-format-output/).

## Can I add a flag to make the tool stricter / looser?

Possibly. Open an issue describing the use case. Flags add surface area
that every future maintainer has to respect, so the bar is high.

## Where can I report bugs or request features?

<https://github.com/joelparkerhenderson/nhs-number-cli/issues>.

## Who do I contact for anything else?

Joel Parker Henderson, <joel@joelparkerhenderson.com>.
