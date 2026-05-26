//! The `--counts` subcommand.
//!
//! Read NHS Numbers one per line from `stdin`, classify each line,
//! and at EOF print a summary on `stdout`.
//!
//! The text format (default) is four key/value rows:
//!
//! ```text
//! valid:       <n>
//! invalid:     <n>
//! parse-error: <n>
//! blank:       <n>
//! ```
//!
//! With `--format json` the summary is a single JSON object:
//!
//! ```text
//! {"valid":3,"invalid":1,"parse_error":1,"blank":2}
//! ```
//!
//! With `--format tsv` the summary is a header row followed by a
//! row of integer counts.

use crate::app::args::Format;
use crate::subcommands::pick_column;
use nhs_number::NHSNumber;
use std::io::{self, BufRead};
use std::str::FromStr;

pub fn counts(column: Option<usize>, format: Format) {
    let stdin = io::stdin();
    let mut valid: usize = 0;
    let mut invalid: usize = 0;
    let mut parse_error: usize = 0;
    let mut blank: usize = 0;
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                if line.is_empty() {
                    blank += 1;
                    continue;
                }
                let candidate = match column {
                    Some(n) => match pick_column(&line, n) {
                        Some(s) => s,
                        None => {
                            parse_error += 1;
                            continue;
                        }
                    },
                    None => &line,
                };
                match NHSNumber::from_str(candidate) {
                    Ok(nhs_number) => {
                        if nhs_number.validate_check_digit() {
                            valid += 1;
                        } else {
                            invalid += 1;
                        }
                    }
                    Err(_) => {
                        parse_error += 1;
                    }
                }
            }
            Err(_) => {
                parse_error += 1;
            }
        }
    }
    match format {
        Format::Text => {
            println!("valid:       {}", valid);
            println!("invalid:     {}", invalid);
            println!("parse-error: {}", parse_error);
            println!("blank:       {}", blank);
        }
        Format::Json => {
            println!(
                r#"{{"valid":{},"invalid":{},"parse_error":{},"blank":{}}}"#,
                valid, invalid, parse_error, blank,
            );
        }
        Format::Tsv => {
            println!("valid\tinvalid\tparse-error\tblank");
            println!("{}\t{}\t{}\t{}", valid, invalid, parse_error, blank);
        }
    }
}
