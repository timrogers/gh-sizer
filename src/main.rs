use clap::{Parser, Subcommand};
use exitcode;
use tempfile::tempdir;

#[cfg(test)]
use mockall::{predicate::*};

use std::io::{Error, ErrorKind};
use std::process::Command;

#[cfg(test)]
use assert_cmd::prelude::*;

#[cfg(test)]
use predicates::prelude::*;

use gh_sizer::generate_script;
use gh_sizer::enums::Output;
use gh_sizer::enums::OutputFormat;
use gh_sizer::github_repository_lister::GitHubRepositoryListerImpl;

#[derive(Debug, Parser)]
#[clap(name = "gh-sizer", version)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Repo {
        repository: String,
        #[clap(value_enum, long, short, default_value_t = OutputFormat::Text)]
        output_format: OutputFormat,
        #[clap(long, short = 't')]
        github_token: Option<String>,
        #[clap(long, hide = true, default_value = "gh")]
        gh_command: String,
    },
    GenerateScript {
        organization: String,
        #[clap(value_enum, long, short = 'o', default_value_t = Output::File)]
        output: Output,
        #[clap(value_enum, long, short = 'f', default_value_t = OutputFormat::Text)]
        output_format: OutputFormat,
        #[clap(long, short = 'd', default_value = "output")]
        output_directory: String,
        #[clap(long, short = 'n', default_value = "${repository}.txt")]
        output_filename: String,
        #[clap(long, short = 'c', default_value = "gh sizer")]
        gh_sizer_command: String,
        #[clap(long, hide = true, default_value = "gh")]
        gh_command: String,
        #[clap(long, short = 't')]
        github_token: Option<String>,
    },
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

fn command_succeeds(command: &str, args: Vec<String>) -> bool {
    match Command::new(command).args(args).output() {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Command {} returned an unexpected error: {}", command, e);
            return false;
        }
    }
}

fn run_git_sizer_on_repository(
    nwo: &str,
    format: OutputFormat,
    github_token: Option<String>,
) -> Result<String, Error> {
    let temporary_directory = tempdir().expect("Failed to create temporary directory");
    let temporary_directory_path = temporary_directory.path().to_str().unwrap();

    eprintln!("Cloning {} from GitHub...", &nwo);

    let mut binding = Command::new("gh");
    let clone_command = binding
        .arg("repo")
        .arg("clone")
        .arg(&nwo)
        .arg(temporary_directory_path)
        .arg("--")
        .arg("--bare");

    if github_token.is_some() {
        clone_command.env("GITHUB_TOKEN", github_token.unwrap().to_string());
    }

    let clone_output = clone_command.output()?;

    if !clone_output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&clone_output.stderr));
        std::process::exit(exitcode::DATAERR);
    }

    eprintln!("Running git-sizer on cloned repository...");

    let mut sizer_command = Command::new("git-sizer");

    sizer_command.current_dir(temporary_directory_path);
    sizer_command.arg("--verbose");

    if matches!(format, OutputFormat::Json) {
        sizer_command.arg("--json");
    }

    let sizer_command_output = sizer_command.output()?;

    if !sizer_command_output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&sizer_command_output.stderr));
        std::process::exit(exitcode::DATAERR);
    }

    let output_text = String::from_utf8_lossy(&sizer_command_output.stdout);
    Ok(output_text.to_string())
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Repo {
            repository,
            output_format,
            gh_command,
            github_token,
        } => {
            if !command_exists("gh") {
                eprintln!("`gh` not found. To use gh-sizer, please install the GitHub CLI (https://cli.github.com).");
                std::process::exit(exitcode::DATAERR);
            }

            if !command_succeeds(gh_command, vec!["auth".to_string(), "status".to_string()]) {
                eprintln!("You don't seem to be authenticated with the GitHub CLI, or your current access token is invalid. To authenticate, run `gh auth login`.");
                std::process::exit(exitcode::DATAERR);
            }

            if !command_exists("git-sizer") {
                eprintln!("`git-sizer` not found. To use gh-sizer, please install git-sizer (https://github.com/github/git-sizer).");
                std::process::exit(exitcode::DATAERR);
            }

            match run_git_sizer_on_repository(
                repository,
                output_format.to_owned(),
                github_token.to_owned(),
            ) {
                Ok(output) => {
                    println!("{}", output);
                    std::process::exit(exitcode::OK);
                }
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(exitcode::DATAERR);
                }
            }
        }
        Commands::GenerateScript {
            organization,
            output,
            output_format,
            output_directory,
            output_filename,
            gh_sizer_command,
            gh_command,
            github_token,
        } => {
            if !command_exists(gh_command) {
                eprintln!("`gh` not found. To use gh-sizer, please install the GitHub CLI (https://cli.github.com).");
                std::process::exit(exitcode::DATAERR);
            }

            if !command_succeeds(gh_command, vec!["auth".to_string(), "status".to_string()]) {
                eprintln!("You don't seem to be authenticated with the GitHub CLI, or your current access token is invalid. To authenticate, run `gh auth login`.");
                std::process::exit(exitcode::DATAERR);
            }

            match generate_script::call(
                organization,
                output.to_owned(),
                output_format.to_owned(),
                output_directory,
                output_filename,
                gh_sizer_command,
                github_token.to_owned(),
                &GitHubRepositoryListerImpl {},
                &mut std::io::stdout(),
            ) {
                Ok(output) => {
                    println!("{}", output);
                    std::process::exit(exitcode::OK);
                }
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(exitcode::DATAERR);
                }
            }
        }
    };
}
#[test]
fn generate_script_command_errors_without_gh() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gh-sizer")?;

    cmd.arg("generate-script")
        .arg("github")
        .arg("--output")
        .arg("stdout")
        .arg("--output-format")
        .arg("text")
        .arg("--output-directory")
        .arg("output/directory")
        .arg("--output-filename")
        .arg("${repository}.txt")
        .arg("--gh-command")
        .arg("noop");

    cmd.assert().failure().stderr(predicate::str::contains(
        "`gh` not found. To use gh-sizer, please install the GitHub CLI (https://cli.github.com)",
    ));

    Ok(())
}
