//! Args for the application.
//!
//! These args correspond to the matches in the file `clap.rs`.
//! We have these args in their own file in order to be flexible,
//! such as being able to start our app with other arg parsers.

#[derive(Debug, Default)]
pub struct Args {
    /// Test flag that sets whether the app prints diagnostics.
    /// Example: true means print diagnostics.
    pub(crate) test: bool,

    /// Log level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace.
    /// Example: 5 means print debug diagnostics.
    pub(crate) log_level: Option<::log::Level>,

    /// Line validation subcommand: true=run, false=skip
    pub(crate) check_lines: Option<bool>,

    /// Counts subcommand: emit a summary of valid / invalid /
    /// parse-error / blank line counts to stdout instead of
    /// per-line filtering. Mutually exclusive with `check_lines`.
    pub(crate) counts: Option<bool>,

    /// 1-based column index. When set, each input line is split on
    /// `,` and the value at column N is taken as the candidate NHS
    /// Number. When None, the whole line is the candidate. Honoured
    /// by every subcommand that classifies lines.
    pub(crate) column: Option<usize>,

    /// Output format selector. `Text` is the FR-10 stable contract
    /// (`Error invalid line N. …` on stderr; key/value rows on stdout
    /// from `--counts`). `Json` and `Tsv` are machine-readable
    /// alternatives for downstream tooling.
    pub(crate) format: Format,
}

/// Output format for diagnostics (line-validation subcommand) and
/// for the counts summary. Plain stdout from line-validation (the
/// canonical NHS Numbers) is always text and is not affected.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// FR-10 stable text format. The default; do not change.
    #[default]
    Text,
    /// One JSON object per diagnostic line (NDJSON). The counts
    /// summary is a single JSON object.
    Json,
    /// Tab-separated values. The line-validation diagnostic stream
    /// is one row per failure with a fixed five-column schema
    /// (kind, line_number, nhs_number, line, error). The counts
    /// summary is one row of column headers followed by one row of
    /// integer counts.
    Tsv,
}

impl Format {
    /// Map the clap string value to a `Format`. Returns `Text` for
    /// any unrecognised string; clap's `value_parser` rejects those
    /// before this function ever sees them.
    pub fn from_str(s: &str) -> Self {
        match s {
            "json" => Format::Json,
            "tsv" => Format::Tsv,
            _ => Format::Text,
        }
    }
}
