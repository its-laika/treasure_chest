use core::fmt;

/// Possible errors while hashing / verifying
pub enum Error {
    HashingFailure,
    VerifyingFailure,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::HashingFailure => write!(f, "Hashing failed"),
            Error::VerifyingFailure => write!(f, "Verifying failed"),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HashingFailure => write!(f, "HashingFailure"),
            Self::VerifyingFailure => write!(f, "VerifyingFailure"),
        }
    }
}

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
    /// Hash on success or `Error` if hashing fails.
    fn hash(data: &[u8]) -> Result<String, Error>;

    /// Verifies given `data` against `hash`
    ///
    /// # Arguments
    ///
    /// * `data` - Data to verify hash against
    /// * `hash` - Hash to verify
    ///
    /// # Returns
    ///
    /// Either bool that says whether given data matches hash or `Error` if
    /// verification could not be done.
    fn verify(data: &[u8], hash: &str) -> Result<bool, Error>;
}
