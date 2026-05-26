use crate::app::args::Format;
use crate::subcommands::{json_escape, pick_column, tsv_escape};
use nhs_number::NHSNumber;
use std::io::{self, BufRead};
use std::str::FromStr;

/// Zero-based index of a line of input as it is read from `stdin`.
///
/// `Iterator::enumerate` produces `usize`, and that is what scripts
/// see on stderr too. Centralising the type here lets us widen or
/// narrow it in one place if we ever need to.
pub type LineIndex = usize;

/// Run the line-validation subcommand.
///
/// `column` selects a 1-based comma-separated field per row, or
/// `None` for whole-line input. `format` selects the wire format
/// of stderr diagnostics; `Format::Text` is the FR-10 stable
/// contract and the default.
pub fn check_lines(column: Option<usize>, format: Format) {
    let stdin = io::stdin();
    for (i, line) in stdin.lock().lines().enumerate() {
        match line {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }
                let candidate = match column {
                    Some(n) => match pick_column(&line, n) {
                        Some(s) => s,
                        None => {
                            emit_parse(i, &line, &format!("ColumnMissing({n})"), format);
                            continue;
                        }
                    },
                    None => &line,
                };
                match NHSNumber::from_str(candidate) {
                    Ok(nhs_number) => {
                        if nhs_number.validate_check_digit() {
                            println!("{}", nhs_number);
                        } else {
                            emit_check_digit(i, &nhs_number, format);
                        }
                    }
                    Err(e) => {
                        emit_parse(i, &line, &format!("{:?}", e), format);
                    }
                }
            }
            Err(e) => {
                emit_io(i, &e, format);
            }
        }
    }
}

fn emit_check_digit(line_number: LineIndex, nhs_number: &NHSNumber, format: Format) {
    match format {
        Format::Text => eprintln!(
            "{}",
            Error::CheckDigit {
                line_number,
                nhs_number: *nhs_number,
            }
        ),
        Format::Json => eprintln!(
            r#"{{"kind":"check_digit","line_number":{},"nhs_number":{}}}"#,
            line_number,
            json_escape(&nhs_number.to_string()),
        ),
        Format::Tsv => eprintln!(
            "check_digit\t{}\t{}\t\t",
            line_number,
            tsv_escape(&nhs_number.to_string()),
        ),
    }
}

fn emit_parse(line_number: LineIndex, line: &str, error: &str, format: Format) {
    match format {
        Format::Text => eprintln!(
            "{}",
            Error::Parse {
                line_number,
                line: line.to_string(),
                error: error.to_string(),
            }
        ),
        Format::Json => eprintln!(
            r#"{{"kind":"parse_error","line_number":{},"line":{},"error":{}}}"#,
            line_number,
            json_escape(line),
            json_escape(error),
        ),
        Format::Tsv => eprintln!(
            "parse_error\t{}\t\t{}\t{}",
            line_number,
            tsv_escape(line),
            tsv_escape(error),
        ),
    }
}

fn emit_io(line_number: LineIndex, error: &std::io::Error, format: Format) {
    let msg = format!("{}", error);
    match format {
        Format::Text => eprintln!(
            "{}",
            Error::Io {
                line_number,
                error: msg,
            }
        ),
        Format::Json => eprintln!(
            r#"{{"kind":"io_error","line_number":{},"error":{}}}"#,
            line_number,
            json_escape(&msg),
        ),
        Format::Tsv => eprintln!("io_error\t{}\t\t\t{}", line_number, tsv_escape(&msg),),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(
        "Error invalid line {line_number}. Error: validate check digit failed. NHS Number: {nhs_number}"
    )]
    CheckDigit {
        line_number: LineIndex,
        nhs_number: NHSNumber,
    },

    #[error("Error parsing line {line_number}. Error: {error}. Line: {line}")]
    Parse {
        line_number: LineIndex,
        line: String,
        error: String,
    },

    #[error("Error reading line {line_number}. Error: {error}")]
    Io {
        line_number: LineIndex,
        error: String,
    },
}
