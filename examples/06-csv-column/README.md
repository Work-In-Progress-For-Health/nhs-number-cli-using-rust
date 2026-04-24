# 06-csv-column

Pull an NHS Number column out of a CSV file and validate each entry.
The CSV here (`patients.csv`) has three columns: `id`, `name`, and
`nhs_number`. The `run.sh` script strips the header with `tail -n +2`
and picks column 3 with `cut -d, -f3` before piping into the tool.

## Run

```sh
./run.sh
```

## Pattern

```sh
cut -d, -f3 < patients.csv | tail -n +2 | nhs-number-cli > valid.txt 2> errors.txt
```

For CSVs with quoted fields or embedded commas, use a real CSV parser
(such as `xsv`, `csvkit`, or `miller`) instead of `cut`.
