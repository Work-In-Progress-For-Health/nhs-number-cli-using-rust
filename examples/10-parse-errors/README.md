# 10-parse-errors

The tool reports two distinct kinds of failure:

* **Validation failure** — the line parsed as ten digits but the tenth
  digit is not the correct Modulus 11 check digit.
* **Parse failure** — the line could not be read as an NHS Number at
  all.

This example focuses on the second category and shows several inputs
that fail to parse, each for a different reason.

## Run

```sh
./run.sh
```
