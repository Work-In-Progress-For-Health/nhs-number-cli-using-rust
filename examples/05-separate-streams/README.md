# 05-separate-streams

Shows how to redirect valid numbers to one file and error messages to
another, using shell redirection on the two streams:

```sh
nhs-number-cli < input.txt > valid.txt 2> errors.txt
```

This pattern is useful for batch-validating a file and then processing
the two results independently — for example, loading the valid numbers
into a database while emailing the error list to an operator.

## Run

```sh
./run.sh
```

The script writes to `valid.txt` and `errors.txt` next to the example
files, then diffs against the `expected-*` files.
