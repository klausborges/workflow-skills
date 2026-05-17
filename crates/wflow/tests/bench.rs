use std::{
    fs,
    io::Write,
    os::unix::fs::PermissionsExt,
    process::{Command, Output, Stdio},
};

mod support;

use support::{Fixture, stderr};

#[test]
fn bench_count_reports_json_file_and_totals() {
    let fixture = Fixture::valid();
    fixture.write("a.md", "hello\nworld\n");
    fixture.write("b.md", "hello");

    let output = fixture.run(&["bench", "count", "--json", "a.md", "b.md"]);
    assert!(output.status.success(), "stderr:\n{}", stderr(&output));

    let json: serde_json::Value = serde_json::from_str(&stdout(&output)).expect("json output");
    assert_eq!(json["encoding"], "o200k_base");
    assert_eq!(json["files"].as_array().expect("files").len(), 2);
    assert_eq!(json["total"]["lines"], 3);
    assert_eq!(json["total"]["bytes"], 17);
    assert_eq!(json["total"]["tokens"], 5);
}

#[test]
fn bench_count_accepts_files_from_stdin_and_deduplicates() {
    let fixture = Fixture::valid();
    fixture.write("a.md", "hello\n");

    let output = run_with_stdin(
        &fixture,
        &["bench", "count", "--json", "a.md", "--files-from", "-"],
        "a.md\n\n",
    );
    assert!(output.status.success(), "stderr:\n{}", stderr(&output));

    let json: serde_json::Value = serde_json::from_str(&stdout(&output)).expect("json output");
    assert_eq!(json["files"].as_array().expect("files").len(), 1);
    assert_eq!(json["total"]["lines"], 1);
    assert_eq!(json["total"]["bytes"], 6);
}

#[test]
fn bench_count_reports_files_from_path_read_errors_with_context() {
    let fixture = Fixture::valid();

    fixture.assert_err_contains(
        &["bench", "count", "--files-from", "missing-list.txt"],
        "failed to read --files-from missing-list.txt",
    );
}

#[test]
fn bench_count_supports_cl100k_base_encoding() {
    let fixture = Fixture::valid();
    fixture.write("a.md", "hello\n");

    let output = fixture.run(&[
        "bench",
        "count",
        "--json",
        "--encoding",
        "cl100k_base",
        "a.md",
    ]);
    assert!(output.status.success(), "stderr:\n{}", stderr(&output));

    let json: serde_json::Value = serde_json::from_str(&stdout(&output)).expect("json output");
    assert_eq!(json["encoding"], "cl100k_base");
    assert_eq!(json["total"]["tokens"], 2);
}

#[test]
fn bench_count_treats_special_token_sentinels_as_ordinary_text() {
    let fixture = Fixture::valid();
    fixture.write("a.md", "<|endoftext|>");

    let output = fixture.run(&["bench", "count", "--json", "a.md"]);
    assert!(output.status.success(), "stderr:\n{}", stderr(&output));

    let json: serde_json::Value = serde_json::from_str(&stdout(&output)).expect("json output");
    assert_eq!(json["total"]["tokens"], 7);
}

#[test]
fn bench_count_rejects_directories_and_missing_files() {
    let fixture = Fixture::valid();
    fixture.write("a.md", "hello\n");

    let output = fixture.run(&["bench", "count", "skills", "missing.md"]);
    let stderr = stderr(&output);
    assert!(!output.status.success(), "expected failure");
    assert!(stderr.contains("benchmark path is a directory"), "{stderr}");
    assert!(
        stderr.contains("cannot canonicalize benchmark path"),
        "{stderr}"
    );
}

#[test]
fn bench_count_rejects_invalid_utf8() {
    let fixture = Fixture::valid();
    let path = fixture.path("bad.md");
    fs::write(path, [0xff, 0xfe]).expect("write invalid utf8");

    fixture.assert_err_contains(&["bench", "count", "bad.md"], "cannot read benchmark file");
}

#[test]
fn bench_count_rejects_unreadable_files() {
    let fixture = Fixture::valid();
    let path = fixture.path("unreadable.md");
    fs::write(&path, "secret\n").expect("write unreadable file");
    let mut permissions = fs::metadata(&path).expect("metadata").permissions();
    permissions.set_mode(0o000);
    fs::set_permissions(&path, permissions).expect("set unreadable permissions");

    fixture.assert_err_contains(
        &["bench", "count", "unreadable.md"],
        "cannot read benchmark file",
    );

    let mut permissions = fs::metadata(&path).expect("metadata").permissions();
    permissions.set_mode(0o644);
    fs::set_permissions(&path, permissions).expect("restore permissions");
}

fn run_with_stdin(fixture: &Fixture, args: &[&str], stdin: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_wflow"))
        .arg("--root")
        .arg(fixture.root())
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn wflow");

    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(stdin.as_bytes())
        .expect("write stdin");

    child.wait_with_output().expect("run wflow")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}
