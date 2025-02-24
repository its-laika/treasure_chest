use crate::error::Result;

/// Provides functions to hash data or to verify hashes
pub trait Hashing {
    /// Hashes given `data`
    ///
    /// # Arguments
    ///
    /// * `data` - Data to hash
    ///
    /// # Returns
    ///
    /// * [`Ok<String>`] on success, containing the hash
    /// * [`Err<Error>`] on error
    fn hash(data: &[u8]) -> Result<String>;

    /// Verifies given `data` against `hash`
    ///
    /// # Arguments
    ///
    /// * `data` - Data to verify hash against
    /// * `hash` - Hash to verify
    ///
    /// # Returns
    ///
    /// * [`Ok<true>`] on `data` matching `hash`
    /// * [`Ok<false>`] on `data` **not** matching `hash`
    /// * [`Err<Error>`] on error
    fn verify(data: &[u8], hash: &str) -> Result<bool>;
}
