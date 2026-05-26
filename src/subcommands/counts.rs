//! The `--counts` subcommand.
//!
//! Read NHS Numbers one per line from `stdin`, classify each line,
//! and at EOF print a four-row summary to `stdout`:
//!
//! ```text
//! valid:       <n>
//! invalid:     <n>
//! parse-error: <n>
//! blank:       <n>
//! ```
//!
//! "Invalid" means the line parsed as an NHS Number but failed the
//! Modulus 11 check digit. "Parse-error" means the line could not
//! be parsed as ten digits at all. Read errors are counted under
//! parse-error.
//!
//! Honours the optional `column` argument the same way that
//! `check_lines` does: when `Some(n)`, each non-blank line is split
//! on `,` and the n-th (1-based) field is the candidate.

use crate::subcommands::pick_column;
use nhs_number::NHSNumber;
use std::io::{self, BufRead};
use std::str::FromStr;

pub fn counts(column: Option<usize>) {
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
    println!("valid:       {}", valid);
    println!("invalid:     {}", invalid);
    println!("parse-error: {}", parse_error);
    println!("blank:       {}", blank);
}
