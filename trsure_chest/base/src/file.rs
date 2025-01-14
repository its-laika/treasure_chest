//! Facade for working with files on the file system
use chacha20poly1305::aead::{rand_core::RngCore, OsRng};
use log::error;
use std::{
    env,
    fs::{self, OpenOptions},
    io::{Error, ErrorKind, Read, Write},
    path::{Path, PathBuf},
};

/// Stores given `content` to `path`.
/// This function only creates new files and fails if `path` already exists.
/// Ensures that - if the file could not be written completely - the incomplete
/// file is deleted.
///
/// # Arguments
///
/// * `path` - The target path of the new file.
/// * `content` - Contents of the file
///
/// # Returns
///
/// * `Err(std::io::Error)` if e.g. file exists or cannot be written.
pub fn store(path: &Path, content: &[u8]) -> Result<(), Error> {
    let mut file = match OpenOptions::new().create_new(true).write(true).open(path) {
        Ok(file) => file,
        Err(error) => {
            error!("Could not create file: {}", error);
            return Err(error);
        }
    };

    if let Err(error) = file.write_all(content) {
        error!("Could not write to file: {}", error);

        ensure_deleted(path)?;
        return Err(error);
    }

    Ok(())
}

/// Loads file content from given `path` as bytes.
///
/// # Arguments
///
/// * `path` - The path to read.
///
/// # Returns
///
/// * `Err(std::io::Error)` if e.g. file cannot be read or does not exist.
pub fn load(path: &Path) -> Result<Vec<u8>, Error> {
    let mut file_content = vec![];

    let mut file = match OpenOptions::new().read(true).open(path) {
        Ok(file) => file,
        Err(error) => {
            error!("Could not open file: {}", error);
            return Err(error);
        }
    };

    if let Err(error) = file.read_to_end(&mut file_content) {
        error!("Could not read file: {}", error);
        return Err(error);
    };

    Ok(file_content)
}

/// Tries finding file by given `file_name` in `ENV_VAR_TARGET_DIRECTORY`.
///
/// # Arguments
///
/// * `file_name` - Name of file that should be loaded
///
/// # Returns
///
/// * `Err(std::io::Error)` if e.g. file cannot be read or does not exist.
pub fn get_directory_path(env_key: &str) -> Result<PathBuf, Error> {
    let target_directory = env::var(env_key).unwrap_or_default();
    let target_directory = target_directory.trim();

    if target_directory.is_empty() {
        error!("Missing environment variable value '{}'", env_key);
        return Err(Error::new(ErrorKind::NotFound, "Missing env data"));
    }

    let path = match PathBuf::from(target_directory).canonicalize() {
        Ok(path) => path,
        Err(error) => {
            error!("Could not canonicalize path: {}", error);
            return Err(error);
        }
    };

    if !path.exists() {
        error!(
            "Configured directory (by env {}={}) doesn't not exist",
            env_key, target_directory
        );

        return Err(Error::new(ErrorKind::NotFound, "Configured file not found"));
    }

    Ok(path)
}

/// Ensures that given `path` - if exists - is deleted.
/// Removes files and directories.
///
/// # Arguments
///
/// * `path` - File or directory to delete, if exists
///
/// # Returns
///
/// * `Err(std::io::Error)` if path cannot be checked or deleted
pub fn ensure_deleted(path: &Path) -> Result<(), Error> {
    match fs::exists(path) {
        Ok(false) => return Ok(()),
        Ok(true) => (),
        Err(error) => {
            error!("Could not check whether path exists: {}", error);
            return Err(error);
        }
    }

    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(error) => {
            error!("Could not get metadata: {}", error);
            return Err(error);
        }
    };

    if metadata.is_file() {
        if let Err(error) = fs::remove_file(path) {
            error!("Could not remove file: {}", error);
        }
    } else if let Err(error) = fs::remove_dir_all(path) {
        error!("Could not remove directory: {}", error);
    }

    Ok(())
}

/// Generates a random file name that can be used
/// Uses `ENV_VAR_TARGET_DIRECTORY` to get the target directory and builds a
/// unique (enough) file name, ensuring that it doesn't exist at the time
/// checking. (This function is not TOCTOU resistant but [`store`] is.)
///
/// # Returns
///
/// * Ok(`file_path`) - A currently unused, available path
/// * Err(`std::io::Error`) if either `ENV_VAR_TARGET_DIRECTORY` not correct or
///   an io error happens.
pub fn get_random_file_path(directory: &Path) -> Result<PathBuf, Error> {
    if !directory.is_dir() {
        error!("Given path '{}' is not a directory", directory.display());
        return Err(Error::new(ErrorKind::NotFound, "No directory given"));
    }

    loop {
        let mut file_path = directory.to_path_buf();

        // TODO: Shouldn't use the ChaCha function here.
        // UUID has 128 bit so two u64 should be enough.
        file_path.push(format!("{}{}", OsRng.next_u64(), OsRng.next_u64()));

        if !file_path.exists() {
            return Ok(file_path);
        }
    }
}
