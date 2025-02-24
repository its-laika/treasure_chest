use crate::error::Result;

/// Provides functions to make encrypted data store-able.
/// Handles encoding and decoding of encrypted data including things like nonce.
/// Encoded data can be stored safely.
pub trait Encoding<T> {
    /// Encodes data so that it can be stored.
    ///
    /// # Consumes
    ///
    /// * self
    ///
    /// # Returns
    ///
    /// * Encoded data
    fn encode(self) -> Vec<u8>;

    /// Decodes previously encoded data so that it can be decrypted later.
    ///
    /// # Arguments
    ///
    /// * `data` - Encoded data to decode into `self`
    ///
    /// # Returns
    ///
    /// * [`Ok<self>`] on success
    /// * [`Err<Error>`] on error
    fn decode<TI: IntoIterator<Item = u8>>(data: TI) -> Result<T>;
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
    /// * [`Ok<(T, Vec<u8>)>`] on success, containing (Self, decryption key)
    /// * [`Err<Error>`] on error
    fn encrypt<TI: IntoIterator<Item = u8>>(plain: TI) -> Result<(T, Vec<u8>)>;

    // Encrypts plain data with given key and returns encryption-data.
    ///
    ///
    /// # Arguments
    ///
    /// * `plain` - Plain data to encrypt
    /// * `key` - Predefined key to use
    ///
    /// # Returns
    ///
    /// * [`Ok<Vec<u8>>`] on success, containing encrypted data
    /// * [`Err<Error>`] on error
    fn encrypt_with_key<TI: IntoIterator<Item = u8>>(plain: TI, key: &[u8]) -> Result<T>;

    /// Decrypts data with given key.
    ///
    /// # Arguments
    ///
    /// * `key` - Decryption key for this encrypted data
    ///
    /// # Returns
    ///
    /// * [`Ok<Vec<u8>>`] on success with decrypted data
    /// * [`Err<Error>`] on error
    fn decrypt(self, key: &[u8]) -> Result<Vec<u8>>;
}
