use clap::Parser;
use exitcode;
use tempfile::tempdir;

use std::io::{Error, ErrorKind};
use std::process::Command;

#[derive(Parser)]
struct Cli {
    // The repository to size
    repository: String,
}

fn command_exists(command: &str) -> bool {
    match Command::new(command).output() {
        Ok(_) => true,
        Err(e) => {
            if let ErrorKind::NotFound = e.kind() {
                return false;
            } else {
                panic!("Command {} returned an unexpected error: {}", command, e);
            }
        }
    }
}

fn run_git_sizer_on_repository(nwo: &str) -> Result<(), Error> {
    let temporary_directory = tempdir().expect("Failed to create temporary directory");
    let temporary_directory_path = temporary_directory.path().to_str().unwrap();

    eprintln!("Cloning {} from GitHub...", &nwo);

    let clone_command = Command::new("gh")
        .arg("repo")
        .arg("clone")
        .arg(&nwo)
        .arg(temporary_directory_path)
        .arg("--")
        .arg("--bare")
        .output()?;

    if !clone_command.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&clone_command.stderr));
        std::process::exit(exitcode::DATAERR);
    }

    eprintln!("Running git-sizer on cloned repository...");

    let sizer_command = Command::new("git-sizer")
        .arg("--verbose")
        .current_dir(temporary_directory_path)
        .output()?;

    if !sizer_command.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&sizer_command.stderr));
        std::process::exit(exitcode::DATAERR);
    }

    println!("{}", String::from_utf8_lossy(&sizer_command.stdout));

    Ok(())
}

fn main() {
    let args = Cli::parse();

    if !command_exists("gh") {
        eprintln!("`gh` not found. To use gh-sizer, please install the GitHub CLI (https://cli.github.com).");
        std::process::exit(exitcode::DATAERR);
    }

    if !command_exists("git-sizer") {
        eprintln!("`git-sizer` not found. To use gh-sizer, please install git-sizer (https://github.com/github/git-sizer).");
        std::process::exit(exitcode::DATAERR);
    }

    match run_git_sizer_on_repository(&args.repository) {
        Ok(_) => std::process::exit(exitcode::OK),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(exitcode::DATAERR);
        }
    }
}
