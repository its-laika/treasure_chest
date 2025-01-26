use super::definitions::{Encoding, Encryption};
use crate::error::{Error, Result};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Key, XChaCha20Poly1305, XNonce,
};

/// Container for encrypted data and the necessary information to decrypt it.
pub struct XChaCha20Poly1305Data {
    // Nonce for decrypting `content`
    nonce: Vec<u8>,
    // Encrypted data
    content: Vec<u8>,
}

impl Encoding<XChaCha20Poly1305Data> for XChaCha20Poly1305Data {
    fn encode(mut self) -> Vec<u8> {
        let mut data = vec![];
        data.append(&mut self.nonce);
        data.append(&mut self.content);
        data
    }

    fn decode(data: &[u8]) -> Result<XChaCha20Poly1305Data> {
        if data.len() < 24 {
            return Err(Error::InvalidEncryptionData("Data too short".into()));
        }

        Ok(Self {
            nonce: data[0..24].to_vec(),
            content: data[24..].to_vec(),
        })
    }
}

impl Encryption<XChaCha20Poly1305Data> for XChaCha20Poly1305Data {
    fn encrypt(plain: &[u8]) -> Result<(XChaCha20Poly1305Data, Vec<u8>)> {
        let key = XChaCha20Poly1305::generate_key(&mut OsRng);
        let cipher = XChaCha20Poly1305::new(&key);
        let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);

        let content = cipher
            .encrypt(&nonce, plain)
            .map_err(|_| Error::EncryptionFailed)?;

        let encryption_data = XChaCha20Poly1305Data {
            nonce: nonce.to_vec(),
            content,
        };

        Ok((encryption_data, key.to_vec()))
    }

    fn encrypt_with_key(plain: &[u8], key: &[u8]) -> Result<XChaCha20Poly1305Data> {
        let key = Key::from_slice(key);
        let cipher = XChaCha20Poly1305::new(key);
        let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);

        let content = cipher
            .encrypt(&nonce, plain)
            .map_err(|_| Error::EncryptionFailed)?;

        Ok(XChaCha20Poly1305Data {
            nonce: nonce.to_vec(),
            content,
        })
    }

    fn decrypt(&self, key: &[u8]) -> Result<Vec<u8>> {
        if key.len() != 32 {
            return Err(Error::InvalidEncryptionData("Invalid key length".into()));
        }

        let key = Key::from_slice(key);
        let cipher = XChaCha20Poly1305::new(key);
        let nonce = XNonce::from_slice(&self.nonce);

        cipher
            .decrypt(nonce, self.content.as_ref())
            .map_err(|_| Error::DecryptionFailed)
    }
}
