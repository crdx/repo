use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use tempfile::TempDir;

fn repo_binary() -> &'static str {
    env!("CARGO_BIN_EXE_repo")
}

fn initialise_repository(path: &Path) {
    fs::create_dir_all(path).unwrap();

    let output = Command::new("git")
        .arg("init")
        .arg("--quiet")
        .arg(path)
        .output()
        .unwrap();

    assert!(output.status.success());
}

fn run_repo(root: &Path, args: &[&str], stdin: Stdio) -> String {
    let output = Command::new(repo_binary())
        .current_dir(root)
        .args(args)
        .stdin(stdin)
        .output()
        .unwrap();

    assert!(output.status.success());

    String::from_utf8(output.stdout).unwrap()
}

fn sorted_lines(output: &str) -> Vec<String> {
    let mut lines: Vec<String> = output.lines().map(str::to_string).collect();
    lines.sort();
    lines
}

#[test]
fn scans_filesystem_when_stdin_is_empty() {
    let root = TempDir::new().unwrap();

    initialise_repository(&root.path().join("first"));
    initialise_repository(&root.path().join("second"));
    fs::create_dir(root.path().join("ordinary-directory")).unwrap();

    let output = run_repo(root.path(), &["ls"], Stdio::null());

    assert_eq!(sorted_lines(&output), ["first", "second"]);
}

#[test]
fn reads_repository_paths_from_stdin() {
    let root = TempDir::new().unwrap();
    let first = root.path().join("first");

    initialise_repository(&first);
    initialise_repository(&root.path().join("second"));

    let mut child = Command::new(repo_binary())
        .current_dir(root.path())
        .arg("ls")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"first\n")
        .unwrap();

    let output = child.wait_with_output().unwrap();

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), format!("{}\n", first.display()));
}

#[test]
fn dirty_filter_includes_only_dirty_repositories() {
    let root = TempDir::new().unwrap();

    initialise_repository(&root.path().join("clean"));
    initialise_repository(&root.path().join("dirty"));
    fs::write(root.path().join("dirty/file.txt"), "untracked\n").unwrap();

    let output = run_repo(root.path(), &["--dirty", "ls"], Stdio::null());

    assert_eq!(output, "dirty\n");
}
