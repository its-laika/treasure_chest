use super::error::Error;
use crate::configuration::CONFIGURATION;
use crate::encryption;
use crate::encryption::{Encoding, Encryption};
use entity::file;
use serde::{Deserialize, Serialize};
use std::io;
use std::path::PathBuf;
use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
};

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub file_name: String,
    pub mime_type: String,
}

pub fn store_data<U, T: encryption::Encoding<U>>(
    content: &T,
    base_name: &str,
) -> Result<PathBuf, Error> {
    let mut file_path = CONFIGURATION.file_path.clone();
    file_path.push(base_name);

    let mut file = match OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&file_path)
    {
        Ok(file) => file,
        Err(error) => return Err(Error::SavingFileFailed(error)),
    };

    if let Err(error) = file.write_all(&content.encode()) {
        delete(base_name)?;
        return Err(Error::SavingFileFailed(error));
    }

    Ok(file_path)
}

pub fn load_encrypted_data(base_name: &str, key: &[u8]) -> Result<Vec<u8>, Error> {
    let mut file_path = CONFIGURATION.file_path.clone();
    file_path.push(base_name);

    let mut content = vec![];

    let mut file = match OpenOptions::new().read(true).open(&file_path) {
        Ok(file) => file,
        Err(error) => return Err(Error::LoadingFileFailed(error)),
    };

    if let Err(error) = file.read_to_end(&mut content) {
        return Err(Error::LoadingFileFailed(error));
    };

    encryption::Data::decode(&content)
        .map_err(Error::DecrytpionFailed)?
        .decrypt(key)
        .map_err(Error::DecrytpionFailed)
}

pub fn generate_encrypted_metadata(metadata: &Metadata, key: &[u8]) -> Result<Vec<u8>, Error> {
    let metadata = serde_json::to_string(&metadata).map_err(Error::JsonSerializationFailed)?;

    let encrypted_data = encryption::Data::encrypt_with_key(metadata.as_bytes(), key)
        .map_err(Error::EncryptionFailed)?
        .encode();

    Ok(encrypted_data)
}

pub fn load_encrypted_metadata(file: &file::Model, key: &[u8]) -> Result<Option<Metadata>, Error> {
    let metadata_json = String::from_utf8(
        encryption::Data::decode(&file.encrypted_metadata)
            .map_err(Error::EncryptionFailed)?
            .decrypt(key)
            .map_err(Error::EncryptionFailed)?,
    )
    .map_err(|inner| Error::EncryptionFailed(encryption::Error::InvalidData(inner.to_string())))?;

    let metadata =
        serde_json::from_str::<Metadata>(&metadata_json).map_err(Error::JsonSerializationFailed)?;

    Ok(Some(metadata))
}

pub fn delete(base_name: &str) -> Result<(), Error> {
    let mut file_path = CONFIGURATION.file_path.clone();
    file_path.push(base_name);

    match fs::exists(&file_path) {
        Ok(true) => (),
        Ok(false) => return Ok(()),
        Err(error) => return Err(Error::DeletingFileFailed(error)),
    }

    let metadata = match fs::metadata(&file_path) {
        Ok(metadata) => metadata,
        Err(error) => return Err(Error::DeletingFileFailed(error)),
    };

    if !metadata.is_file() {
        return Err(Error::DeletingFileFailed(io::Error::new(
            io::ErrorKind::IsADirectory,
            "Not a file but directory given",
        )));
    }

    fs::remove_file(&file_path).map_err(Error::DeletingFileFailed)
}
