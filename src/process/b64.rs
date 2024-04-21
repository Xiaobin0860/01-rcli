use base64::prelude::*;
use std::io::Read;

use crate::B64Format;

pub fn b64_encode(input: &str, format: B64Format) -> anyhow::Result<()> {
    let input = read_input(input)?;
    let encoded = match format {
        B64Format::Std => BASE64_STANDARD.encode(input),
        B64Format::Url => BASE64_URL_SAFE_NO_PAD.encode(input),
    };
    println!("{encoded}");
    Ok(())
}

pub fn b64_decode(input: &str, format: B64Format) -> anyhow::Result<()> {
    let input = read_input(input)?;
    // avoid accidental newlines
    let input = input.trim();
    let decoded = match format {
        B64Format::Std => BASE64_STANDARD.decode(input)?,
        B64Format::Url => BASE64_URL_SAFE_NO_PAD.decode(input)?,
    };
    // TODO: maybe not string
    let decoded = String::from_utf8(decoded)?;
    println!("{decoded}");
    Ok(())
}

fn read_input(input: &str) -> Result<String, anyhow::Error> {
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
