//! Crate-integration tests. Each `#[test]` spawns the compiled binary
//! via `std::process::Command`, pipes a fixed byte sequence into
//! stdin, and asserts the observable contract on stdout / stderr /
//! exit code.
//!
//! The numbered requirements that each test pins are noted in the
//! doc comment above the test. See `spec.md` for the full
//! specification and the traceability matrix in § 13.

use std::ffi::OsString;
use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::process::{Command, ExitStatus, Stdio};
use std::sync::LazyLock;

#[allow(dead_code)]
pub static COMMAND_OS: LazyLock<OsString> = LazyLock::new(|| {
    OsString::from(
        [
            env!("CARGO_MANIFEST_DIR"),
            "target",
            "debug",
            "nhs-number-cli",
        ]
        .iter()
        .collect::<PathBuf>(),
    )
});

/// Captured output of a single invocation of the binary.
struct Run {
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    status: ExitStatus,
}

impl Run {
    fn stdout_lines(&self) -> Vec<String> {
        self.stdout.lines().map(|l| l.unwrap()).collect()
    }
    fn stderr_lines(&self) -> Vec<String> {
        self.stderr.lines().map(|l| l.unwrap()).collect()
    }
    fn stderr_lossy(&self) -> std::borrow::Cow<'_, str> {
        String::from_utf8_lossy(&self.stderr)
    }
}

/// Spawn the binary with no args and pipe `input` to its stdin.
/// Accepts raw bytes so tests can feed invalid UTF-8 (needed for
/// FR-9).
fn run_with_stdin(input: &[u8]) -> Run {
    let mut command = Command::new(&*COMMAND_OS)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    if let Some(mut stdin) = command.stdin.take() {
        stdin.write_all(input).unwrap();
    }
    let output = command.wait_with_output().unwrap();
    Run {
        stdout: output.stdout,
        stderr: output.stderr,
        status: output.status,
    }
}

/// FR-3, FR-4, FR-5, FR-6, FR-10 — round-trip: one valid number to
/// stdout in canonical form; one failed-check-digit line to stderr
/// with the stable `Error invalid line N.` prefix.
#[test]
fn test() {
    let select = "999 999 9999";
    let reject = "999 123 4561";
    let input = format!("{}\n{}\n", select, reject);
    let run = run_with_stdin(input.as_bytes());
    let stdout_lines = run.stdout_lines();
    let stderr_lines = run.stderr_lines();
    assert_eq!(stdout_lines[0], select);
    assert!(stderr_lines[0].starts_with("Error invalid line 1."));
    assert!(stderr_lines[0].ends_with(reject));
}

/// FR-2 — blank lines are silently skipped. Mixed blank/valid input
/// should produce stdout in input order and an empty stderr.
#[test]
fn fr_2_blank_lines_skipped() {
    let run = run_with_stdin(b"\n999 999 9999\n\n\n999 000 0018\n\n");
    assert_eq!(run.stdout_lines(), vec!["999 999 9999", "999 000 0018"]);
    assert!(
        run.stderr.is_empty(),
        "stderr should be empty for blank-only noise; got {:?}",
        run.stderr_lossy(),
    );
}

/// FR-7 — a line that fails to parse produces exactly one stderr
/// diagnostic, beginning with the stable `Error parsing line N.`
/// prefix, and nothing on stdout for that line.
#[test]
fn fr_7_parse_failure_to_stderr() {
    let run = run_with_stdin(b"not-an-nhs-number\n");
    assert!(
        run.stdout.is_empty(),
        "stdout should be empty when only input is unparseable; got {:?}",
        run.stdout_lines(),
    );
    let stderr = run.stderr_lines();
    assert_eq!(
        stderr.len(),
        1,
        "expected exactly one diagnostic; got {stderr:?}"
    );
    assert!(stderr[0].starts_with("Error parsing line 0."));
    assert!(stderr[0].contains("not-an-nhs-number"));
}

/// FR-9 — a read error (invalid UTF-8 in the byte stream) is
/// reported on stderr with the stable `Error reading line N.` prefix,
/// and processing continues with the next line.
#[test]
fn fr_9_read_error_to_stderr() {
    let mut input: Vec<u8> = Vec::new();
    input.extend_from_slice(b"999 999 9999\n");
    input.extend_from_slice(&[0xFF, 0xFE, 0xFD, b'\n']); // invalid UTF-8
    input.extend_from_slice(b"999 000 0018\n");

    let run = run_with_stdin(&input);
    let stdout = String::from_utf8_lossy(&run.stdout);
    assert!(
        stdout.contains("999 999 9999") && stdout.contains("999 000 0018"),
        "processing should continue past a read error; stdout was: {stdout}",
    );
    let stderr = run.stderr_lossy();
    assert!(
        stderr.contains("Error reading line"),
        "expected 'Error reading line' on stderr; got: {stderr}",
    );
}

/// FR-11 — stdout and stderr are independent streams. Each is in
/// input order; interleaving between them is not specified, but the
/// invariant proven here is "valid lines never appear on stderr,
/// diagnostics never appear on stdout".
#[test]
fn fr_11_stream_separation() {
    let run = run_with_stdin(b"999 999 9999\n999 123 4561\n999 000 0018\n");
    assert_eq!(run.stdout_lines(), vec!["999 999 9999", "999 000 0018"]);
    let stderr = run.stderr_lines();
    assert_eq!(stderr.len(), 1);
    assert!(stderr[0].starts_with("Error invalid line 1."));
    assert!(stderr[0].ends_with("999 123 4561"));
}

