# Stream validator

Syntax:

```sh
… | nhs-number-cli -l
… | nhs-number-cli --line-validation
```

* If the line is a valid NHS number, then print it.

* If the line is an invalid NHS Number, or is unparseable, then print an error message.

* If the line is blank, then skip it.

## Examples

Suppose you have a text file `input.txt` that contains one NNS Number per line,
and some may be valid or invalid:
//!

```txt
999 999 9999
999 123 4561
```

You can parse each line and validate it:

```sh
cat input.txt | nhs-number-cli --line-validation
```

The output is one stdout line and one stderr line: 

```stdout
999 999 9999
```

```stderr
Error invalid line 1. Error: validate check digit failed. NHS Number: 999 123 4561
```
