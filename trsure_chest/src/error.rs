use base::encryption;
use sea_orm::DbErr;
use std::fmt;

pub enum Error {
    DateCalculationFailed,
    DatabaseOperationFailed(DbErr),
    IpHeaderMissing(String),
    IpHeaderInvalid,
    SavingFileFailed(std::io::Error),
    ReadingBodyFailed(axum::Error),
    EncrytpionFailed(encryption::Error),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DateCalculationFailed => write!(f, "Date calculation failed"),
            Self::DatabaseOperationFailed(db_error) => {
                write!(f, "Database operation failed: {db_error}")
            }
            Self::IpHeaderMissing(header_name) => write!(f, "IP header {header_name} missing"),
            Self::IpHeaderInvalid => write!(f, "IP header invalid"),
            Self::SavingFileFailed(io_error) => write!(f, "Saving file failed: {io_error}"),
            Self::ReadingBodyFailed(axum_error) => write!(f, "Reading body failed: {axum_error}"),
            Self::EncrytpionFailed(inner) => write!(f, "Encryption failed: {:?}", inner),
        }
    }
}
