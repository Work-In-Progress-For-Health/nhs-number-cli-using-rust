use std::io::{self, BufRead};
use nhs_number::NHSNumber;
use std::str::FromStr;

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
                                line_number: i as i32,
                                nhs_number,
                            });
                        }
                    },
                    Err(e) => {
                        eprintln!("{}", Error::Parse {
                            line_number: i as i32,
                            line: line.clone(),
                            error: e,
                        });
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", Error::Io {
                    line_number: i as i32,
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
        line_number: i32,
        nhs_number: NHSNumber,
    },

    #[error("Error parsing line {line_number}. Error: {error:?}. Line: {line}")]
    Parse {
        line_number: i32,
        line: String,
        error: nhs_number::parse_error::ParseError,
    },

    #[error("Error reading line {line_number}. Error: {error}")]
    Io {
        line_number: i32,
        error: std::io::Error,
    },
}
