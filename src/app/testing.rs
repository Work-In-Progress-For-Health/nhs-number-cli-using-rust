//! Testing helpers for the application.
//! 
//! - Path buffers to local directories.
//! - Command line interface file and name.
//! - And you can add more as you wish.

use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::LazyLock;

/// Path to the Cargo manifest directory.
#[allow(dead_code)]
pub static CARGO_MANIFEST_DIR: LazyLock<PathBuf> = LazyLock::new(||
    [env!("CARGO_MANIFEST_DIR")].iter().collect::<PathBuf>()
);

/// Path to the log directory.
#[allow(dead_code)]
pub static LOG_DIR: LazyLock<PathBuf> = LazyLock::new(||
    [env!("CARGO_MANIFEST_DIR"), "log"].iter().collect::<PathBuf>()
);

// Path to the tests directory.
#[allow(dead_code)]
pub static TESTS_DIR: LazyLock<PathBuf> = LazyLock::new(||
    [env!("CARGO_MANIFEST_DIR"), "tests"].iter().collect::<PathBuf>()
);

// Path to the tmp directory.
#[allow(dead_code)]
pub static TMP_DIR: LazyLock<PathBuf> = LazyLock::new(||
    [env!("CARGO_MANIFEST_DIR"), "tmp"].iter().collect::<PathBuf>()
);

// Path to the target directory.
#[allow(dead_code)]
pub static TARGET_DIR: LazyLock<PathBuf> = LazyLock::new(||
    [env!("CARGO_MANIFEST_DIR"), "target"].iter().collect::<PathBuf>()
);

// Path to the debug directory.
#[allow(dead_code)]
pub static DEBUG_DIR: LazyLock<PathBuf> = LazyLock::new(||
    [env!("CARGO_MANIFEST_DIR"), "target", "debug"].iter().collect::<PathBuf>()
);

// Path to the command file i.e. the runnable command line interface.
#[allow(dead_code)]
pub static COMMAND_FILE: LazyLock<PathBuf> = LazyLock::new(||
    [env!("CARGO_MANIFEST_DIR"), "target", "debug", "nhs-number-cli"].iter().collect::<PathBuf>()
);

// Path to the command operating system string i.e. the runnable command line interface file name.
#[allow(dead_code)]
pub static COMMAND_OS: LazyLock<OsString> = LazyLock::new(||
    OsString::from([env!("CARGO_MANIFEST_DIR"), "target", "debug", "nhs-number-cli"].iter().collect::<PathBuf>())
);
