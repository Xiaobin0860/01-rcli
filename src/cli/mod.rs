use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::{Path, PathBuf};

mod b64;
mod csv;
mod http;
mod jwt;
mod pass;
mod text;

pub use b64::*;
pub use csv::*;
pub use http::*;
pub use jwt::*;
pub use pass::*;
pub use text::*;

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
#[enum_dispatch(CmdExecutor)]
pub enum SubCommand {
    #[command(about = "Show CSV, or convert CSV to other formats.")]
    Csv(CsvOpts),

    #[command(about = "Generate password.")]
    Pass(PassOpts),

    #[command(subcommand, about = "Encode or decode data to/from base64.")]
    B64(B64SubCommand),

    #[command(subcommand, about = "Sign or verify text.")]
    Text(TextSubCommand),

    #[command(subcommand, about = "Send HTTP requests.")]
    Http(HttpSubCommand),

    #[command(subcommand, about = "Sign or verify JWT.")]
    Jwt(JwtSubCommand),
}

fn verify_file(input: &str) -> Result<String, &'static str> {
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

fn verify_path(input: &str) -> Result<PathBuf, &'static str> {
    let p: &Path = Path::new(input);
    if p.exists() && p.is_dir() {
        Ok(input.into())
    } else {
        Err("path not found or not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".to_string()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".to_string()));
        assert_eq!(verify_file("nonexistent"), Err("file not found"));
    }
}
