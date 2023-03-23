#[cfg(feature = "windows_integration_tests")]
use std::process::Command;

#[cfg(feature = "windows_integration_tests")]
use tempfile::{Builder, NamedTempFile};

#[cfg(feature = "windows_integration_tests")]
use mockall::predicate::*;

#[cfg(feature = "windows_integration_tests")]
use std::io::Write;

#[cfg(feature = "windows_integration_tests")]
use assert_cmd::prelude::*;

#[cfg(feature = "windows_integration_tests")]
use predicates::prelude::*;

#[test]
#[cfg(feature = "windows_integration_tests")]
fn generate_script_command_errors_without_gh() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gh-sizer")?;

    cmd.arg("generate-script")
        .arg("gh-sizer-sandbox")
        .arg("--output-format")
        .arg("text")
        .arg("--output-directory")
        .arg("output\\directory")
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
#[cfg(feature = "windows_integration_tests")]
fn generate_script_command_errors_without_authenticated_gh_cli(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gh-sizer")?;

    cmd.arg("generate-script")
        .arg("gh-sizer-sandbox")
        .arg("--output-format")
        .arg("text")
        .arg("--output-directory")
        .arg("output\\directory")
        .arg("--output-filename")
        .arg("${repository}.txt")
        .env("GH_TOKEN", "foo");

    let output = cmd.output()?;

    assert!(!output.status.success());
    insta::assert_yaml_snapshot!(String::from_utf8_lossy(&output.stderr));

    Ok(())
}

#[test]
#[cfg(feature = "windows_integration_tests")]
fn generate_script_command_returns_bash_script_by_default() -> Result<(), Box<dyn std::error::Error>>
{
    let mut cmd = Command::cargo_bin("gh-sizer")?;

    cmd.arg("generate-script")
        .arg("gh-sizer-sandbox")
        .arg("--output-format")
        .arg("text")
        .arg("--output-directory")
        .arg("output\\directory")
        .arg("--output-filename")
        .arg("${repository}.txt");

    let output = cmd.output()?;

    assert!(output.status.success());
    insta::assert_yaml_snapshot!(String::from_utf8_lossy(&output.stdout));

    Ok(())
}

#[test]
#[cfg(feature = "windows_integration_tests")]
fn generate_script_command_returns_bash_script() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gh-sizer")?;

    cmd.arg("generate-script")
        .arg("gh-sizer-sandbox")
        .arg("--output-format")
        .arg("text")
        .arg("--output-directory")
        .arg("output\\directory")
        .arg("--output-filename")
        .arg("${repository}.txt")
        .arg("--script-type")
        .arg("bash");

    let output = cmd.output()?;

    assert!(output.status.success());
    insta::assert_yaml_snapshot!(String::from_utf8_lossy(&output.stdout));

    Ok(())
}

#[test]
#[cfg(feature = "windows_integration_tests")]
fn generate_script_command_returns_powershell_script() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gh-sizer")?;

    cmd.arg("generate-script")
        .arg("gh-sizer-sandbox")
        .arg("--output-format")
        .arg("text")
        .arg("--output-directory")
        .arg("output\\directory")
        .arg("--output-filename")
        .arg("${repository}.txt")
        .arg("--script-type")
        .arg("powershell");

    let output = cmd.output()?;

    assert!(output.status.success());
    insta::assert_yaml_snapshot!(String::from_utf8_lossy(&output.stdout));

    Ok(())
}

#[test]
#[cfg(feature = "windows_integration_tests")]
fn generate_script_command_returns_valid_powershell_script(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gh-sizer")?;

    cmd.arg("generate-script")
        .arg("gh-sizer-sandbox")
        .arg("--output-format")
        .arg("text")
        .arg("--output-directory")
        .arg("output\\directory")
        .arg("--output-filename")
        .arg("${repository}.txt")
        .arg("--script-type")
        .arg("powershell")
        .arg("--gh-sizer-command")
        .arg("cargo run --");

    let output = cmd.output()?;

    assert!(output.status.success());
    let generated_script = String::from_utf8_lossy(&output.stdout);

    println!("{}", generated_script);

    let mut script_file = Builder::new().suffix(".ps1").tempfile()?;

    write!(script_file, "{}", generated_script)?;

    let mut pwsh_command = Command::new("pwsh");
    pwsh_command.arg(script_file.path());

    let pwsh_command_output = pwsh_command.output()?;

    println!(
        "STDOUT: {}",
        String::from_utf8_lossy(&pwsh_command_output.stdout)
    );

    println!(
        "STDERR: {}",
        String::from_utf8_lossy(&pwsh_command_output.stderr)
    );

    assert!(pwsh_command_output.status.success());
    assert_eq!(String::from_utf8_lossy(&pwsh_command_output.stdout), "");

    insta::assert_yaml_snapshot!(String::from_utf8_lossy(&pwsh_command_output.stdout));

    Ok(())
}

#[test]
#[cfg(feature = "windows_integration_tests")]
fn generate_script_command_errors_when_output_filename_is_a_path(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gh-sizer")?;

    cmd.arg("generate-script")
        .arg("gh-sizer-sandbox")
        .arg("--output-format")
        .arg("text")
        .arg("--output-directory")
        .arg("output\\directory")
        .arg("--output-filename")
        .arg("foo/${repository}.txt");

    let output = cmd.output()?;

    assert!(!output.status.success());
    insta::assert_yaml_snapshot!(String::from_utf8_lossy(&output.stderr));

    Ok(())
}

#[test]
#[cfg(feature = "windows_integration_tests")]
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
#[cfg(feature = "windows_integration_tests")]
fn repo_command_errors_without_authenticated_gh_cli() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gh-sizer")?;

    cmd.arg("repo")
        .arg("gh-sizer-sandbox/first-repo")
        .env("GH_TOKEN", "foo");

    let output = cmd.output()?;

    assert!(!output.status.success());
    insta::assert_yaml_snapshot!(String::from_utf8_lossy(&output.stderr));

    Ok(())
}

#[test]
#[cfg(feature = "windows_integration_tests")]
fn repo_command_outputs_repo_size_in_text_to_stdout() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gh-sizer")?;

    cmd.arg("repo").arg("gh-sizer-sandbox/first-repo");

    let output = cmd.output()?;

    assert!(output.status.success());
    insta::assert_yaml_snapshot!(String::from_utf8_lossy(&output.stdout));

    Ok(())
}

#[test]
#[cfg(feature = "windows_integration_tests")]
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
