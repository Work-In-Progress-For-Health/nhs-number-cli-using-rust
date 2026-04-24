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
of the [behavioural contract](../AGENTS.md#behavioural-contract).

## Why does the exit code stay zero on bad input?

Because this is a filter, not a validator. "I processed your lines" is
the exit-0 contract; per-line results are on `stderr`. See the
[architecture doc](./architecture.md#why-exit-code-0-even-on-errors) for
the full rationale and a wrapper script if you need a different policy.

## Can I add a flag to make the tool stricter / looser?

Possibly. Open an issue describing the use case. Flags add surface area
that every future maintainer has to respect, so the bar is high.

## Where can I report bugs or request features?

<https://github.com/GIG-Cymru-NHS-Wales/nhs-number-cli-using-rust/issues>.

## Who do I contact for anything else?

Joel Henderson, <joel.henderson@wales.nhs.uk>.
