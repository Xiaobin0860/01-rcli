use super::verify_file;
use crate::{b64_decode, b64_encode, CmdExecutor};
use anyhow::Result;
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum B64SubCommand {
    #[command(about = "Encode data to base64.")]
    Encode(B64EncodeOpts),

    #[command(about = "Decode base64 data.")]
    Decode(B64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct B64EncodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = parse_b64format, default_value = "std")]
    pub format: B64Format,
}

#[derive(Debug, Parser)]
pub struct B64DecodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = parse_b64format, default_value = "std")]
    pub format: B64Format,
}

#[derive(Debug, Parser, Clone, Copy)]
pub enum B64Format {
    Std,
    Url,
}

fn parse_b64format(s: &str) -> Result<B64Format, &'static str> {
    match s {
        "std" => Ok(B64Format::Std),
        "url" => Ok(B64Format::Url),
        _ => Err("invalid base64 format"),
    }
}

impl CmdExecutor for B64EncodeOpts {
    async fn execute(&self) -> Result<()> {
        b64_encode(&self.input, self.format)
    }
}

impl CmdExecutor for B64DecodeOpts {
    async fn execute(&self) -> Result<()> {
        b64_decode(&self.input, self.format)
    }
}
