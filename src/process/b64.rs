use crate::B64Format;
use anyhow::Result;
use base64::prelude::*;
use std::io::Read;

pub fn base64_encode(input: impl AsRef<[u8]>) -> String {
    BASE64_STANDARD.encode(input)
}

pub fn base64_decode(input: &str) -> Result<Vec<u8>> {
    Ok(BASE64_STANDARD.decode(input)?)
}

pub fn b64_encode(input: impl AsRef<[u8]>, format: B64Format) -> Result<()> {
    let encoded = match format {
        B64Format::Std => base64_encode(input),
        B64Format::Url => BASE64_URL_SAFE_NO_PAD.encode(input),
    };
    println!("{encoded}");
    Ok(())
}

pub fn b64_decode(input: &str, format: B64Format) -> Result<()> {
    let input = read_input(input)?;
    // avoid accidental newlines
    let input = input.trim();
    let decoded = match format {
        B64Format::Std => base64_decode(input)?,
        B64Format::Url => BASE64_URL_SAFE_NO_PAD.decode(input)?,
    };
    // TODO: maybe not string
    let decoded = String::from_utf8(decoded)?;
    println!("{decoded}");
    Ok(())
}

fn read_input(input: &str) -> Result<String> {
    let input = if input == "-" {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else {
        // TODO: maybe not string
        std::fs::read_to_string(input)?
    };
    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b64_encode() {
        assert!(b64_encode("Cargo.toml", B64Format::Std).is_ok());
        assert!(b64_encode("Cargo.toml", B64Format::Url).is_ok());
    }

    #[test]
    fn test_b64_decode() {
        assert!(b64_decode("fixtures/std.txt", B64Format::Std).is_ok());
        assert!(b64_decode("fixtures/url.txt", B64Format::Url).is_ok());
    }
}
