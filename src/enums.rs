use std::fmt;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Text,
    Json,
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OutputFormat::Text => write!(f, "text"),
            OutputFormat::Json => write!(f, "json"),
        }
    }
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ScriptType {
    Bash,
    Powershell,
}

impl fmt::Display for ScriptType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScriptType::Bash => write!(f, "bash"),
            ScriptType::Powershell => write!(f, "powershell"),
        }
    }
}
