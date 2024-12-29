//! Facade for working with files on the file system
use chacha20poly1305::aead::{rand_core::RngCore, OsRng};
use std::{
    env,
    fs::{self, OpenOptions},
    io::{Error, Read, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

/// Env var that contains the directory where new file paths should be generated.
const ENV_VAR_TARGET_DIRECTORY: &str = "FILE_PATH";

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
    let mut file = OpenOptions::new().create_new(true).write(true).open(path)?;

    if let Err(e) = file.write_all(content) {
        ensure_deleted(path)?;
        return Err(e);
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

    let mut file = OpenOptions::new().read(true).open(path)?;
    file.read_to_end(&mut file_content)?;

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
pub fn find(file_name: &str) -> Result<PathBuf, Error> {
    let target_directory = env::var(ENV_VAR_TARGET_DIRECTORY).unwrap_or_default();
    let target_directory = target_directory.trim();

    if target_directory.is_empty() {
        return Err(Error::new(
            std::io::ErrorKind::NotFound,
            format!("{} not defined", ENV_VAR_TARGET_DIRECTORY),
        ));
    }

    let mut path = PathBuf::from(target_directory);
    path.push(file_name);

    let path = path.canonicalize()?;
    if !path.exists() {
        return Err(Error::new(
            std::io::ErrorKind::NotFound,
            format!("{} doesn't not exist", path.display()),
        ));
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
    if !fs::exists(path)? {
        return Ok(());
    }

    if fs::metadata(path)?.is_file() {
        fs::remove_file(path)
    } else {
        fs::remove_dir_all(path)
    }
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
pub fn get_random_file_path() -> Result<PathBuf, Error> {
    let target_directory = env::var(ENV_VAR_TARGET_DIRECTORY).unwrap_or_default();
    let target_directory = target_directory.trim();

    if target_directory.is_empty() {
        return Err(Error::new(
            std::io::ErrorKind::NotFound,
            format!("{} not defined", ENV_VAR_TARGET_DIRECTORY),
        ));
    }

    let path = PathBuf::from(target_directory).canonicalize()?;

    if !path.exists() {
        return Err(Error::new(
            std::io::ErrorKind::NotFound,
            format!("{} doesn't not exist", ENV_VAR_TARGET_DIRECTORY),
        ));
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Could not get unix timestamp")
        .as_secs();

    loop {
        let mut new_path = path.clone();

        // TODO: I shouldn't use the ChaCha function here.
        // UUID has 128 bit so two u64 + timestamp should be enough.
        new_path.push(format!(
            "{timestamp}_{}{}",
            OsRng.next_u64(),
            OsRng.next_u64()
        ));

        if !new_path.exists() {
            return Ok(new_path);
        }
    }
}
