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
    /// * [`Ok`]\([`String`]) - Hash
    /// * [`Err`]\([`Error::HashingFailure`]) - On hashing failure
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
    /// * [`Ok`]\(true) - `data` matches `hash`
    /// * [`Ok`]\(false) - `data` *does not* match `hash`
    /// * [`Err`]\([`Error::HashVerificationFailure`]) - On verification failure
    fn verify(data: &[u8], hash: &str) -> Result<bool>;
}
