# 09-counts-summary

The `--counts` flag prints a four-row summary to stdout instead of
running the per-line filter. Useful after a batch import or when you
only need to know how clean the input is.

## Run

```sh
./run.sh
```

## Pattern

```sh
nhs-number-cli --counts < input.txt
```

produces output like:

```
valid:       4
invalid:     1
parse-error: 1
blank:       2
```

* **valid** — line parsed and the Modulus 11 check digit was correct.
* **invalid** — line parsed as ten digits but the check digit was wrong.
* **parse-error** — line could not be parsed as ten digits at all,
  or a read error occurred.
* **blank** — empty line after stripping `\r?\n`.

`--counts` is mutually exclusive with `--line-validation`; clap will
reject `nhs-number-cli --counts --line-validation` with an error.

## Counting it yourself with shell tools

If you want the counts *and* the per-line output, run the binary
twice (once without `--counts`, once with) or build a shell wrapper
on top of the line-validation output:

```sh
nhs-number-cli < input.txt > valid.txt 2> errors.txt
echo "valid:   $(wc -l < valid.txt)"
echo "invalid: $(wc -l < errors.txt)"
```

The shape there is slightly different — "invalid" lumps check-digit
failures and parse errors together — but it composes naturally with
the rest of a pipeline.
