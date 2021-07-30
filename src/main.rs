#![warn(future_incompatible)]
#![warn(missing_copy_implementations)]
#![warn(nonstandard_style)]
#![warn(rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::struct_excessive_bools)]

use std::env;
use std::io::{self, BufRead, Read};
use std::path::PathBuf;
use std::process::exit;

use ansi_term::Colour::{Red, Yellow};
use atty::Stream;
use docopt::Docopt;
use serde::Deserialize;
use unindent::unindent;

mod fs;
mod git;
mod repo;

use repo::RepositoryFilter;

fn get_program_name() -> Option<String> {
    env::current_exe()
        .ok()?
        .file_name()?
        .to_os_string()
        .into_string()
        .ok()
}

fn usage() -> String {
    let usage = format!(
        "
        Usage:
            {0} [options] ls
            {0} [options] run [ -- ] <command>...

        Find git repository paths starting from the current directory,
        or from stdin if supplied.

        Options:
            -d, --dirty          Include only dirty repos
            -u, --unpushed       Include only unpushed repos
            -x, --absolute       Use absolute paths
            -v, --verbose        Show a header for each repo when executing a command
            -i, --interactive    Pause between command executions
            --ignore-errors      Ignore errors when executing commands
            -h, --help           Show help",
        get_program_name().unwrap()
    );

    unindent(&usage)
}

fn parse_opts() -> Opts {
    if let Ok(opts) = Docopt::new(usage()).and_then(|a| a.deserialize()) {
        opts
    } else {
        println!("{}", usage());
        exit(1);
    }
}

fn pause() {
    let mut stdin = io::stdin();
    stdin.read_exact(&mut [0_u8]).unwrap();
}

#[derive(Debug, Deserialize)]
pub struct Opts {
    cmd_ls: bool,
    cmd_run: bool,
    arg_command: Vec<String>,
    flag_dirty: bool,
    flag_unpushed: bool,
    flag_absolute: bool,
    flag_verbose: bool,
    flag_interactive: bool,
    flag_ignore_errors: bool,
    flag_help: bool,
}

fn main() {
    let opts = parse_opts();
    let base_dir = env::current_dir().unwrap();

    let filter = RepositoryFilter {
        dirty: opts.flag_dirty,
        unpushed: opts.flag_unpushed,
    };

    let repos = if atty::is(Stream::Stdin) {
        repo::list_from_fs(&filter, &base_dir)
    } else {
        let stdin = io::stdin();
        let paths = stdin
            .lock()
            .lines()
            .map(|line| PathBuf::from(line.unwrap()))
            .collect();

        repo::list_from_vec(&filter, &base_dir, paths)
    };

    if opts.cmd_ls {
        for repo in &repos {
            println!("{}", repo.get_path(&base_dir, opts.flag_absolute).display());
        }
    }

    if opts.cmd_run {
        let mut first_iteration_done = false;

        for repo in &repos {
            if opts.flag_verbose {
                // Don't add an extra new line before the header for the
                // first item.
                // The enter key press from interactive mode does the
                // job of adding the new line for us.
                if first_iteration_done && !opts.flag_interactive {
                    println!();
                }
                first_iteration_done = true;
                println!(
                    "== {} ==",
                    Yellow.bold().paint(
                        repo.get_path(&base_dir, opts.flag_absolute)
                            .display()
                            .to_string()
                    )
                );
            }

            if !repo.run(&opts.arg_command) && !opts.flag_ignore_errors {
                println!();
                println!("{}", Red.paint("Exiting due to error when running command"));
                println!("{}", Red.paint("Run with --ignore-errors to bypass"));
                exit(1);
            }

            if opts.flag_interactive {
                pause();
            }
        }
    }
}
