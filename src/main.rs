use clap::{Parser, Subcommand};
use exitcode;
use tempfile::tempdir;

#[cfg(test)]
use tempfile::NamedTempFile;

#[cfg(test)]
use mockall::predicate::*;

#[cfg(test)]
use std::os::unix::fs::PermissionsExt;

#[cfg(test)]
use std::io::Write;

#[cfg(test)]
use std::fs;

use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::Command;

#[cfg(test)]
use assert_cmd::prelude::*;

#[cfg(test)]
use predicates::prelude::*;

use gh_sizer::enums::OutputFormat;
use gh_sizer::generate_script;
use gh_sizer::github_repository_lister::GitHubRepositoryListerImpl;

/// Run `git-sizer` on GitHub repositories without cloning each repository manually
#[derive(Debug, Parser)]
#[clap(name = "gh-sizer", version)]
struct Cli {
    // Test
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run `git-sizer` on a specific repo and output the results to stdout
    Repo {
        #[clap(
            help = "The owner and repository name of the repo to size, separated by a slash, e.g. `timrogers/gh-sizer`"
        )]
        repository: String,
        #[clap(value_enum, long, short, default_value_t = OutputFormat::Text, help = "The format to use for the output")]
        output_format: OutputFormat,
        // Hidden options are used for testing and may change between versions without notice.
        #[clap(long, hide = true, default_value = "gh")]
        gh_command: String,
    },
    /// Generate a Bash script to run `git-sizer` on all the repos owned by a user or organization and output the results to stdout or files
    GenerateScript {
        #[clap(
            help = "The owner of the repositories you want to size - either a user or an organization"
        )]
        owner: String,
        #[clap(value_enum, long, short = 'f', default_value_t = OutputFormat::Text, help = "The format to use for the output")]
        output_format: OutputFormat,
        #[clap(
            long,
            short = 'd',
            default_value = "output",
            help = "The directory to save the output files to"
        )]
        output_directory: String,
        #[clap(
            long,
            short = 'n',
            default_value = "${repository}.txt",
            help = "The filename to use for the output files. Use `${owner}` and `${repository}` to include the owner and repository name in the filename. This must be a filename, and cannot include a directory."
        )]
        output_filename: String,
        // Hidden options are used for testing and may change between versions without notice.
        #[clap(long, short = 'c', default_value = "gh sizer", hide = true)]
        gh_sizer_command: String,
        #[clap(long, hide = true, default_value = "gh", hide = true)]
        gh_command: String,
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

