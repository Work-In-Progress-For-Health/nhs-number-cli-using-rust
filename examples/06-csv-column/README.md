# 06-csv-column

Pull an NHS Number column out of a CSV file and validate each entry.
The CSV here (`patients.csv`) has three columns: `id`, `name`, and
`nhs_number`. The `run.sh` script strips the header with `tail -n +2`
and lets the binary's built-in `--column 3` flag pick the third
field of each remaining row.

## Run

```sh
./run.sh
```

## Pattern

```sh
tail -n +2 < patients.csv | nhs-number-cli --column 3 > valid.txt 2> errors.txt
```

The `--column N` flag splits each non-blank input line on `,` and
takes the *N*-th (1-based) field as the candidate NHS Number. The
*full* input line is preserved in `Line: …` on stderr so a downstream
script can correlate failures back to the original CSV row.

## When to use which pattern

`--column N` is the minimal-dependency approach: no extra shell
plumbing, works on every platform that runs the binary. Limitations:

* Splitting is on the literal byte `,`. Quoted fields containing
  commas are not understood. NHS Numbers never contain commas in
  any documented format, so this is fine for an NHS-Number column
  but not for free-text columns.
* Header rows are not skipped automatically. Pipe through
  `tail -n +2` (Unix) or equivalent.

For CSVs with quoted fields, embedded commas, or other quoting
edge cases, prefer a real CSV parser (such as `xsv`, `csvkit`, or
`miller`) upstream of the binary, and feed it a clean stream of
single-field lines:

```sh
xsv select nhs_number patients.csv | tail -n +2 | nhs-number-cli
```
