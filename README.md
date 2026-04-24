# NHS Number command line interface

A National Health Service (NHS) Number is a unique number allocated in a shared
numbering scheme to registered users of the three public health services in
England, Wales, and the Isle of Man.

This tool is a command line interface that parses each standard input line into
an NHS number, then validates the check digit is correct.

* If the line is a valid NHS number, then print it.

* If the line is an invalid NHS Number, or is unparseable, then print an error message.

* If the line is blank, then skip it.

References:

* [National Health Service (NHS)](https://en.wikipedia.org/wiki/National_Health_Service)

* [NHS Number](https://en.wikipedia.org/wiki/NHS_number)

## Examples

Suppose you have a text file `input.txt` that contains one NNS Number per line,
and some may be valid or invalid:

```txt
999 123 4560
999 123 4561
```

If you use Linux or macOS, then here's how to parse each line and validate it:

```sh
cat input.txt | nhs-number-cli
```

If you use Windows, then here's how to parse each line and validate it:

```sh
type input.txt | nhs-number-cli.exe
```

The commmand prints the valid NHS Number to stdout:

```stdout
999 123 4560
```

The commmand prints the invalid NHS Number to stderr:

```stderr
Error invalid line 1. Error: validate check digit failed. NHS Number: 999 123 4561
```

## Releases

You can build a release for your own platform:

```sh
cargo build --release
```

You can download a prebuilt release for Windows:

* Windows with GNU: <target/x86_64-pc-windows-gnu/release/nhs-number-cli.exe>

We aim to add more prebuilt releases soon.

## Documentation

Full documentation lives under [`docs/`](./docs/):

* [About NHS Numbers](./docs/about-nhs-numbers.md)
* [Installation](./docs/installation.md)
* [Usage](./docs/usage.md)
* [Examples](./docs/examples.md)
* [Troubleshooting](./docs/troubleshooting.md)
* [Architecture](./docs/architecture.md)
* [Development](./docs/development.md)
* [FAQ](./docs/faq.md)

Runnable examples with expected-output fixtures and `run.sh` scripts
live under [`examples/`](./examples/). Run them all with:

```sh
cargo build
./examples/run-all.sh
```

## For AI coding agents

See [`AGENTS.md`](./AGENTS.md) for conventions, the behavioural
contract, and things to avoid when modifying this repository with the
help of AI tools.
