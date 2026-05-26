//! Run the app

use crate::app::args::Args;
use crate::app::config::Config;

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
    let args = initialize_arguments();
    dispatch(&args);
    Ok(())
}

/// Dispatch to a subcommand based on the parsed `Args`.
///
/// Default behaviour with no subcommand flag is the line validator,
/// so callers who do nothing more than `cat input.txt | nhs-number-cli`
/// keep working. Set `args.check_lines = Some(false)` to suppress it
/// (currently not expressible via the CLI).
fn dispatch(args: &Args) {
    trace!("dispatch");
    if args.check_lines.unwrap_or(true) {
        crate::subcommands::check_lines::check_lines();
    }
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
    #[error("Error loading configuration. Error: {0:?}")]
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

// cSpell:ignore confy
