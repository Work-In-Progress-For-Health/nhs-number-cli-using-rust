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
/// keep working. Explicit flags pick a non-default subcommand:
/// `--counts` runs `subcommands::counts`, and `--line-validation` is
/// the explicit form of the default. The flags are mutually exclusive
/// at the clap layer; this function honours that by checking `counts`
/// first.
fn dispatch(args: &Args) {
    trace!("dispatch");
    if args.counts.unwrap_or(false) {
        crate::subcommands::counts::counts(args.column, args.format);
    } else if args.check_lines.unwrap_or(true) {
        crate::subcommands::check_lines::check_lines(args.column, args.format);
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

// cSpell:ignore confy
//
// `run()` itself is not unit-testable in isolation: it pulls
// process-level `stdin` via `confy::load` and `std::io::stdin()` and
// hands off to the subcommand dispatch. The end-to-end behaviour is
// exercised by `tests/test.rs` (which spawns the compiled binary as
// a subprocess) and by every `examples/*/run.sh`. Don't add an
// empty unit-test module here just to have one.
