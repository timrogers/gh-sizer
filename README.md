# gh sizer

`gh sizer` is an extension for the [GitHub CLI](https://cli.github.com/) which makes it easy to size one or more GitHub repos with [`git-sizer`](https://github.com/github/git-sizer) without having to clone the repo(s) manually.

## Prerequisites

To use `gh sizer`, you must already have the following dependencies available in your `$PATH`:

* the [GitHub CLI](https://cli.github.com/), `gh`, installed using the instructions [here](https://github.com/cli/cli#installation)
* [`git-sizer`](https://github.com/github/git-sizer)

Before using `gh sizer`, you should log in to the GitHub CLI by running `gh auth login`.

## Installation

`gh sizer` is distributed as an extension for the GitHub CLI. Once you've installed the GitHub CLI and logged in, you can install the extension with a single command:

```bash
gh extension install timrogers/gh-sizer
```

## Usage

### Sizing a single repo

To size a single repo, use the `gh sizer repo` command:

```bash
gh sizer repo timrogers/gh-sizer
```

The repo will be automatically cloned and sized, and the output from `git-sizer` will be printed to STDOUT. Afterwards, the cloned repo will be automatically deleted.

By default, the `git-sizer` output will be in human-readable text format. For a machine-readable JSON output, specify the `--output-format json` option:

```bash
gh sizer repo timrogers/gh-sizer --output-format json
```

### Generating a script to size multiple repos

The `gh sizer generate-script` command allows you to generate a Bash or PowerShell script to size all repos belonging to a specific user or organization. 

#### Generating a Bash script

By default, the `generate-script` command generates a Bash script. 

Just run the following command, replacing `gh-sizer-sandbox` with the user or organization whose repos you want to size:

```bash
gh sizer generate-script gh-sizer-sandbox
```

The command will print a Bash script to STDOUT which will size all of the repos belonging to `gh-sizer-sandbox` and store the results in the `output/` directory, with one file per repo named `${repository}.txt`.

You can store the generated script by piping the output to a file. To execute the script, just make that file executable and then run it:

```bash
gh sizer generate-script gh-sizer-sandbox > script.sh
chmod +x script.sh
./script.sh
```

### Generating a PowerShell script

You can ask the `generate-script` command to generate a PowerShell script with the `--script-type powershell` argument.

Just run the following command, replacing `gh-sizer-sandbox` with the user or organization whose repos you want to size:

```pwsh
gh sizer generate-script gh-sizer-sandbox --script-type powershell
```

The command will print a PowerShell script to STDOUT which will size all of the repos belonging to `gh-sizer-sandbox` and store the results in the `output/` directory, with one file per repo named `${repository}.txt`.

You can store the generated script by piping the output to a file, and then execute it.

```pwsh
gh sizer generate-script gh-sizer-sandbox >> script.ps1
.\script.ps1
```

### Customizing your script

By default, `gh-sizer` will run output one `.txt` file for each repo to `output/${repository.txt}`.

For machine-readable JSON output, specify the `--output-format json` option:

```bash
gh sizer generate-script gh-sizer-sandbox --output-format json
```

You can customise the directory where the script saves its output and the filenames used with the `--output-directory` and `--output-filename` arguments. In the `--output-filename` argument, you can use `${owner}` and `${repository}` as placeholders for the owner and repository name:

```bash
gh sizer generate-script gh-sizer-sandbox --output-directory results --output-filename "${owner}-${repository}.txt"
```
