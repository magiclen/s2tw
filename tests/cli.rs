use std::{
    fs,
    io::Write,
    process::{Command, Stdio},
    sync::Once,
};

use tempfile::TempDir;

/// Build a command running the executable, after making sure its dictionaries have been generated.
fn s2tw() -> Command {
    static INIT: Once = Once::new();

    // The dictionaries live in a shared directory, so the very first run must not race with others.
    INIT.call_once(|| {
        let status = Command::new(env!("CARGO_BIN_EXE_s2tw"))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .status()
            .unwrap();

        assert!(status.success());
    });

    Command::new(env!("CARGO_BIN_EXE_s2tw"))
}

fn convert_stdin(input: &str) -> String {
    let mut child = s2tw().stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().unwrap();

    child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();

    let output = child.wait_with_output().unwrap();

    assert!(output.status.success());

    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn stdin_multiple_lines() {
    assert_eq!("測試字串\n繁體中文\n", convert_stdin("测试字符串\n繁体中文\n"));
}

#[test]
fn stdin_last_line_without_newline() {
    assert_eq!("測試字串\n", convert_stdin("测试字符串"));
}

#[test]
fn file_with_explicit_output_path() {
    let temporary_dir = TempDir::new().unwrap();

    let s_path = temporary_dir.path().join("input.txt");
    let tw_path = temporary_dir.path().join("output.txt");

    fs::write(s_path.as_path(), "测试字符串\n繁体中文\n").unwrap();

    let status = s2tw().arg(s_path.as_path()).arg(tw_path.as_path()).status().unwrap();

    assert!(status.success());
    assert_eq!("測試字串\n繁體中文\n", fs::read_to_string(tw_path.as_path()).unwrap());
}

#[test]
fn file_with_derived_output_path() {
    let temporary_dir = TempDir::new().unwrap();

    let s_path = temporary_dir.path().join("a.chs.txt");

    fs::write(s_path.as_path(), "测试字符串\n").unwrap();

    let status = s2tw().arg(s_path.as_path()).status().unwrap();

    assert!(status.success());
    assert_eq!("測試字串\n", fs::read_to_string(temporary_dir.path().join("a.cht.txt")).unwrap());
}
