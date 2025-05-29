use crate::build::{
    Seal, SerdeOptionalExtension, TACA_RUNTIME_SIGNING_CONTEXT, user_data_dir_ensure,
    user_data_file_write,
};
use anyhow::{Result, anyhow};
use base64::engine::{Engine, general_purpose};
use ed25519_dalek::{Signature, VerifyingKey};
use jiff::Timestamp;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_rusqlite::{from_row, to_params_named};
use sha2::{Digest, Sha512};
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufReader, Read, Seek};
use std::path::{self, Path};
use zip::{ZipArchive, read::ZipFile};

/// Abstracted to allow flexible representation in the future.
pub struct Part {
    archive: RefCell<ZipArchive<Box<dyn ReadSeek>>>,
}

pub trait ReadSeek: Read + Seek {}
impl<T: Read + Seek + ?Sized> ReadSeek for T {}

impl Part {
    pub fn from_path(path: &Path) -> Result<Part> {
        let file = File::open(path)?;
        // BufReader internally delegates on read_to_end, so wrapping is cheap.
        let reader: Box<dyn ReadSeek> = Box::new(BufReader::new(file));
        let archive = RefCell::new(ZipArchive::new(reader)?);
        let part = Part { archive };
        part.verify()?;
        Ok(part)
    }

    // /// Includes only files with legal names.
    // pub fn entries(&self) -> Vec<String> {
    //     // TODO Abstract entries for access by numeric index also?
    //     let archive = self.archive.borrow_mut();
    //     for index in 0..archive.len() {
    //         let file = self.zip.by_index(i).ok()?;
    //         let name = file.name();
    //         if file.is_dir() || name.contains("..") || name.starts_with('/') {
    //             return None;
    //         }
    //         Some(name)
    //     }
    // }

    /// TODO How does this relate to large subparts?
    pub fn read_bytes(&self, name: &str) -> Result<Vec<u8>> {
        let mut archive = self.archive.borrow_mut();
        let mut file = archive.by_name(name)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(buf)
    }

    pub fn read_string(&self, name: &str) -> Result<String> {
        let mut archive = self.archive.borrow_mut();
        let mut file = archive.by_name(name)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        Ok(buf)
    }

    fn verify(&self) -> Result<()> {
        // Check and read seal.
        let seal_bytes = self.read_bytes("seal.json")?;
        let seal: Seal = serde_json::from_str(str::from_utf8(&seal_bytes)?)?;
        {
            let key = general_purpose::STANDARD.decode(&seal.key.public)?;
            let key = key.try_into().map_err(|_| anyhow!("bad length"))?;
            let key = VerifyingKey::from_bytes(&key)?;
            let sig = self.read_string("seal.sig")?;
            let sig = general_purpose::STANDARD.decode(sig.trim())?;
            let sig = sig.try_into().map_err(|_| anyhow!("bad length"))?;
            let sig = Signature::from_bytes(&sig);
            let mut hasher = Sha512::new();
            hasher.update(&seal_bytes);
            key.verify_prehashed_strict(hasher, Some(TACA_RUNTIME_SIGNING_CONTEXT), &sig)?;
        }
        // Check only good entries.
        {
            let mut archive = self.archive.borrow_mut();
            for index in 0..archive.len() {
                let entry = archive.by_index(index)?;
                if !is_safe_file(&entry) {
                    return Err(anyhow!(format!("bad entry: {:?}", entry.name())));
                }
            }
            // Check total entry count, with +2 for the seal and sig.
            // TODO If mismatch, provide diff?
            anyhow::ensure!(
                archive.len() == seal.entries.len() + 2,
                "wrong seal entry count"
            );
        }
        // Check entry hashes.
        {
            for entry in &seal.entries {
                anyhow::ensure!(
                    !matches!(entry.0.as_str(), "seal.json" | "seal.sig"),
                    "bad seal entry"
                );
                let bytes = self.read_bytes(&entry.0)?;
                let mut hasher = Sha512::new();
                hasher.update(&bytes);
                let hash = hasher.finalize().to_vec();
                let hash = general_purpose::STANDARD.encode(&hash);
                anyhow::ensure!(hash == entry.1, "bad hash for: {:?}", &entry.0);
            }
        }
        // Check existing and/or store public key for this owner.
        track_owner(&seal)?;
        // Done.
        Ok(())
    }
}

fn is_safe_file<R: Read>(file: &ZipFile<R>) -> bool {
    let name = file.name();
    file.is_file()
        && !(name.starts_with('/')
            || name.contains('\\')
            || name.contains('\0')
            || Path::new(name)
                .components()
                .any(|it| matches!(it, path::Component::ParentDir)))
}

#[derive(Debug, Deserialize, Serialize)]
struct PublicKeyRow {
    /// Explicit naming to ensure we think about it while handling it.
    public: String,
    owner: String,
    created_at: Timestamp,
    revoked_at: Option<Timestamp>,
}

fn track_owner(seal: &Seal) -> Result<()> {
    // TODO Also track and check https web pages in the future.
    let mut path = user_data_dir_ensure("meta")?;
    path.push("taca-meta.sqlite");
    user_data_file_write(&path)?;
    let conn = Connection::open(&path)?;
    conn.execute(
        "create table if not exists public_key (
            public text not null primary key,
            owner text not null,
            created_at text not null,
            revoked_at text
        )",
        [],
    )?;
    conn.execute(
        "create index if not exists idx_public_owner on public_key(owner)",
        [],
    )?;
    match conn
        .query_row_and_then(
            "select * from public_key where public = ?1",
            [&seal.key.public],
            from_row::<PublicKeyRow>,
        )
        .optional()?
    {
        Some(key_info) => {
            // println!("found existing public key");
            anyhow::ensure!(key_info.owner == seal.owner, "wrong owner for public key");
            anyhow::ensure!(
                key_info.created_at == seal.key.created_at,
                "public key created timestamp mismatch"
            );
            anyhow::ensure!(key_info.revoked_at.is_none(), "public key revoked");
        }
        None => {
            let key_info = PublicKeyRow {
                public: seal.key.public.clone(),
                owner: seal.owner.clone(),
                created_at: seal.key.created_at,
                revoked_at: None,
            };
            // TODO Require interactive consent?
            println!("Adding new public key: {:?}", key_info);
            conn.execute(
                "insert into public_key (public, owner, created_at, revoked_at)
                    values (:public, :owner, :created_at, :revoked_at)
                ",
                to_params_named(&key_info)?.to_slice().as_slice(),
            )?;
        }
    }
    Ok(())
}
