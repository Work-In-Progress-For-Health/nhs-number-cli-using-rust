# 04-blank-lines

Blank lines are silently skipped: they produce no output on either
stream and do not advance the "error" count. They do, however, count
toward the zero-based line index used in error messages, so you can
still correlate an error back to a line number in the source file.

## Run

```sh
./run.sh
```
