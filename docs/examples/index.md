# Examples

Cookbook-style recipes. Longer, runnable examples live under
[`examples/`](../examples/) in the repository root.

## Validate a single number

```sh
echo "999 123 4560" | nhs-number-cli
```

```
999 123 4560
```

## Validate a list of numbers

`input.txt`:

```
999 123 4560
9991234560
999 123 4561
not-an-nhs-number

999 000 0011
```

Run:

```sh
cat input.txt | nhs-number-cli
```

Expected `stdout`:

```
999 123 4560
999 123 4560
999 000 0011
```

Expected `stderr`:

```
Error invalid line 2. Error: validate check digit failed. NHS Number: 999 123 4561
Error parsing line 3. Error: ParseError. Line: not-an-nhs-number
```

(The exact error `Debug` output for parse errors comes from the
[`nhs-number`](https://crates.io/crates/nhs-number) crate and may change
between crate versions.)

## Save valid and invalid to separate files

```sh
nhs-number-cli < input.txt > valid.txt 2> errors.txt
```

## Filter a CSV

Given `patients.csv` with a header and an `nhs_number` column in
position 3:

```sh
cut -d, -f3 < patients.csv | tail -n +2 | nhs-number-cli > valid.txt 2> errors.txt
```

## Validate without mutating the file

If you want to report on a file without changing it:

```sh
nhs-number-cli < input.txt 2>&1 >/dev/null | tee report.txt
```

## Fail a CI job on any invalid number

```sh
if nhs-number-cli < input.txt 2>&1 >/dev/null | grep -q '^Error '; then
  echo "Input contains invalid NHS Numbers; failing." >&2
  exit 1
fi
```

## Handle CRLF line endings from Windows

```sh
tr -d '\r' < input.txt | nhs-number-cli
```

## Count valid and invalid

```sh
cat input.txt | nhs-number-cli > valid.txt 2> errors.txt
printf 'valid:   %d\ninvalid: %d\n' "$(wc -l < valid.txt)" "$(wc -l < errors.txt)"
```

## Deduplicate

```sh
sort -u input.txt | nhs-number-cli
```

## Process a stream from a database query

```sh
psql -At -c "SELECT nhs_number FROM patients" | nhs-number-cli > valid.txt 2> errors.txt
```

## PowerShell (Windows)

```powershell
Get-Content .\input.txt | .\nhs-number-cli.exe
```

Redirecting both streams:

```powershell
Get-Content .\input.txt | .\nhs-number-cli.exe 1> valid.txt 2> errors.txt
```
