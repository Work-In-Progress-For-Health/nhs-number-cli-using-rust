# 07-fail-on-invalid

`nhs-number-cli` always exits 0 by design. If you need a CI job or
pre-commit hook to fail when any invalid line is detected, wrap the
tool in a small shell script. `validate-strict.sh` is one such wrapper.

It reads from `stdin`, passes everything through `nhs-number-cli`, and
exits:

* `0` when every non-blank line is valid;
* `1` when at least one line was invalid or unparseable.

Both `stdout` and `stderr` of `nhs-number-cli` are forwarded to the
caller, so you still see the valid lines and the error messages.

## Run

```sh
./run.sh
```

The example has two inputs: one with only valid numbers (exit 0) and
one with a bad check digit (exit 1).

## The wrapper

```sh
./validate-strict.sh < good.txt   # exit 0
./validate-strict.sh < bad.txt    # exit 1
```
