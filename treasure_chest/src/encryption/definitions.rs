use chacha20poly1305::aead;
use std::fmt::{self, Debug, Formatter};

/// Possible errors during encryption / encoding
pub enum Error {
    EncryptionFailure(aead::Error),
    DecryptionFailure(aead::Error),
    InvalidData(String),
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::EncryptionFailure(inner) => write!(f, "Encryption failure: {inner}"),
            Self::DecryptionFailure(inner) => write!(f, "Decryption failure: {inner}"),
            Self::InvalidData(inner) => write!(f, "Invalid data given: {inner}"),
        }
    }
}

/// Provides functions to make encrypted data store-able.
/// Handles encoding and decoding of encrypted data including things like nonce.
/// Encoded data can be stored safely.
pub trait Encoding<T> {
    /// Encodes data so that it can be stored.
    fn encode(&self) -> Vec<u8>;

    /// Decodes previously encoded data so that it can be decrypted later.
    fn decode(data: &[u8]) -> Result<T, Error>;
}

/// Provides functions to create encrypted data and decrypt it back.
pub trait Encryption<T> {
    /// Encrypts given data.
    /// Encrypts plain data and returns encryption-data and the key as a tuple.
    ///
    /// # Arguments
    ///
    /// * `plain` - Plain data to encrypt
    ///
    /// # Returns
    ///
    /// * Err([`Error`]) on encryption failure
    /// * Ok(`(encryption data, decryption key)`) on success
    fn encrypt(plain: &[u8]) -> Result<(T, Vec<u8>), Error>;

    // Encrypts plain data with given key and returns encryption-data.
    ///
    /// # Arguments
    ///
    /// * `plain` - Plain data to encrypt
    /// * `key` - Predefined key to use
    ///
    /// # Returns
    ///
    /// * Err([`Error`]) on encryption failure
    /// * Ok(`encryption data`) on success
    fn encrypt_with_key(plain: &[u8], key: &[u8]) -> Result<T, Error>;

    /// Decrypts data with given key.
    ///
    /// # Arguments
    ///
    /// * `key` - Decryption key for this encrypted data
    ///
    /// # Returns
    ///
    /// * Err([`Error`]) on decryption failure
    /// * Ok(`decrypted data`) on success
    fn decrypt(&self, key: &[u8]) -> Result<Vec<u8>, Error>;
}
