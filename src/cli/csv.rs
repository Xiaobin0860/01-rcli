use std::fmt::Display;

use clap::Parser;

use super::verify_input_file;

#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long)] // value.into()
    pub output: Option<String>,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = false)]
    pub no_header: bool,

    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Yaml => write!(f, "yaml"),
        }
    }
}

fn parse_format(value: &str) -> Result<OutputFormat, &'static str> {
    match value {
        "json" => Ok(OutputFormat::Json),
        "yaml" => Ok(OutputFormat::Yaml),
        _ => Err("invalid format"),
    }
}
