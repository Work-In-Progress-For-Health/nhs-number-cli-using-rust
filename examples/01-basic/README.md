# 01-basic

The smallest useful example: one valid NHS Number and one invalid
number. Demonstrates that:

* a valid number appears on `stdout` in canonical form;
* an invalid number produces a diagnostic on `stderr`;
* the two streams are kept separate.

## Run

```sh
./run.sh
```

The script pipes `input.txt` through the binary and diffs the observed
output against `expected-stdout.txt` and `expected-stderr.txt`.
