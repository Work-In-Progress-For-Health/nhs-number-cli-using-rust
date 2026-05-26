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

use nhs_number::NHSNumber;
use std::io::{self, BufRead};
use std::str::FromStr;

pub fn counts() {
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
                match NHSNumber::from_str(&line) {
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
