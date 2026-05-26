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

/// Minimal JSON string escaper. Returns the string wrapped in
/// double quotes with the standard JSON escape sequences applied.
///
/// Only escapes the characters JSON requires: `"`, `\`, and the
/// C0 control range (`U+0000`–`U+001F`). NHS Numbers and the
/// strings the binary normally formats are ASCII, but raw input
/// lines (in the `line` field of parse errors) can contain
/// anything, including embedded quotes and control characters.
pub(crate) fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Sanitise a string for use as a TSV field: replace tab and
/// newline characters with spaces so they cannot interfere with
/// the field/row separators. Other control characters are left
/// alone; downstream tools that care should validate input
/// themselves.
pub(crate) fn tsv_escape(s: &str) -> String {
    s.replace(['\t', '\n', '\r'], " ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pick_column_basic() {
        assert_eq!(pick_column("a,b,c", 1), Some("a"));
        assert_eq!(pick_column("a,b,c", 2), Some("b"));
        assert_eq!(pick_column("a,b,c", 3), Some("c"));
        assert_eq!(pick_column("a,b,c", 4), None);
        assert_eq!(pick_column("a,b,c", 0), None);
        assert_eq!(pick_column("", 1), Some(""));
    }

    #[test]
    fn json_escape_specials() {
        assert_eq!(json_escape(""), r#""""#);
        assert_eq!(json_escape("abc"), r#""abc""#);
        assert_eq!(json_escape(r#"a"b"#), r#""a\"b""#);
        assert_eq!(json_escape("a\\b"), r#""a\\b""#);
        assert_eq!(json_escape("a\nb"), r#""a\nb""#);
        assert_eq!(json_escape("a\tb"), r#""a\tb""#);
        assert_eq!(json_escape("\x01"), r#""\u0001""#);
    }

    #[test]
    fn tsv_escape_basic() {
        assert_eq!(tsv_escape("abc"), "abc");
        assert_eq!(tsv_escape("a\tb"), "a b");
        assert_eq!(tsv_escape("a\nb"), "a b");
        assert_eq!(tsv_escape("a\rb"), "a b");
    }
}
