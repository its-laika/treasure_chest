use super::definitions::{Encoding, Encryption};
use crate::error::{Error, Result};
use chacha20poly1305::{
    aead::{AeadCore, AeadMutInPlace, KeyInit, OsRng},
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

    fn decode<TI: IntoIterator<Item = u8>>(data: TI) -> Result<XChaCha20Poly1305Data> {
        let mut data = data.into_iter().collect::<Vec<u8>>();
        if data.len() < 24 {
            return Err(Error::InvalidEncryptionData("Data too short".into()));
        }

        let content = data.split_off(24);

        Ok(Self {
            nonce: data,
            content,
        })
    }
}

impl Encryption<XChaCha20Poly1305Data> for XChaCha20Poly1305Data {
    fn encrypt<TI: IntoIterator<Item = u8>>(plain: TI) -> Result<(XChaCha20Poly1305Data, Vec<u8>)> {
        let key = XChaCha20Poly1305::generate_key(&mut OsRng);
        let mut cipher = XChaCha20Poly1305::new(&key);
        let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);

        let mut plain = plain.into_iter().collect::<Vec<u8>>();

        cipher
            .encrypt_in_place(&nonce, &[], &mut plain)
            .map_err(|_| Error::EncryptionFailed)?;

        let encryption_data = XChaCha20Poly1305Data {
            nonce: nonce.to_vec(),
            content: plain,
        };

        Ok((encryption_data, key.to_vec()))
    }

    fn encrypt_with_key<TI: IntoIterator<Item = u8>>(
        plain: TI,
        key: &[u8],
    ) -> Result<XChaCha20Poly1305Data> {
        let key = Key::from_slice(key);
        let mut cipher = XChaCha20Poly1305::new(key);
        let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);

        let mut content = plain.into_iter().collect::<Vec<u8>>();

        cipher
            .encrypt_in_place(&nonce, &[], &mut content)
            .map_err(|_| Error::EncryptionFailed)?;

        Ok(XChaCha20Poly1305Data {
            nonce: nonce.to_vec(),
            content,
        })
    }

    fn decrypt(mut self, key: &[u8]) -> Result<Vec<u8>> {
        if key.len() != 32 {
            return Err(Error::InvalidEncryptionData("Invalid key length".into()));
        }

        let key = Key::from_slice(key);
        let mut cipher = XChaCha20Poly1305::new(key);
        let nonce = XNonce::from_slice(&self.nonce);

        cipher
            .decrypt_in_place(nonce, &[], &mut self.content)
            .map_err(|_| Error::EncryptionFailed)?;

        Ok(self.content)
    }
}
