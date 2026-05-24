//! Run the app

use std::path::PathBuf;
use crate::app::args::Args;
use crate::app::config::Config;
use crate::types::list::*;

/// Run everything.
///
/// Steps:
///
///   * Initialize configuration.
///
///   * Initialize arguments.
///
//    * Run the subcommands.
///
/// Example:
///
/// ```
/// run();
/// //-> Initialize everything then run the subcommands.
/// ```
///
pub(crate) fn run() -> Result<(), Error> {
    trace!("run");
    let _config = initialize_configuration()?;
    let _args = initialize_arguments();
    crate::subcommands::check_lines::check_lines();
    Ok(())
}

fn initialize_configuration() -> Result<Config, Error> {
    trace!("initialize_configuration");
    match confy::load("nhs-number-cli", None) {
        Ok(val) => Ok(val),
        Err(err) => Err(Error::Confy(err)),
    }
}

fn initialize_arguments() -> Args {
    trace!("initialize_arguments");
    let args: Args = crate::app::clap::args();
    if args.test { 
        println!("{:?}", args);
        println!("log level: {:?}", args.log_level); 
    }
    args
}

#[derive(thiserror::Error, Debug)]
pub enum Error {

    #[error("ParseError ➡ {string:?}")]
    ParseError {
        string: String
    },

    #[error("Confy ➡ {0:?}")]
    Confy(::confy::ConfyError),
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_run() {
        //TODO
    }

}

// cSpell:ignore walkdir
