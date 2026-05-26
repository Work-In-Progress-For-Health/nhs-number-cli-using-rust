pub(crate) mod check_lines;
pub(crate) mod counts;

/// Pick a 1-based column from a comma-separated line.
///
/// Returns `None` if the line has fewer than `column` fields.
/// Splitting is on the literal byte `,`; quoted fields containing
/// commas are not supported (out of scope — NHS Numbers never
/// contain commas in any documented format).
///
/// Callers pass the whole input line if `column` is `None`.
pub(crate) fn pick_column(line: &str, column: usize) -> Option<&str> {
    if column == 0 {
        return None;
    }
    line.split(',').nth(column - 1)
}
