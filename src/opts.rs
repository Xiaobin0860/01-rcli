use std::path::Path;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

impl Opts {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[command(about = "Show CSV, or convert CSV to other formats.")]
    Csv(CsvOpts),
}

#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")] // value.into()
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = false)]
    pub no_header: bool,
}

fn verify_input_file(input: &str) -> Result<String, &'static str> {
    if Path::new(input).exists() {
        Ok(input.to_string())
    } else {
        Err("file not found")
    }
}
