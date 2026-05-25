use std::ffi::OsString;
use std::path::PathBuf;
use std::io::{BufRead, Write};
use std::sync::LazyLock;
use std::process::{Command, Stdio};

#[allow(dead_code)]
pub static COMMAND_OS: LazyLock<OsString> = LazyLock::new(|| {
    OsString::from(
        [env!("CARGO_MANIFEST_DIR"), "target", "debug", "nhs-number-cli"]
            .iter()
            .collect::<PathBuf>(),
    )
});

#[test]
fn test() {
    let mut command = Command::new(&*COMMAND_OS)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let select = "999 999 9999";
    let reject = "999 123 4561";
    let input = format!("{}\n{}\n", select, reject);
    if let Some(mut stdin) = command.stdin.take() {
        stdin.write_all(input.as_ref()).unwrap();
    }
    let output = command.wait_with_output().unwrap();
    let stdout_lines: Vec<_> = output.stdout.lines()
        .map(|line| line.unwrap())
        .collect();
    let stderr_lines: Vec<_> = output.stderr.lines()
        .map(|line| line.unwrap())
        .collect();
    assert_eq!(stdout_lines[0], select);
    assert!(stderr_lines[0].ends_with(reject));
}
