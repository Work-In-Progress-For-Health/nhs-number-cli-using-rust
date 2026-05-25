use std::io::{self, BufRead};
use nhs_number::NHSNumber;
use std::str::FromStr;

/// Zero-based index of a line of input as it is read from `stdin`.
///
/// `Iterator::enumerate` produces `usize`, and that is what scripts
/// see on stderr too. Centralising the type here lets us widen or
/// narrow it in one place if we ever need to.
pub type LineIndex = usize;

pub fn check_lines() {
    let stdin = io::stdin();
    for (i, line) in stdin.lock().lines().enumerate() {
        match line {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }
                match NHSNumber::from_str(&line) {
                    Ok(nhs_number) => {
                        if nhs_number.validate_check_digit() {
                            println!("{}", nhs_number);
                        } else {
                            eprintln!("{}", Error::CheckDigit {
                                line_number: i,
                                nhs_number,
                            });
                        }
                    },
                    Err(e) => {
                        eprintln!("{}", Error::Parse {
                            line_number: i,
                            line: line.clone(),
                            error: e,
                        });
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", Error::Io {
                    line_number: i,
                    error: e,
                });
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error invalid line {line_number}. Error: validate check digit failed. NHS Number: {nhs_number}")]
    CheckDigit {
        line_number: LineIndex,
        nhs_number: NHSNumber,
    },

    #[error("Error parsing line {line_number}. Error: {error:?}. Line: {line}")]
    Parse {
        line_number: LineIndex,
        line: String,
        error: nhs_number::parse_error::ParseError,
    },

    #[error("Error reading line {line_number}. Error: {error}")]
    Io {
        line_number: LineIndex,
        error: std::io::Error,
    },
}
