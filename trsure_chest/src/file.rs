use super::error::Error;
use crate::configuration::CONFIGURATION;
use base::{encryption::Encoding, file::store};
use std::{io, path::PathBuf};

pub fn store_data<U, T: Encoding<U>>(content: T, base_name: &str) -> Result<PathBuf, Error> {
    let mut file_path = CONFIGURATION.file_path.clone();
    file_path.push(base_name);

    if file_path.exists() {
        return Err(Error::SavingFileFailed(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "File already exists",
        )));
    }

    store(&file_path, &content.encode()).map_err(Error::SavingFileFailed)?;

    Ok(file_path)
}
