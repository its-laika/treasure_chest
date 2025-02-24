//! Module containing functions for saving / reading encrypted data on disk

use super::error::{Error, Result};
use crate::configuration::CONFIGURATION;
use serde::{Deserialize, Serialize};
use std::io;
use std::path::PathBuf;
use std::str::FromStr;
use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
};
use uuid::Uuid;

/// File metadata that will be stored serialized and encrypted in the database
#[derive(Serialize, Deserialize)]
pub struct Metadata {
    /// Name of the uploaded file
    pub file_name: String,
    /// MIME type of the uploaded file
    pub mime_type: String,
}

/// Stores new file on disk
///
/// # Arguments
///
/// * `id` - File id (to use as file name)
/// * `content` - Content to store
///
/// # Returns
///
/// * [`Ok<PathBuf>`] on success with file path
/// * [`Err<Error>`] on error
pub fn store_data(id: &Uuid, content: Vec<u8>) -> Result<PathBuf> {
    let mut file_path = CONFIGURATION.file_path.clone();
    file_path.push(id.to_string());

    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&file_path)
        .map_err(Error::SavingFileFailed)?;

    if let Err(error) = file.write_all(&content) {
        delete(id)?;
        return Err(Error::SavingFileFailed(error));
    }

    drop(content);

    if let Err(error) = file.sync_all() {
        delete(id)?;
        return Err(Error::SavingFileFailed(error));
    }

    Ok(file_path)
}

/// Retrieves the Ids of all stored files.
///
/// This function reads the directory and collects the UUIDs of all files stored.
///
/// # Returns
///
/// * [`Ok<Vec<Uuid>>`] - A vector containing the UUIDs of all stored files.
/// * [`Err<Error>`] on error
pub fn get_stored_file_ids() -> Result<Vec<Uuid>> {
    let mut file_ids = vec![];

    let read_dir =
        fs::read_dir(CONFIGURATION.file_path.clone()).map_err(Error::ReadingDirectoryFailed)?;

    for dir_entry in read_dir {
        let file_name = dir_entry
            .map_err(Error::ReadingDirectoryFailed)?
            .file_name();

        let file_name = file_name
            .to_str()
            .ok_or(Error::ReadingDirectoryFailed(io::Error::new(
                io::ErrorKind::Other,
                "Could not get file name",
            )))?;

        let file_id = Uuid::from_str(file_name).map_err(|_| {
            Error::ReadingDirectoryFailed(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("File name not a Uuid: {file_name}"),
            ))
        })?;

        file_ids.push(file_id);
    }

    Ok(file_ids)
}

/// Load data from disk
///
/// # Arguments
///
/// * `id` - File id
///
/// # Returns
///
/// * [`Ok<Vec<u8>>`] on success, containing file content
/// * [`Err<Error>`] on error
pub fn load_data(id: &Uuid) -> Result<Vec<u8>> {
    let mut file_path = CONFIGURATION.file_path.clone();
    file_path.push(id.to_string());

    let mut content = vec![];

    let mut file = OpenOptions::new()
        .read(true)
        .open(&file_path)
        .map_err(Error::LoadingFileFailed)?;

    if let Err(error) = file.read_to_end(&mut content) {
        return Err(Error::LoadingFileFailed(error));
    };

    Ok(content)
}

/// Ensure file is deleted
///
/// # Arguments
///
/// * `id` - File id
///
/// # Returns
///
/// * [`Ok<()>`] ensuring file doesn't exist (anymore)  
/// * [`Err<Error>`] on error
pub fn delete(id: &Uuid) -> Result<()> {
    let mut file_path = CONFIGURATION.file_path.clone();
    file_path.push(id.to_string());

    if !(fs::exists(&file_path).map_err(Error::DeletingFileFailed)?) {
        return Ok(());
    }

    if !fs::metadata(&file_path)
        .map_err(Error::DeletingFileFailed)?
        .is_file()
    {
        return Err(Error::DeletingFileFailed(io::Error::new(
            io::ErrorKind::IsADirectory,
            "Directory given",
        )));
    }

    fs::remove_file(&file_path).map_err(Error::DeletingFileFailed)
}
