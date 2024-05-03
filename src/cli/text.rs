use super::{verify_file, verify_path};
use crate::{
    base64_encode, data_decrypt, data_encrypt, get_reader, key_gen, text_sign, text_verify,
    CmdExecutor,
};
use anyhow::Result;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::{fmt::Display, path::PathBuf, str::FromStr};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum TextSubCommand {
    #[command(about = "Sign a text with a private/session key and return a signature")]
    Sign(TextSignOpts),

    #[command(about = "Verify a signature with a public/session key")]
    Verify(TextVerifyOpts),

    #[command(about = "Generate a random blake3 key or ed25519 key pair")]
    Generate(KeyGenerateOpts),

    #[command(about = "Chacha20 encrypt a text with a key")]
    Encrypt(TextEncryptOpts),

    #[command(about = "Chacha20 decrypt a text with a key")]
    Decrypt(TextDecryptOpts),
}

#[derive(Debug, Parser)]
pub struct TextEncryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_chacha_key)]
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct TextDecryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_chacha_key)]
    pub key: String,
}

fn verify_chacha_key(s: &str) -> Result<String, &'static str> {
    if s.len() != 32 {
        return Err("key length must be 32 bytes");
    }
    Ok(s.to_string())
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,

    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,

    #[arg(short, long, value_parser = verify_file)]
    pub key: String,

    #[arg(long)]
    pub sig: String,
}

#[derive(Debug, Parser)]
pub struct KeyGenerateOpts {
    #[arg(short, long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,

    #[arg(short, long, value_parser = verify_path)]
    pub output_path: PathBuf,
}

#[derive(Debug, Parser, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_format(s: &str) -> Result<TextSignFormat, &'static str> {
    s.parse()
}

impl FromStr for TextSignFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err("invalid format"),
        }
    }
}

impl Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextSignFormat::Blake3 => write!(f, "blake3"),
            TextSignFormat::Ed25519 => write!(f, "ed25519"),
        }
    }
}

impl CmdExecutor for TextSignOpts {
    async fn execute(&self) -> Result<()> {
        let mut reader = get_reader(&self.input)?;
        let encoded = text_sign(self.format, &mut reader, &self.key)?;
        println!("{encoded}");
        Ok(())
    }
}

impl CmdExecutor for TextVerifyOpts {
    async fn execute(&self) -> Result<()> {
        let mut reader = get_reader(&self.input)?;
        let valid = text_verify(self.format, &mut reader, &self.key, &self.sig)?;
        println!("{valid}");
        Ok(())
    }
}

impl CmdExecutor for KeyGenerateOpts {
    async fn execute(&self) -> Result<()> {
        key_gen(self.format, &self.output_path)
    }
}

impl CmdExecutor for TextEncryptOpts {
    async fn execute(&self) -> Result<()> {
        let mut data_reader = get_reader(&self.input)?;
        let encrypted = data_encrypt(&mut data_reader, &self.key)?;
        let b64 = base64_encode(encrypted);
        println!("{b64}");
        Ok(())
    }
}

impl CmdExecutor for TextDecryptOpts {
    async fn execute(&self) -> Result<()> {
        let mut data_reader = get_reader(&self.input)?;
        let decrypted = data_decrypt(&mut data_reader, &self.key)?;
        let decrypted = String::from_utf8(decrypted)?;
        println!("{decrypted}");
        Ok(())
    }
}
