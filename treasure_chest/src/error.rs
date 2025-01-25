use crate::encryption;
use sea_orm::DbErr;
use std::fmt;

pub enum Error {
    DateCalculationFailed,
    DatabaseOperationFailed(DbErr),
    IpHeaderMissing(String),
    IpHeaderInvalid,
    SavingFileFailed(std::io::Error),
    LoadingFileFailed(std::io::Error),
    DeletingFileFailed(std::io::Error),
    ReadingBodyFailed(axum::Error),
    EncryptionFailed(encryption::Error),
    DecrytpionFailed(encryption::Error),
    KeyInvalid,
    JsonSerializationFailed(serde_json::Error),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DateCalculationFailed => write!(f, "Date calculation failed"),
            Self::DatabaseOperationFailed(inner) => {
                write!(f, "Database operation failed: {inner}")
            }
            Self::IpHeaderMissing(header_name) => write!(f, "IP header {header_name} missing"),
            Self::IpHeaderInvalid => write!(f, "IP header invalid"),
            Self::SavingFileFailed(inner) => write!(f, "Saving file failed: {inner}"),
            Self::LoadingFileFailed(inner) => write!(f, "Loading file failed: {inner}"),
            Self::DeletingFileFailed(inner) => write!(f, "Removing file failed: {inner}"),
            Self::ReadingBodyFailed(inner) => write!(f, "Reading body failed: {inner}"),
            Self::EncryptionFailed(inner) => write!(f, "Encryption failed: {:?}", inner),
            Self::DecrytpionFailed(inner) => write!(f, "Decryption failed: {:?}", inner),
            Self::KeyInvalid => write!(f, "Key invalid"),
            Self::JsonSerializationFailed(inner) => write!(f, "JSON Serialization failed: {inner}"),
        }
    }
}
