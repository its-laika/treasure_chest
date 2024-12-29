use core::fmt;

/// Possible errors during encryption / encoding
pub enum Error {
    EncryptionFailure,
    DecryptionFailure,
    InvalidData,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::DecryptionFailure => write!(f, "Decryption failed"),
            Error::EncryptionFailure => write!(f, "Encryption failed"),
            Error::InvalidData => write!(f, "Invalid data"),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EncryptionFailure => write!(f, "EncryptionFailure"),
            Self::DecryptionFailure => write!(f, "DecryptionFailure"),
            Self::InvalidData => write!(f, "InvalidData"),
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
