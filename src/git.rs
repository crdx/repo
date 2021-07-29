use std::io;
use std::path::Path;
use std::process::Command;

pub fn run_in_context(path: &Path, parts: &[String]) -> io::Result<bool> {
    let (command, args) = parts.split_first().unwrap();

    Ok(Command::new(command)
        .current_dir(path)
        .args(args)
        .spawn()?
        .wait()?
        .success())
}

pub fn is_actual(path: &Path) -> bool {
    let command = &["rev-parse", "--is-inside-git-dir"];
    stdout(&path.join(".git"), command) == b"true\n"
}

pub fn is_dirty(path: &Path) -> bool {
    let command = &["status", "--short"];
    !stdout(path, command).is_empty()
}

pub fn is_unpushed(path: &Path) -> bool {
    let command = &["log", "@{u}.."];
    !stdout(path, command).is_empty()
}

fn stdout(path: &Path, args: &[&str]) -> Vec<u8> {
    Command::new("git")
        .arg("-C")
        .arg(path)
        .args(args)
        .output()
        .unwrap()
        .stdout
}
