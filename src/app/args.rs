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
}
