use std::io::Read;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

use crate::{get_content, get_reader, TextSignFormat};

pub trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>>;
}

pub trait TextVerify {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> anyhow::Result<bool>;
}

pub struct Blake3Signer {
    key: [u8; 32],
}

impl Blake3Signer {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }
}

impl TextSign for Blake3Signer {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
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

    pub fn try_new(key: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }
}

impl TextVerify for Blake3Verifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        Ok(hash.as_bytes() == sig)
    }
}

pub fn text_sign(input: &str, key: &str) -> anyhow::Result<()> {
    let key = get_content(key)?;
    let mut reader = get_reader(input)?;
    let signer = Blake3Signer::try_new(key)?;
    let sig = signer.sign(&mut reader)?;
    let encoded = URL_SAFE_NO_PAD.encode(sig);
    println!("{encoded}");
    Ok(())
}

pub fn text_verify(input: &str, key: &str, sig: &str) -> anyhow::Result<()> {
    println!("Verify: input: {input}, key: {key}, sig: {sig}");
    let key = get_content(key)?;
    let sig = URL_SAFE_NO_PAD.decode(sig)?;
    let mut reader = get_reader(input)?;
    let verifier = Blake3Verifier::try_new(key)?;
    let valid = verifier.verify(&mut reader, &sig)?;
    println!("{valid}");
    Ok(())
}

pub fn key_gen(format: TextSignFormat, output_path: &str) -> anyhow::Result<()> {
    println!("Generate: format: {format}, output_path: {output_path}");
    Ok(())
}
