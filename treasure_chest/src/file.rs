use super::error::Error;
use crate::configuration::CONFIGURATION;
use crate::encryption::{Encoding, Encryption, EncryptionData};
use std::path::PathBuf;
use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
};

pub fn store_data<U, T: Encoding<U>>(content: T, base_name: &str) -> Result<PathBuf, Error> {
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
        delete_file(base_name)?;
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

    EncryptionData::decode(&content)
        .map_err(Error::DecrytpionFailed)?
        .decrypt(key)
        .map_err(Error::DecrytpionFailed)
}

pub fn delete_file(base_name: &str) -> Result<(), Error> {
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

    if metadata.is_file() {
        if let Err(error) = fs::remove_file(&file_path) {
            return Err(Error::DeletingFileFailed(error));
        }
    } else if let Err(error) = fs::remove_dir_all(&file_path) {
        return Err(Error::DeletingFileFailed(error));
    }

    Ok(())
}
