use std::time::{SystemTime, UNIX_EPOCH};

use rusty_box_base::hash::{self, Hashing};

pub const DURATION_SECS: u64 = 7 * 24 * 60 * 60;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct FileInfo {
    pub name: String,
    pub hash: String,
    pub uploaded_at: u64,
    pub uploaded_by: String,
    pub download_until: u64,
    pub downloaded_at: Option<u64>,
    pub downloaded_by: Option<String>,
}

impl FileInfo {
    pub fn new(
        name: String,
        hash: String,
        uploaded_at: u64,
        uploaded_by: String,
        download_until: u64,
    ) -> Self {
        Self {
            name,
            hash,
            uploaded_at,
            uploaded_by,
            download_until,
            downloaded_at: None,
            downloaded_by: None,
        }
    }

    pub fn now(name: String, hash: String, uploaded_by: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Could not load time stamp")
            .as_secs();

        Self::new(name, hash, now, uploaded_by, now + DURATION_SECS)
    }

    pub fn mark_download(&mut self, downloaded_by: String) {
        self.downloaded_at = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Could not load time stamp")
                .as_secs(),
        );
        self.downloaded_by = Some(downloaded_by);
    }

    pub fn is_downloadable(&self) -> bool {
        if self.downloaded_at.is_some() {
            return false;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Could not load time stamp")
            .as_secs();

        self.download_until >= now
    }

    pub fn matches_key(&self, key: &[u8]) -> Result<bool, hash::Error> {
        hash::Argon2::verify(key, &self.hash)
    }
}
