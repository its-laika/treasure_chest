//! Module containing functions for saving / reading encrypted data on disk

use super::error::{Error, Result};
use crate::configuration::CONFIGURATION;
use serde::{Deserialize, Serialize};
use std::io;
use std::path::PathBuf;
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
/// * [`Ok`]\([`PathBuf`]) - path to file
/// * [`Err`]\([`Error::SavingFileFailed`]) - if file couldn't be saved
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

    Ok(file_path)
}

/// Load data from disk
///
/// # Arguments
///
/// * `id` - File id
///
/// # Returns
///
/// * [`Ok`]\([`Vec<u8>`]) - file content
/// * [`Err`]\([`Error::LoadingFileFailed`]) - if file couldn't be loaded
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
/// * [`Ok`]\(`()`) - file doesn't exist (anymore)  
/// * [`Err`]\([`Error::DeletingFileFailed`]) - file couldn't be deleted
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
