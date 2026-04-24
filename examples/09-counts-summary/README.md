# 09-counts-summary

A wrapper that prints a summary: how many lines were valid, how many
were invalid, and how many were blank. Useful after a batch import.

## Run

```sh
./run.sh
```

## Pattern

```sh
./summary.sh < input.txt
```

produces output like:

```
valid:   4
invalid: 2
blank:   1
```

Valid and invalid counts come from counting lines on each output
stream. The blank count comes from a simple `awk` scan over the input.
