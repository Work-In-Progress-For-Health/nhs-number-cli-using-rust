# About NHS Numbers

An **NHS Number** is a unique identifier allocated to every person
registered with the public health services of England, Wales, and the Isle
of Man. Scotland uses a different identifier (the CHI number), and
Northern Ireland uses the H&C number.

## Shape

An NHS Number is **ten digits**, conventionally written as three groups:

```
NNN NNN NNNN
```

For example: `999 123 4560`.

The digits are often stored without spaces in databases. `nhs-number-cli`
accepts both styles — any whitespace between digits is tolerated by the
underlying `nhs-number` crate's parser. The canonical `Display` form that
the tool emits on standard output is the spaced `NNN NNN NNNN` form.

## Check digit

The tenth (rightmost) digit is a **check digit** derived from the first
nine digits using the Modulus 11 algorithm:

1. Multiply each of the first nine digits by a weight. The weights are
   `10, 9, 8, 7, 6, 5, 4, 3, 2`, applied left to right.
2. Sum the products.
3. Divide by 11 and take the remainder.
4. Subtract the remainder from 11.
5. If the result is 11, the check digit is 0. If the result is 10, the
   number is invalid (it cannot be a legal NHS Number). Otherwise, the
   result is the check digit.

`nhs-number-cli` rejects any input whose computed check digit does not
match the tenth digit on the line.

## Test numbers

The NHS publishes synthetic test ranges for use in development and test
environments. The ranges `999 000 0000` through `999 999 9999` are
reserved for testing and will never be issued to a real patient. Use only
these ranges in test data, bug reports, and public examples.

The examples in this repository use numbers drawn from the `999`
synthetic range.

## References

* [NHS Number](https://en.wikipedia.org/wiki/NHS_number) on Wikipedia.
* [NHS Digital — NHS Number](https://digital.nhs.uk/services/nhs-number).
* [`nhs-number` crate](https://crates.io/crates/nhs-number) — the Rust
  library that does the parsing and validation.
