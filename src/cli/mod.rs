use std::path::Path;

use clap::Parser;

mod csv;
pub use csv::*;

mod pass;
pub use pass::*;

mod b64;
pub use b64::*;

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

    #[command(about = "Generate password.")]
    Pass(PassOpts),

    #[command(about = "Encode or decode data to/from base64.")]
    B64(B64Opts),
}

fn verify_input_file(input: &str) -> Result<String, &'static str> {
    match input {
        "-" => Ok(input.to_string()),
        _ => {
            if Path::new(input).exists() {
                Ok(input.to_string())
            } else {
                Err("file not found")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".to_string()));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".to_string()));
        assert_eq!(verify_input_file("nonexistent"), Err("file not found"));
    }
}
