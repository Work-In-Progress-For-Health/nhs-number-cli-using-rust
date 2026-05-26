//! Command line argument parsing (CLAP) for the application.
//!
//! clap is a crate for command line argument parsing.
//! See https://docs.rs/clap/
//!
//! Clap has a variety of setup approaches:
//!
//!   * via typical functions, which favors advanced uses yet is verbose.
//!   * via usage strings, which looks more like writing documentation.
//!   * via macros, which is fast and less verbose, yet atypical to read.
//!   * via YAML file, which favors localization and text file readability.
//!
//! We prefer the typical functions, because they provide maximum capability,
//! and in our experience are the easiest for Rust IDEs to read and debug.
//!
//! We favor our convention of doing clap setup in a file named `clap.rs`,
//! rather than in `main.rs`, because we like the separation of concerns.

use crate::app::args::Args;
use clap::{Arg, Command};

/// Create a clap app.
pub fn app() -> Command {
    trace!("app");
    Command::new("nhs-number-cli")
    .name(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .arg(Arg::new("line-validation")
        .help("Validate each line of standard input as an NHS number.\nPrint the valid ones to standard output.\nPrint the invalid ones to standard error.\nExample: --line-validation")
        .short('l')
        .long("line-validation")
        .action(clap::ArgAction::SetTrue)
        .conflicts_with("counts")
    )
    .arg(Arg::new("counts")
        .help("Read lines from stdin and print a summary count of valid, invalid, parse-error, and blank lines on stdout.\nExample: --counts")
        .short('c')
        .long("counts")
        .action(clap::ArgAction::SetTrue)
    )
    .arg(Arg::new("column")
        .help("1-based column index. Each input line is split on `,` and the value at column N is taken as the candidate NHS Number. Whole-line behaviour is the default.\nExample: --column 3")
        .long("column")
        .value_name("N")
        .value_parser(clap::value_parser!(usize))
    )
    .arg(Arg::new("test")
        .help("Print test output for debugging, verifying, tracing, and the like.\nExample: --test")
        .long("test")
        .action(clap::ArgAction::SetTrue)
    )
    .arg(Arg::new("verbose")
        .help("Set the verbosity level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace.\nExample: --verbose …")
        .short('v')
        .long("verbose")
        .action(clap::ArgAction::Count)
    )
}

/// Parse argv with the clap `Command` and project the matches into `Args`.
pub fn args() -> Args {
    trace!("args");
    let matches = app().get_matches();
    let test = matches.get_flag("test");
    let log_level = match matches.get_count("verbose") {
        0 => None,
        1 => Some(::log::Level::Error),
        2 => Some(::log::Level::Warn),
        3 => Some(::log::Level::Info),
        4 => Some(::log::Level::Debug),
        _ => Some(::log::Level::Trace),
    };
    let check_lines = if matches.get_flag("line-validation") {
        Some(true)
    } else {
        None
    };
    let counts = if matches.get_flag("counts") {
        Some(true)
    } else {
        None
    };
    let column = matches.get_one::<usize>("column").copied();
    Args {
        test,
        log_level,
        check_lines,
        counts,
        column,
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    use crate::testing::*;
    use assertables::*;

    /// Test that the special argument `--test` is working.
    ///
    /// This test must succeed in order for any of the rest of the tests here to
    /// succeed, because the `--test` argument turns on the runtime output to stdout,
    /// which includes a typical print debug of the entire `args` structure.
    ///
    /// Example:
    ///
    ///     nhs-number-cli --test
    ///
    #[test]
    fn test_test() {
        assert_program_args_stdout_string_contains!(&*COMMAND_OS, &["--test"], r#"Args { "#);
    }

    /// Test that the special argument `--verbose` is working.
    ///
    /// This test must succeed in order for any of the rest of the tests here to
    /// show diagnostics, because the `--verbose` argument turns on logging output,
    /// which can include debugging messages, warnings, errors, and so on.
    ///
    /// Example:
    ///
    ///     sita --verbose
    ///
    #[test]
    fn test_verbose() {
        assert_program_args_stdout_string_contains!(
            &*COMMAND_OS,
            &["--test"],
            r#" log_level: None"#
        );
        assert_program_args_stdout_string_contains!(
            &*COMMAND_OS,
            &["--test", "-v"],
            r#" log_level: Some(Error)"#
        );
        assert_program_args_stdout_string_contains!(
            &*COMMAND_OS,
            &["--test", "-vv"],
            r#" log_level: Some(Warn)"#
        );
        assert_program_args_stdout_string_contains!(
            &*COMMAND_OS,
            &["--test", "-vvv"],
            r#" log_level: Some(Info)"#
        );
        assert_program_args_stdout_string_contains!(
            &*COMMAND_OS,
            &["--test", "-vvvv"],
            r#" log_level: Some(Debug)"#
        );
        assert_program_args_stdout_string_contains!(
            &*COMMAND_OS,
            &["--test", "-vvvvv"],
            r#" log_level: Some(Trace)"#
        );
        assert_program_args_stdout_string_contains!(
            &*COMMAND_OS,
            &["--test", "--verbose"],
            r#" log_level: Some(Error)"#
        );
        assert_program_args_stdout_string_contains!(
            &*COMMAND_OS,
            &["--test", "--verbose", "--verbose"],
            r#" log_level: Some(Warn)"#
        );
        assert_program_args_stdout_string_contains!(
            &*COMMAND_OS,
            &["--test", "--verbose", "--verbose", "--verbose"],
            r#" log_level: Some(Info)"#
        );
        assert_program_args_stdout_string_contains!(
            &*COMMAND_OS,
            &["--test", "--verbose", "--verbose", "--verbose", "--verbose"],
            r#" log_level: Some(Debug)"#
        );
        assert_program_args_stdout_string_contains!(
            &*COMMAND_OS,
            &[
                "--test",
                "--verbose",
                "--verbose",
                "--verbose",
                "--verbose",
                "--verbose"
            ],
            r#" log_level: Some(Trace)"#
        );
    }

    /// Test that the special argument `--verbose` is working.
    ///
    /// This test must succeed in order for any of the rest of the tests here to
    /// show diagnostics, because the `--verbose` argument turns on logging output,
    /// which can include debugging messages, warnings, errors, and so on.
    ///
    /// Example:
    ///
    ///     sita --verbose
    ///
    #[test]
    fn test_check_lines() {
        assert_program_args_stdout_string_contains!(
            &*COMMAND_OS,
            &["--test", "--line-validation"],
            r#" check_lines: Some(true)"#
        );
    }

    /// Test that the `--counts` argument is parsed into
    /// `Args::counts`. The `--test` argument prints the `Args`
    /// struct to stdout so we can assert on its `Debug` form.
    #[test]
    fn test_counts() {
        assert_program_args_stdout_string_contains!(
            &*COMMAND_OS,
            &["--test", "--counts"],
            r#" counts: Some(true)"#
        );
    }
}
