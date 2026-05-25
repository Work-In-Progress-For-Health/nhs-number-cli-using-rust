use std::io::{self, BufRead};
use nhs_number::NHSNumber;
use std::str::FromStr;

pub fn check_lines() {
    let stdin = io::stdin();
    for (i, line) in stdin.lock().lines().enumerate() {
        match line {
            Ok(line) => {
                if line == "" {
                    continue;
                }
                match NHSNumber::from_str(&line) {
                    Ok(nhs_number) => {
                        match nhs_number.validate_check_digit() {
                            true => {
                                println!("{}", nhs_number)
                            },
                            false => {
                                let error = Error::CheckDigitError {
                                    line_number: i as i32,
                                    nhs_number,
                                };
                                eprintln!("{}", error)
                            }
                        }
                    },
                    Err(e) => {
                        let error = Error::ParseError {
                            line_number: i as i32,
                            line: line.clone(),
                            error: e,
                        };
                        eprintln!("{}", error);
                    }
                }
            }
            Err(e) => {
                let error = Error::IOError {
                    line_number: i as i32,
                    error: e,
                };
                eprintln!("{}", error);
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {

    #[error("Error invalid line {line_number}. Error: validate check digit failed. NHS Number: {nhs_number}")]
    CheckDigitError {
        line_number: i32,
        nhs_number: NHSNumber,
    },

    #[error("Error parsing line {line_number}. Error: {error:?}. Line: {line}")]
    ParseError {
        line_number: i32,
        line: String,
        error: nhs_number::parse_error::ParseError,
    },

    #[error("Error reading line {line_number}. Error: {error}")]
    IOError {
        line_number: i32,
        error: std::io::Error,
    },

}
