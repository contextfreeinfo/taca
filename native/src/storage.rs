// TODO S3 storage or some such.

use anyhow::Result;
use sha2::{Digest, Sha512};
use zip::result;
use std::{
    fs::{DirBuilder, File, OpenOptions},
    os::unix::fs::{DirBuilderExt, OpenOptionsExt},
    path::{Path, PathBuf},
};

pub struct LocalStorage {
    root: PathBuf,
}

impl LocalStorage {
    pub fn init(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        let marker = root.join(MARKER_NAME);
        match () {
            _ if root.exists() => {
                // Check if we believe we should be in here.
                anyhow::ensure!(marker.exists());
            }
            _ => {
                if let Some(parent) = root.parent() {
                    make_dir(parent, true)?;
                }
                make_dir(&root, false)?;
                // Mark that we expect to be in here.
                make_new_file(&marker)?;
            }
        }
        Ok(Self { root })
    }

    pub fn find_existing_path(&self, key: &str) -> PathBuf {
        panic!()
    }

    fn open_for_write(&self, key: &str) -> Result<(PathBuf, File)> {
        panic!()
    }

    pub fn get_bytes(&self, key: &str) -> Option<Vec<u8>> {
        None
    }

    // TODO get_info

    pub fn get_text(&self, key: &str) -> Option<String> {
        None
    }

    pub fn set_bytes(&self, key: &str, value: &[u8]) -> Option<Vec<u8>> {
        None
    }

    pub fn set_text(&self, key: &str, value: &str) -> Option<String> {
        None
    }
}

pub const MARKER_NAME: &str = "hash-storage";

    pub fn hash_key_for(key: &str) -> String {
        let mut hasher = Sha512::new();
        hasher.update(key);
        let result = hasher.finalize();
        hex::encode(&result[..16])
    }

fn make_dir(path: &Path, recursive: bool) -> Result<()> {
    let mut builder = DirBuilder::new();
    // For now on windows, rely on local data dir being available only to user.
    // TODO Use windows crate and ACLs to ensure?
    #[cfg(not(windows))]
    {
        builder.mode(0o700);
    }
    builder.recursive(recursive).create(path)?;
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
