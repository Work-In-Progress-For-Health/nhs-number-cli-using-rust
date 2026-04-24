# 03-mixed-formats

Demonstrates which input spacing styles the parser accepts. At the time
of writing, the underlying `nhs-number` crate accepts:

* ten digits with no spaces (`9991234560`);
* ten digits with single spaces in the canonical `NNN NNN NNNN` form
  (`999 123 4560`).

It does **not** accept:

* doubled or irregular spacing (`999  123  4560`);
* dashes or other separators (`999-123-4560`).

On output, the tool always prints the canonical `NNN NNN NNNN` form.
Whitespace-free input is normalised on the way through.

If you need to tolerate more permissive input, normalise it before
piping. For example, to strip all non-digit characters and re-insert
canonical spaces, use `awk` or `sed` as a preprocessing step.

## Run

```sh
./run.sh
```
