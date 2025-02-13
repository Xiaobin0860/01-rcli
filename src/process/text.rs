use std::{fs, io::Read, ops::Deref, path::Path};

use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chacha20poly1305::{
    aead::{Aead, AeadCore},
    ChaCha20Poly1305, ChaChaPoly1305, KeyInit, Nonce,
};
use ed25519_dalek::{
    Signature, Signer, SigningKey, Verifier, VerifyingKey, PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH,
};
use rand::rngs::OsRng;

use crate::{base64_decode, gen_pass, get_content, get_data, TextSignFormat};

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

pub fn key_gen(format: TextSignFormat, output_path: &Path) -> Result<()> {
    match format {
        TextSignFormat::Blake3 => {
            let key = gen_pass(32, false, false, false, false)?;
            fs::write(output_path.join("blake3.txt"), key)?;
        }
        TextSignFormat::Ed25519 => {
            let mut os_rng = OsRng;
            let signing_key = SigningKey::generate(&mut os_rng);
            let verifying_key = signing_key.verifying_key();
            fs::write(output_path.join("ed25519.sign"), signing_key.to_bytes())?;
            fs::write(output_path.join("ed25519.verify"), verifying_key.to_bytes())?;
        }
    }
    Ok(())
}

pub trait DataEncrypt {
    fn data_encrypt(&self, data: &[u8]) -> Result<Vec<u8>>;
}

pub trait DataDecrypt {
    fn data_decrypt(&self, data: &[u8]) -> Result<Vec<u8>>;
}

struct ChaCha(ChaCha20Poly1305);

impl ChaCha {
    pub fn try_new(key: &[u8]) -> Result<Self> {
        Ok(Self(ChaChaPoly1305::new_from_slice(key)?))
    }
}

impl Deref for ChaCha {
    type Target = ChaCha20Poly1305;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DataEncrypt for ChaCha {
    fn data_encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let mut buf = nonce.to_vec();
        eprintln!("nonce_len={}", nonce.len());
        let enc = self.encrypt(&nonce, data).unwrap();
        eprintln!("enc_len={}", enc.len());
        buf.extend(enc);
        eprintln!("total_len={}", buf.len());
        Ok(buf)
    }
}

impl DataDecrypt for ChaCha {
    fn data_decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let nonce = Nonce::from_slice(&data[..12]);
        Ok(self.decrypt(nonce, &data[12..]).unwrap())
    }
}

pub fn data_encrypt(data_reader: &mut dyn Read, key: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    data_reader.read_to_end(&mut data)?;
    let key = key.as_ref();
    eprintln!("key_len={}, data_len={}", key.len(), data.len());
    let chacha = ChaCha::try_new(key)?;
    chacha.data_encrypt(&data)
}

pub fn data_decrypt(data_reader: &mut dyn Read, key: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    let mut b64_string = String::new();
    data_reader.read_to_string(&mut b64_string)?;
    let encrypted = base64_decode(&b64_string)?;
    eprintln!("encrypted_total_len={}", encrypted.len());
    let chacha = ChaCha::try_new(key.as_ref())?;
    chacha.data_decrypt(&encrypted)
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

    #[test]
    fn test_chacha() {
        let key = gen_pass(32, false, false, false, true).unwrap();
        let data = b"hello";
        let chacha = ChaCha::try_new(key.as_bytes()).unwrap();
        let encrypted = chacha.data_encrypt(data).unwrap();
        let decrypted = chacha.data_decrypt(&encrypted).unwrap();
        println!("data={data:?}, decrypted={decrypted:?}");
        assert_eq!(data, decrypted.as_slice());
    }
}
