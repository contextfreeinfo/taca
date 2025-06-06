// TODO S3 storage or some such.

use anyhow::Result;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Write, format},
    fs::{DirBuilder, File, OpenOptions},
    os::unix::fs::{DirBuilderExt, OpenOptionsExt},
    path::{Path, PathBuf},
};
use tempfile::NamedTempFile;

pub struct LocalStorage {
    store: PathBuf,
    tmp: PathBuf,
}

impl LocalStorage {
    pub fn init(root: impl AsRef<Path>) -> Result<Self> {
        // Use awkward names as a marker of usage intent.
        let store = root.as_ref().join("fskv-store");
        match () {
            _ if root.as_ref().exists() => {
                // Check if we believe we should be in here. This is just a
                // nicety to help avoid accidentally spamming wrong places.
                anyhow::ensure!(store.exists());
            }
            _ => {
                // Make the dir. If someone else is fighting to make this dir at
                // this same moment, hopefully it's for the same purpose.
                make_dir(&store)?;
            }
        }
        // Put tmp under same root to expect they're on the same fs.
        let tmp = root.as_ref().join("fskv-tmp");
        make_dir(&tmp)?;
        Ok(Self { store, tmp })
    }

    pub fn find_existing_path(&self, key: &str) -> Result<Option<(PathBuf, EntryInfo)>> {
        let mut hash = hash_to_u64(key.as_bytes());
        loop {
            let mut path = self.path_for(hash);
            path.set_extension("fskv.json");
            if !path.exists() {
                return Ok(None);
            }
            let mut file = File::open(&path)?;
            let info: EntryInfo = serde_json::from_reader(&mut file)?;
            if info.key == key {
                path.set_file_name(&info.name);
                // TODO Check hash.
                return Ok(Some((path, info)));
            }
            hash += 1;
        }
    }

    fn path_for(&self, hash: u64) -> PathBuf {
        let hex = hex_u64(hash);
        let mut path = self.store.clone();
        path.push(&hex[..2]);
        path.push(hex);
        path
    }

    fn persist(&self, key: &str, file: NamedTempFile) -> Result<()> {
        let path: PathBuf = panic!();
        // See https://github.com/Stebalien/tempfile/pull/111/files#r322282841
        Ok(file.persist(&path)?.sync_all()?)
    }

    pub fn get_bytes(&self, key: &str) -> Option<Vec<u8>> {
        None
    }

    // TODO get_info

    pub fn get_text(&self, key: &str) -> Option<String> {
        None
    }

    pub fn remove(&self, key: &str) -> Result<()> {
        anyhow::bail!("")
    }

    pub fn set_bytes(&self, key: &str, value: &[u8]) -> Result<()> {
        anyhow::bail!("")
    }

    pub fn set_text(&self, key: &str, value: &str) -> Result<()> {
        anyhow::bail!("")
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct EntryInfo {
    key: String,
    name: String,
    hash: String,
    sequence: u32,
    timestamp: Timestamp,
}

pub fn hash_bytes(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_string()
}

pub fn hash_to_u64(bytes: &[u8]) -> u64 {
    let hash = blake3::hash(bytes);
    u64::from_le_bytes(hash.as_bytes()[..8].try_into().unwrap())
}

pub fn hex_u64(n: u64) -> String {
    format!("{n:016x}")
}

fn make_dir(path: &Path) -> Result<()> {
    let mut builder = DirBuilder::new();
    // For now on windows, rely on local data dir being available only to user.
    // TODO Use windows crate and ACLs to ensure?
    #[cfg(not(windows))]
    {
        builder.mode(0o700);
    }
    builder.recursive(true).create(path)?;
    Ok(())
}

fn make_new_file(path: &Path) -> Result<()> {
    let mut builder = OpenOptions::new();
    // For now on windows, rely on local data dir being available only to user.
    // TODO Use windows crate and ACLs to ensure?
    #[cfg(not(windows))]
    {
        builder.mode(0o600);
    }
    builder.create_new(true).open(path)?;
    Ok(())
}
