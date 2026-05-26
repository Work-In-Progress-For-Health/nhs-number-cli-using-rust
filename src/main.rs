//! # NHS Number command line interface (cli)
//!
//! A National Health Service (NHS) Number is a unique ten-digit identifier
//! allocated to registered patients of NHS England and the NHS Isle of Man.
//!
//! This tool is a command line interface that parses each standard input line
//! into an NHS Number, then validates that the check digit is correct.
//!
//! References:
//!
//! * [National Health Service (NHS)](https://en.wikipedia.org/wiki/National_Health_Service)
//!
//! * [NHS Number](https://en.wikipedia.org/wiki/NHS_number)
//!

mod app;
mod subcommands;
mod types;

#[cfg(test)]
mod testing;

#[macro_use]
extern crate log;
extern crate assertables;
extern crate env_logger;

fn main() {
    env_logger::init();
    match crate::app::run::run() {
        Ok(()) => {
            std::process::exit(0);
        }
        Err(err) => {
            error!("{:?}", err);
            std::process::exit(1);
        }
    }
}