fn run_git_sizer_on_repository(nwo: &str, format: OutputFormat) -> Result<String, Error> {
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
        } => {
            if !command_exists(gh_command) {
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

            match run_git_sizer_on_repository(repository, output_format.to_owned()) {
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
            owner,
            output_format,
            output_directory,
            output_filename,
            gh_sizer_command,
            gh_command,
        } => {
            if !command_exists(gh_command) {
                eprintln!("`gh` not found. To use gh-sizer, please install the GitHub CLI (https://cli.github.com).");
                std::process::exit(exitcode::DATAERR);
            }

            if !command_succeeds(gh_command, vec!["auth".to_string(), "status".to_string()]) {
                eprintln!("You don't seem to be authenticated with the GitHub CLI, or your current access token is invalid. To authenticate, run `gh auth login`.");
                std::process::exit(exitcode::DATAERR);
            }

            if Path::new(output_filename).components().count() > 1 {
                eprintln!("--output-filename must be a filename, not a path");
                std::process::exit(exitcode::DATAERR);
            }

            match generate_script::call(
                owner,
                output_format.to_owned(),
                output_directory,
                output_filename,
                gh_sizer_command,
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
#[ignore]
fn generate_script_command_errors_without_gh() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gh-sizer")?;

    cmd.arg("generate-script")
        .arg("gh-sizer-sandbox")
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

#[test]
#[ignore]
fn generate_script_command_errors_without_authenticated_gh_cli(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gh-sizer")?;

    cmd.arg("generate-script")
        .arg("gh-sizer-sandbox")
        .arg("--output-format")
        .arg("text")
        .arg("--output-directory")
        .arg("output/directory")
        .arg("--output-filename")
        .arg("${repository}.txt")
        .env("GITHUB_TOKEN", "foo");

    let output = cmd.output()?;

    assert!(!output.status.success());
    insta::assert_yaml_snapshot!(String::from_utf8_lossy(&output.stderr));

    Ok(())
}

#[test]
#[ignore]
fn generate_script_command_returns_script() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gh-sizer")?;

    cmd.arg("generate-script")
        .arg("gh-sizer-sandbox")
        .arg("--output-format")
        .arg("text")
        .arg("--output-directory")
        .arg("output/directory")
        .arg("--output-filename")
        .arg("${repository}.txt");

    let output = cmd.output()?;

    assert!(output.status.success());
    insta::assert_yaml_snapshot!(String::from_utf8_lossy(&output.stdout));

    Ok(())
}

#[test]
#[ignore]
fn generate_script_returns_valid_script() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gh-sizer")?;

    cmd.arg("generate-script")
        .arg("gh-sizer-sandbox")
        .arg("--output-format")
        .arg("text")
        .arg("--output-directory")
        .arg("output/directory")
        .arg("--output-filename")
        .arg("${repository}.txt")
        .arg("--gh-sizer-command")
        .arg("cargo run --");

    let output = cmd.output()?;

    assert!(output.status.success());
    let generated_script = String::from_utf8_lossy(&output.stdout);

    println!("{}", generated_script);

    let mut script_file = NamedTempFile::new()?;
    write!(script_file, "{}", generated_script)?;
    fs::set_permissions(script_file.path(), fs::Permissions::from_mode(0o755))?;

    let mut bash_command = Command::new("bash");
    bash_command.arg(script_file.path());

    let bash_command_output = bash_command.output()?;

    assert!(bash_command_output.status.success());
    assert_eq!(String::from_utf8_lossy(&bash_command_output.stdout), "");

    insta::assert_yaml_snapshot!(String::from_utf8_lossy(&bash_command_output.stdout));

    Ok(())
}

#[test]
#[ignore]
fn generate_script_command_errors_when_output_filename_is_a_path(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gh-sizer")?;

    cmd.arg("generate-script")
        .arg("gh-sizer-sandbox")
        .arg("--output-format")
        .arg("text")
        .arg("--output-directory")
        .arg("output/directory")
        .arg("--output-filename")
        .arg("foo/${repository}.txt");

    let output = cmd.output()?;

    assert!(!output.status.success());
    insta::assert_yaml_snapshot!(String::from_utf8_lossy(&output.stderr));

    Ok(())
}

#[test]
#[ignore]
fn repo_command_errors_without_gh() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gh-sizer")?;

    cmd.arg("repo")
        .arg("gh-sizer-sandbox/first-repo")
        .arg("--gh-command")
        .arg("noop");

    cmd.assert().failure().stderr(predicate::str::contains(
        "`gh` not found. To use gh-sizer, please install the GitHub CLI (https://cli.github.com)",
    ));

    Ok(())
}

#[test]
#[ignore]
fn repo_command_errors_without_authenticated_gh_cli() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gh-sizer")?;

    cmd.arg("repo")
        .arg("gh-sizer-sandbox/first-repo")
        .env("GITHUB_TOKEN", "foo");

    let output = cmd.output()?;

    assert!(!output.status.success());
    insta::assert_yaml_snapshot!(String::from_utf8_lossy(&output.stderr));

    Ok(())
}

#[test]
#[ignore]
fn repo_command_outputs_repo_size_in_text_to_stdout() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gh-sizer")?;

    cmd.arg("repo").arg("gh-sizer-sandbox/first-repo");

    let output = cmd.output()?;

    assert!(output.status.success());
    insta::assert_yaml_snapshot!(String::from_utf8_lossy(&output.stdout));

    Ok(())
}

#[test]
#[ignore]
fn repo_command_outputs_repo_size_in_json_to_stdout() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gh-sizer")?;

    cmd.arg("repo")
        .arg("gh-sizer-sandbox/first-repo")
        .arg("--output-format")
        .arg("json");

    let output = cmd.output()?;

    assert!(output.status.success());
    insta::assert_yaml_snapshot!(String::from_utf8_lossy(&output.stdout));

    Ok(())
}
