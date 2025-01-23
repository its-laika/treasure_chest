use super::error::Error;
use crate::configuration::CONFIGURATION;
use base::{
    encryption::{Encoding, Encryption, XChaCha20Poly1305},
    file::{ensure_deleted, load, store},
};
use std::path::PathBuf;

pub fn store_data<U, T: Encoding<U>>(content: T, base_name: &str) -> Result<PathBuf, Error> {
    let mut file_path = CONFIGURATION.file_path.clone();
    file_path.push(base_name);

    store(&file_path, &content.encode()).map_err(Error::SavingFileFailed)?;

    Ok(file_path)
}

pub fn load_encrypted_data(base_name: &str, key: &[u8]) -> Result<Vec<u8>, Error> {
    let mut file_path = CONFIGURATION.file_path.clone();
    file_path.push(base_name);

    let content = load(&file_path).map_err(Error::LoadingFileFailed)?;

    XChaCha20Poly1305::decode(&content)
        .map_err(Error::DecrytpionFailed)?
        .decrypt(key)
        .map_err(Error::DecrytpionFailed)
}

pub fn delete_file(base_name: &str) -> Result<(), Error> {
    let mut file_path = CONFIGURATION.file_path.clone();
    file_path.push(base_name);

    ensure_deleted(&file_path).map_err(Error::DeletingFileFailed)
}
