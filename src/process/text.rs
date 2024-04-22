use std::{fs, io::Read, path::Path};

use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{
    Signature, Signer, SigningKey, Verifier, VerifyingKey, PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH,
};
use rand::rngs::OsRng;

use crate::{gen_pass, get_content, get_data, TextSignFormat};

pub trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool>;
}

pub struct Blake3Signer {
    key: [u8; 32],
}

impl Blake3Signer {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }
}

impl TextSign for Blake3Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        Ok(hash.as_bytes().to_vec())
    }
}

pub struct Blake3Verifier {
    key: [u8; 32],
}

impl Blake3Verifier {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }
}

impl TextVerify for Blake3Verifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        Ok(hash.as_bytes() == sig)
    }
}

struct Ed25519Signer {
    key: SigningKey,
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..SECRET_KEY_LENGTH]).try_into()?;
        let key = SigningKey::from_bytes(key);
        Ok(Self::new(key))
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}

struct Ed25519Verifier {
    key: VerifyingKey,
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..PUBLIC_KEY_LENGTH]).try_into()?;
        let key = VerifyingKey::from_bytes(key)?;
        Ok(Self::new(key))
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::try_from(sig)?;
        Ok(self.key.verify(&buf, &sig).is_ok())
    }
}

pub fn text_sign(format: TextSignFormat, reader: &mut dyn Read, key: &str) -> Result<String> {
    let signer: Box<dyn TextSign> = match format {
        TextSignFormat::Blake3 => {
            let key = get_content(key)?;
            Box::new(Blake3Signer::try_new(key)?)
        }
        TextSignFormat::Ed25519 => {
            let key = get_data(key)?;
            Box::new(Ed25519Signer::try_new(key)?)
        }
    };
    let sig = signer.sign(reader)?;
    let encoded = URL_SAFE_NO_PAD.encode(sig);
    Ok(encoded)
}

pub fn text_verify(
    format: TextSignFormat,
    reader: &mut dyn Read,
    key: &str,
    sig: &str,
) -> Result<bool> {
    let sig = URL_SAFE_NO_PAD.decode(sig)?;
    let verifier: Box<dyn TextVerify> = match format {
        TextSignFormat::Blake3 => {
            let key = get_content(key)?;
            Box::new(Blake3Verifier::try_new(key)?)
        }
        TextSignFormat::Ed25519 => {
            let key = get_data(key)?;
            Box::new(Ed25519Verifier::try_new(key)?)
        }
    };
    verifier.verify(reader, &sig)
}

pub fn key_gen(format: TextSignFormat, output_path: &str) -> Result<()> {
    println!("Generate: format: {format}, output_path: {output_path}");
    let p = Path::new(output_path);
    match format {
        TextSignFormat::Blake3 => {
            let key = gen_pass(32, false, false, false, false)?;
            fs::write(p.join("blake3.txt"), key)?;
        }
        TextSignFormat::Ed25519 => {
            let mut csprng = OsRng;
            let signing_key = SigningKey::generate(&mut csprng);
            let verifying_key = signing_key.verifying_key();
            fs::write(p.join("ed25519.sign"), signing_key.to_bytes())?;
            fs::write(p.join("ed25519.verify"), verifying_key.to_bytes())?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3() {
        let mut reader = "hello".as_bytes();
        let encoded =
            text_sign(TextSignFormat::Blake3, &mut reader, "fixtures/blake3.txt").unwrap();
        let mut reader = "hello".as_bytes();
        let valid = text_verify(
            TextSignFormat::Blake3,
            &mut reader,
            "fixtures/blake3.txt",
            &encoded,
        )
        .unwrap();
        assert!(valid);
    }

    #[test]
    fn test_ed25519() {
        let mut reader = "hello".as_bytes();
        let encoded = text_sign(
            TextSignFormat::Ed25519,
            &mut reader,
            "fixtures/ed25519.sign",
        )
        .unwrap();
        let mut reader = "hello".as_bytes();
        let valid = text_verify(
            TextSignFormat::Ed25519,
            &mut reader,
            "fixtures/ed25519.verify",
            &encoded,
        )
        .unwrap();
        assert!(valid);
    }
}
