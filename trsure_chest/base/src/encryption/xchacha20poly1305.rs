use super::definitions::{Encoding, Encryption, Error};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Key, XChaCha20Poly1305, XNonce,
};

/// Container for encrypted data and the necessary information to decrypt it.
pub struct EncryptionData {
    // Nonce for decrypting `content`
    nonce: Vec<u8>,
    // Encrypted data
    content: Vec<u8>,
}

impl Encoding<EncryptionData> for EncryptionData {
    fn encode(&self) -> Vec<u8> {
        let mut data = vec![];
        data.append(&mut self.nonce.clone());
        data.append(&mut self.content.clone());
        data
    }

    fn decode(data: &[u8]) -> Result<EncryptionData, Error> {
        if data.len() < 24 {
            return Err(Error::InvalidData("Given data too short to decode".into()));
        }

        Ok(Self {
            nonce: data[0..24].to_vec(),
            content: data[24..].to_vec(),
        })
    }
}

impl Encryption<EncryptionData> for EncryptionData {
    fn encrypt(plain: &[u8]) -> Result<(EncryptionData, Vec<u8>), Error> {
        let key = XChaCha20Poly1305::generate_key(&mut OsRng);
        let cipher = XChaCha20Poly1305::new(&key);
        let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);

        let content = cipher
            .encrypt(&nonce, plain)
            .map_err(Error::EncryptionFailure)?;

        let encryption_data = EncryptionData {
            nonce: nonce.to_vec(),
            content,
        };

        Ok((encryption_data, key.to_vec()))
    }

    fn decrypt(&self, key: &[u8]) -> Result<Vec<u8>, Error> {
        if key.len() != 32 {
            return Err(Error::InvalidData("Given key has invalid length".into()));
        }

        let key = Key::from_slice(key);
        let cipher = XChaCha20Poly1305::new(key);
        let nonce = XNonce::from_slice(&self.nonce);

        cipher
            .decrypt(nonce, self.content.as_ref())
            .map_err(Error::DecryptionFailure)
    }
}
