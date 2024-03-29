#![warn(future_incompatible)]
#![warn(missing_copy_implementations)]
#![warn(nonstandard_style)]
#![warn(rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic)]

use std::env;
use std::io::{self, BufRead, Read};
use std::path::PathBuf;
use std::process::exit;

use ansi_term::Colour::{Red, Yellow};
use atty::Stream;
use docopt::Docopt;
use serde::Deserialize;
use unindent::unindent;

mod config;
mod fs;
mod git;
mod ignore;
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
            {0} [options] run [--] <command>...

        Find git repository paths starting from the current directory, or from stdin if supplied.
        Paths from stdin are always displayed as absolute paths.

        Options:
            -d, --dirty          Include only dirty repositories
            -u, --unpushed       Include only unpushed repositories
            -a, --all            Include ignored repositories
            -x, --absolute       Show absolute paths
            -v, --verbose        Show a header for each repository when executing a command
            -i, --interactive    Pause between command executions
            --ignore-errors      Ignore errors when executing commands
        ",
        get_program_name().unwrap()
    );

    unindent(usage.trim())
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

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Deserialize)]
pub struct Opts {
    cmd_ls: bool,
    cmd_run: bool,
    arg_command: Vec<String>,
    flag_dirty: bool,
    flag_unpushed: bool,
    flag_all: bool,
    flag_absolute: bool,
    flag_verbose: bool,
    flag_interactive: bool,
    flag_ignore_errors: bool,
}

fn main() {
    let opts = parse_opts();
    let base_dir = env::current_dir().unwrap();

    let filter = RepositoryFilter {
        dirty: opts.flag_dirty,
        unpushed: opts.flag_unpushed,
        ignorer: config::get_ignorer(opts.flag_all),
    };

    let paths_from_stdin = !atty::is(Stream::Stdin);

    let repos = if paths_from_stdin {
        let stdin = io::stdin();

        let paths = stdin
            .lock()
            .lines()
            .map(|line| PathBuf::from(line.unwrap()))
            .collect();

        repo::list_from_vec(&filter, &base_dir, paths)
    } else {
        repo::list_from_fs(&filter, &base_dir)
    };

    let show_absolute_paths = opts.flag_absolute || paths_from_stdin;

    if opts.cmd_ls {
        for repo in &repos {
            println!(
                "{}",
                repo.get_path(&base_dir, show_absolute_paths).display()
            );
        }
    }

    if opts.cmd_run {
        let mut first_iteration_done = false;

        for repo in &repos {
            if opts.flag_verbose {
                // Don't add an extra new line before the header for the first item.
                // The enter key press from interactive mode does the job of adding the new line for
                // us.
                if first_iteration_done && !opts.flag_interactive {
                    println!();
                }
                first_iteration_done = true;
                println!(
                    "== {} ==",
                    Yellow.bold().paint(
                        repo.get_path(&base_dir, show_absolute_paths)
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
