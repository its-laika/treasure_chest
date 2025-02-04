//! Error module for this whole crate

use sea_orm::DbErr;
use std::{fmt, result};

pub type Result<T> = result::Result<T, Error>;

pub enum Error {
    DateCalculationFailed,
    DatabaseOperationFailed(DbErr),
    IpHeaderMissing(String),
    IpHeaderInvalid,
    SavingFileFailed(std::io::Error),
    LoadingFileFailed(std::io::Error),
    DeletingFileFailed(std::io::Error),
    ReadingDirectoryFailed(std::io::Error),
    EncryptionFailed,
    DecryptionFailed,
    KeyInvalid,
    JsonSerializationFailed(serde_json::Error),
    InvalidEncryptionData(String),
    HashingFailure(String),
    HashVerificationFailure(String),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DateCalculationFailed => write!(f, "Date calculation failed"),
            Self::DatabaseOperationFailed(inner) => {
                write!(f, "Database operation failed: {inner}")
            }
            Self::IpHeaderMissing(header_name) => write!(f, "Ip header {header_name} missing"),
            Self::IpHeaderInvalid => write!(f, "Ip header invalid"),
            Self::SavingFileFailed(inner) => write!(f, "Saving file failed: {inner}"),
            Self::LoadingFileFailed(inner) => write!(f, "Loading file failed: {inner}"),
            Self::DeletingFileFailed(inner) => write!(f, "Removing file failed: {inner}"),
            Self::ReadingDirectoryFailed(inner) => write!(f, "Reading directory failed: {inner}"),
            Self::EncryptionFailed => write!(f, "Encryption failed"),
            Self::DecryptionFailed => write!(f, "Decryption failed"),
            Self::KeyInvalid => write!(f, "Key invalid"),
            Self::JsonSerializationFailed(inner) => write!(f, "JSON Serialization failed: {inner}"),
            Self::InvalidEncryptionData(inner) => write!(f, "Invalid encryption data: {inner}"),
            Self::HashingFailure(inner) => write!(f, "Hashing failure: {inner}"),
            Self::HashVerificationFailure(inner) => write!(f, "Hash verification failure: {inner}"),
        }
    }
}
