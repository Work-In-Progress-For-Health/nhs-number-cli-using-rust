# Usage

`nhs-number-cli` is a filter in the Unix tradition. It reads lines from
standard input and writes results to standard output and standard error.

## Synopsis

```
nhs-number-cli < INPUT > STDOUT 2> STDERR
```

The tool accepts no command-line options or arguments. All input is read
from `stdin`; there are no flags to configure.

## Input

* One NHS Number per line.
* Whitespace between digits is tolerated. Both `9991234560` and
  `999 123 4560` parse successfully.
* Blank lines are skipped.
* Lines with other content (letters, punctuation other than spaces,
  fewer or more than ten digits) are reported as parse errors.

## Output

* `stdout` — one line per **valid** NHS Number, in the canonical
  `NNN NNN NNNN` form.
* `stderr` — one line per input line that could not be validated, either
  because it failed to parse or because its check digit was wrong.

## Stream ordering

Stdout and stderr are separate streams and may be reordered by the
terminal or by downstream pipelines. Within each stream, output is in
input order. If you need a combined, ordered log, redirect both streams
into the same sink:

```sh
nhs-number-cli < input.txt > output.log 2>&1
```

## Exit status

The exit status is always **0**. Invalid input does not cause a non-zero
exit. If you need to fail a build or script when invalid numbers are
present, inspect the `stderr` output:

```sh
if nhs-number-cli < input.txt 2>&1 >/dev/null | grep -q '^Error '; then
  echo "Invalid NHS Numbers detected." >&2
  exit 1
fi
```

## Error message format

Errors on `stderr` have one of two forms:

```
Error invalid line <n>. Error: validate check digit failed. NHS Number: <nhs-number>
Error parsing line <n>. Error: <debug>. Line: <original-line>
```

Where `<n>` is the **zero-based** line index from the input. The format
is considered stable; scripts may grep for `^Error ` to detect problems.

## Reading from a file

`nhs-number-cli` does not take a filename argument. Redirect or pipe
the file:

```sh
cat input.txt | nhs-number-cli
nhs-number-cli < input.txt
```

On Windows:

```sh
type input.txt | nhs-number-cli.exe
```

## Separating valid from invalid output

Because the streams are already separated, you can redirect each to its
own file:

```sh
nhs-number-cli < input.txt > valid.txt 2> errors.txt
```

## Counting valid and invalid

```sh
cat input.txt | nhs-number-cli > valid.txt 2> errors.txt
echo "valid:   $(wc -l < valid.txt)"
echo "invalid: $(wc -l < errors.txt)"
```

## Integration with other tools

### Strip whitespace first

If your input file has trailing whitespace or CR line endings (common on
Windows), normalize before piping:

```sh
tr -d '\r' < windows-file.txt | nhs-number-cli
```

### Deduplicate before validating

```sh
sort -u input.txt | nhs-number-cli
```

### Extract NHS Numbers from a CSV

If the NHS Number is in column 3 of a comma-separated file:

```sh
cut -d, -f3 < patients.csv | tail -n +2 | nhs-number-cli
```

### Pipe directly from a query result

```sh
psql -At -c "SELECT nhs_number FROM patients" | nhs-number-cli
```

See [Examples](./examples.md) and the top-level [`examples/`](../examples)
directory for full working recipes.
