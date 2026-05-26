# 12-format-output

The `--format` flag selects how diagnostics are emitted on stderr
(and how `--counts` writes its summary on stdout). `text` is the
default and the FR-10 stable contract; `json` and `tsv` are for
downstream tooling that wants machine-readable output.

## Run

```sh
./run.sh
```

## Patterns

```sh
nhs-number-cli --format json < input.txt 2> diagnostics.ndjson
nhs-number-cli --format tsv  < input.txt 2> diagnostics.tsv
```

* **`text`** (default): preserves the byte-for-byte FR-10 contract
  (`Error invalid line N. …`, `Error parsing line N. …`,
  `Error reading line N. …`).
* **`json`**: one JSON object per diagnostic line (NDJSON). Three
  kinds:
  - `{"kind":"check_digit","line_number":N,"nhs_number":"…"}`
  - `{"kind":"parse_error","line_number":N,"line":"…","error":"…"}`
  - `{"kind":"io_error","line_number":N,"error":"…"}`
* **`tsv`**: a fixed five-column schema —
  `kind` `\t` `line_number` `\t` `nhs_number` `\t` `line` `\t` `error` —
  with empty fields where they don't apply.

Stdout from the line-validation subcommand (the canonical
`NNN NNN NNNN` numbers) is **plain text in every format**. Only the
diagnostic stream changes shape.

## With `--counts`

`--counts` also honours `--format`:

```sh
nhs-number-cli --counts --format json   # → {"valid":3,"invalid":1,"parse_error":1,"blank":2}
nhs-number-cli --counts --format tsv    # → valid\tinvalid\tparse-error\tblank
                                        #   3\t1\t1\t2
nhs-number-cli --counts                 # → four key/value rows (default)
```

## Why hand-roll JSON?

The binary writes JSON without `serde_json` to keep the dependency
closure small (NFR-7). The escape rules implemented are the minimum
JSON requires: `"`, `\\`, and C0 controls (`U+0000`–`U+001F`). NHS
Numbers and Modulus-11 results are ASCII; raw input lines (in the
`line` field of parse errors) may contain anything and are escaped
the same way.