/// FR-18 — `--column N` extracts the N-th comma-separated field
/// from each input line before parsing. Honoured by both subcommands;
/// a row with fewer fields than `N` produces a `ColumnMissing(N)`
/// parse error.
#[test]
fn fr_18_column_extraction() {
    let mut command = Command::new(&*COMMAND_OS)
        .args(["--column", "3"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    if let Some(mut stdin) = command.stdin.take() {
        stdin
            .write_all(b"1,Alice,999 999 9999\n2,Bob,999 123 4561\n3,short\n")
            .unwrap();
    }
    let output = command.wait_with_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert_eq!(stdout, "999 999 9999\n");
    assert!(
        stderr.contains("Error invalid line 1.") && stderr.contains("999 123 4561"),
        "stderr was: {stderr:?}"
    );
    assert!(
        stderr.contains("Error parsing line 2.") && stderr.contains("ColumnMissing(3)"),
        "stderr was: {stderr:?}"
    );
}

/// FR-19 — `--format json` emits NDJSON diagnostics on stderr.
/// Stdout (valid numbers) is unchanged.
#[test]
fn fr_19_format_json() {
    let mut command = Command::new(&*COMMAND_OS)
        .args(["--format", "json"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    if let Some(mut stdin) = command.stdin.take() {
        stdin
            .write_all(b"999 999 9999\n999 123 4561\nnot-an-nhs-number\n")
            .unwrap();
    }
    let output = command.wait_with_output().unwrap();
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "999 999 9999\n");
    let stderr = String::from_utf8(output.stderr).unwrap();
    let lines: Vec<&str> = stderr.lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(
        lines[0].starts_with(r#"{"kind":"check_digit","line_number":1"#)
            && lines[0].contains(r#""nhs_number":"999 123 4561""#),
        "got: {}",
        lines[0]
    );
    assert!(
        lines[1].starts_with(r#"{"kind":"parse_error","line_number":2"#)
            && lines[1].contains(r#""line":"not-an-nhs-number""#),
        "got: {}",
        lines[1]
    );
}

/// FR-19 — `--format tsv` emits five-column TSV rows on stderr.
#[test]
fn fr_19_format_tsv() {
    let mut command = Command::new(&*COMMAND_OS)
        .args(["--format", "tsv"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    if let Some(mut stdin) = command.stdin.take() {
        stdin
            .write_all(b"999 999 9999\n999 123 4561\nnot-an-nhs-number\n")
            .unwrap();
    }
    let output = command.wait_with_output().unwrap();
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "999 999 9999\n");
    let stderr = String::from_utf8(output.stderr).unwrap();
    let lines: Vec<&str> = stderr.lines().collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "check_digit\t1\t999 123 4561\t\t");
    assert_eq!(lines[1], "parse_error\t2\t\tnot-an-nhs-number\tParseError");
}

/// FR-19 — `--counts --format json` emits a single JSON object
/// summary on stdout.
#[test]
fn fr_19_counts_format_json() {
    let mut command = Command::new(&*COMMAND_OS)
        .args(["--counts", "--format", "json"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    if let Some(mut stdin) = command.stdin.take() {
        stdin
            .write_all(b"999 999 9999\n\n999 123 4561\nnot-an-nhs-number\n")
            .unwrap();
    }
    let output = command.wait_with_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        stdout.trim_end(),
        r#"{"valid":1,"invalid":1,"parse_error":1,"blank":1}"#
    );
    assert!(output.stderr.is_empty());
}

/// FR-19 (default) — without `--format`, the FR-10 text contract is
/// preserved byte-for-byte. Guards against accidental format-flag
/// leakage when `--format text` is the default.
#[test]
fn fr_19_default_text_unchanged() {
    let run = run_with_stdin(b"999 999 9999\n999 123 4561\n");
    assert_eq!(run.stdout_lines(), vec!["999 999 9999"]);
    let stderr = run.stderr_lines();
    assert_eq!(stderr.len(), 1);
    assert!(stderr[0].starts_with("Error invalid line 1."));
    assert!(stderr[0].ends_with("999 123 4561"));
}

/// FR-17 — `--counts` emits a four-row summary to stdout and
/// nothing to stderr. The default (no flag) and `--line-validation`
/// behaviour must be unchanged; only the explicit flag opts in.
#[test]
fn fr_17_counts_summary() {
    let mut command = Command::new(&*COMMAND_OS)
        .arg("--counts")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    if let Some(mut stdin) = command.stdin.take() {
        stdin
            .write_all(
                b"999 999 9999\n999 000 0069\n\n999 123 4561\nnot-an-nhs-number\n\n999 555 0016\n",
            )
            .unwrap();
    }
    let output = command.wait_with_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("valid:       3"), "stdout was: {stdout:?}");
    assert!(stdout.contains("invalid:     1"), "stdout was: {stdout:?}");
    assert!(stdout.contains("parse-error: 1"), "stdout was: {stdout:?}");
    assert!(stdout.contains("blank:       2"), "stdout was: {stdout:?}");
    assert!(
        output.stderr.is_empty(),
        "stderr should be empty in --counts mode; got {:?}",
        String::from_utf8_lossy(&output.stderr),
    );
    assert!(output.status.success());
}

/// FR-12 — exit code is `0` even when the input contains invalid
/// lines. Callers that need a non-zero exit on bad input wrap the
/// binary (see `examples/07-fail-on-invalid/`).
#[test]
fn fr_12_exit_zero_on_per_line_failures() {
    let run = run_with_stdin(b"999 123 4561\nnot-an-nhs-number\n");
    assert!(
        run.status.success(),
        "exit status should be success even with bad input; got {:?}",
        run.status,
    );
    assert_eq!(run.status.code(), Some(0));
}
