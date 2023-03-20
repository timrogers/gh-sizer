use std::io::Error;
use std::process::Command;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait GitHubRepositoryLister {
    fn call(&self, owner: &str) -> Result<Vec<String>, Error>;
}

pub struct GitHubRepositoryListerImpl;

impl GitHubRepositoryLister for GitHubRepositoryListerImpl {
    fn call(&self, owner: &str) -> Result<Vec<String>, Error> {
        let mut list_command = Command::new("gh");
        list_command.arg("repo");
        list_command.arg("list");
        list_command.arg(owner);
        list_command.arg("-L");
        list_command.arg("10000");
        list_command.arg("--json");
        list_command.arg("name");
        list_command.arg("--jq");
        list_command.arg(".[].name");

        let list_command_output = list_command.output()?;

        if !list_command_output.status.success() {
            eprintln!("{}", String::from_utf8_lossy(&list_command_output.stderr));
            std::process::exit(exitcode::DATAERR);
        }

        let output_text = String::from_utf8_lossy(&list_command_output.stdout);
        let output_lines: Vec<String> = output_text
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        Ok(output_lines)
    }
}
