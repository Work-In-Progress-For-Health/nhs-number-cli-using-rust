pub(crate) mod args; // Arguments struct, such as set via `clap`.
pub(crate) mod clap; // Command line argument parser.
pub(crate) mod config; // Configuration struct, such as set via `confy`.
pub(crate) mod confy; // Configuration tests for loading and parsing.
pub(crate) mod run; // Run function that handles everything.
