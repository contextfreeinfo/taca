use crate::Cli;
use anyhow::{Result, anyhow};
use base64::engine::{Engine, general_purpose};
use directories::ProjectDirs;
use ed25519_dalek::SigningKey;
use jiff::{Timestamp, TimestampRound, Unit};
use rand::rngs::OsRng;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_rusqlite::{from_row, to_params_named};
use sha2::{Digest, Sha512};
use std::fs::{
    self, {DirBuilder, File, OpenOptions},
};
use std::io::{Read, Seek, Write};
#[cfg(unix)]
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use zip::{ZipWriter, write::SimpleFileOptions};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AppInfo {
    id: String,
    owner: String,
}

struct Bundler<'a> {
    app_info: &'a AppInfo,
    entries: Vec<Entry>,
    root: &'a Path,
    signing_key_info: &'a SigningKeyInfo,
}

#[derive(Debug)]
struct Entry {
    name: String,
    hash: Vec<u8>,
}

#[derive(Debug)]
struct SigningKeyInfo {
    signing_key: SigningKey,
    created_at: Timestamp,
}

pub fn build(cli: Cli) {
    bundle(cli.build.as_ref().unwrap()).unwrap();
}

fn bundle(src: &Path) -> Result<()> {
    let dest = src.with_extension("taca");
    // Overwrites if present.
    let zip_file = File::create(dest)?;
    let mut zip = ZipWriter::new(zip_file);
    // Get some app info.
    let app_info: AppInfo = {
        let file = File::open(src.join("app.json"))?;
        serde_json::from_reader(file)?
    };
    let signing_key_info = ensure_private_key(&app_info)?;
    // TODO Reuse or store key in private file, with created timestamp.
    // TODO Option to export/import key with strength-checked passphrase.
    let mut bundler = Bundler {
        app_info: &app_info,
        entries: vec![],
        root: src,
        signing_key_info: &signing_key_info,
    };
    // TODO Verify required contents.
    add_dir_to_zip(&mut bundler, &mut zip, src)?;
    write_seal_signed(&bundler, &mut zip)?;
    zip.finish()?;
    // dbg!(&bundler.entries);
    Ok(())
}

fn add_dir_to_zip<W: Seek + Write>(
    bundler: &mut Bundler,
    zip: &mut ZipWriter<W>,
    path: &Path,
) -> Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let name = path.strip_prefix(bundler.root).unwrap().to_str().unwrap();
        match name {
            // "app.json" => {
            //     add_pubkey_to_json(bundler, zip, &path)?;
            // }
            // TODO Reject .is_symlink() cases?
            _ if path.is_file() => {
                // TODO Specially process some files like app.json.
                // TODO Automate things like public key.
                // TODO Sign individual file contents instead of all at end?
                // TODO Automate componentization?
                let file = File::open(&path)?;
                add_signed_file(bundler, zip, name, file)?;
            }
            _ if path.is_dir() => {
                zip.add_directory(name.to_string() + "/", zip_options())?;
                add_dir_to_zip(bundler, zip, &path)?;
            }
            _ => panic!(),
        }
    }
    Ok(())
}

fn add_signed_file<R: Read, W: Seek + Write>(
    bundler: &mut Bundler,
    zip: &mut zip::ZipWriter<W>,
    name: &str,
    mut reader: R,
) -> Result<()> {
    zip.start_file(name, zip_options())?;
    let mut hasher = Sha512::new();
    let mut buf = [0u8; 8192];
    loop {
        let len = reader.read(&mut buf)?;
        if len == 0 {
            break;
        }
        hasher.update(&buf[..len]);
        zip.write_all(&buf[..len])?;
    }
    let hash = hasher.finalize().to_vec();
    bundler.entries.push(Entry {
        name: name.to_string(),
        hash,
    });
    Ok(())
}

fn zip_options() -> SimpleFileOptions {
    SimpleFileOptions::default()
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct PrivateKeyInfo {
    owner: String,
    /// Explicit naming to ensure we think about it while handling it.
    private: String,
    created_at: Timestamp,
}

#[derive(Debug, Deserialize, Serialize)]
struct PrivateKeyRow {
    owner: String,
    /// Explicit naming to ensure we think about it while handling it.
    private_key_bytes: Vec<u8>,
    created_at: Timestamp,
}

fn ensure_private_key(app_info: &AppInfo) -> Result<SigningKeyInfo> {
    // Make a secrets dir and a file for storing private keys.
    let mut path = user_data_dir_ensure("secret")?;
    path.push("private-taca-keys-keep-secret.sqlite");
    user_data_file_write(&path)?;
    // And reopen from scratch for connection.
    let conn = Connection::open(&path)?;
    // Be very explicit in naming to remember things are private/secret.
    // TODO Another table to store multiple public key URLs per owner?
    // TODO But that part's not secret.
    // TODO Also track revoked public keys elsewhere.
    // TODO Keep old private keys in case someone wants to revert?
    // TODO Name private keys?
    // TODO See https://crates.io/crates/rusqlite_migration eventually?
    conn.execute(
        "create table if not exists private_key (
            owner text primary key not null,
            private_key_bytes blob not null unique,
            created_at text not null
        )",
        [],
    )?;
    let key_info = match conn
        .query_row_and_then(
            "select * from private_key where owner = ?1",
            [&app_info.owner],
            from_row::<PrivateKeyRow>,
        )
        .optional()?
    {
        Some(key_info) => key_info,
        None => {
            // Make a new key for this owner.
            // TODO If network connected, see if a known public key already exists?
            let mut secure_rng = OsRng;
            let signing_key = SigningKey::generate(&mut secure_rng);
            let created_at =
                Timestamp::now().round(TimestampRound::new().smallest(Unit::Millisecond))?;
            let private_key_bytes: Vec<u8> = signing_key.to_bytes().to_vec();
            let key_info = PrivateKeyRow {
                owner: app_info.owner.clone(),
                private_key_bytes,
                created_at,
            };
            conn.execute(
                "insert into private_key (owner, private_key_bytes, created_at)
                    values (:owner, :private_key_bytes, :created_at)
                ",
                to_params_named(&key_info)?.to_slice().as_slice(),
            )?;
            key_info
        }
    };
    Ok(SigningKeyInfo {
        signing_key: SigningKey::from_bytes(<&[u8; 32]>::try_from(
            &key_info.private_key_bytes[..],
        )?),
        created_at: key_info.created_at,
    })
}

pub fn user_data_dir_ensure(name: &str) -> Result<PathBuf> {
    let dirs = ProjectDirs::from("", "", "Taca").ok_or_else(|| anyhow!("no data dir"))?;
    let mut path = dirs.data_local_dir().join(name);
    // TODO Ok to have profile dirs like "default" right under taca dir?
    path.push("default");
    let mut builder = DirBuilder::new();
    // For now on windows, rely on local data dir being available only to user.
    // TODO Use windows crate and ACLs to ensure?
    #[cfg(not(windows))]
    {
        builder.mode(0o700);
    }
    builder.recursive(true).create(&path)?;
    Ok(path)
}

pub fn user_data_file_write(path: &Path) -> Result<File> {
    let mut builder = OpenOptions::new();
    // For now on windows, rely on local data dir being available only to user.
    // TODO Use windows crate and ACLs to ensure?
    #[cfg(not(windows))]
    {
        builder.mode(0o600);
    }
    Ok(builder.create(true).write(true).open(path)?)
}

pub trait SerdeOptionalExtension<T> {
    fn optional(self) -> Result<Option<T>, serde_rusqlite::error::Error>;
}

impl<T> SerdeOptionalExtension<T> for Result<T, serde_rusqlite::error::Error> {
    fn optional(self) -> Result<Option<T>, serde_rusqlite::error::Error> {
        match self {
            Ok(v) => Ok(Some(v)),
            Err(serde_rusqlite::error::Error::Rusqlite(rusqlite::Error::QueryReturnedNoRows)) => {
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }
}

// fn serde_to_rusqlite(err: serde_rusqlite::error::Error) -> rusqlite::Error {
//     match err {
//         serde_rusqlite::error::Error::Rusqlite(e) => e,
//         other => rusqlite::Error::UserFunctionError(Box::new(other)),
//     }
// }

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Seal {
    // If we don't timestamp, we get the same seal file and sig for same content and key.
    // sealed_at: Timestamp,
    pub id: String,
    // TODO Version number.
    pub owner: String,
    pub key: SealKeyInfo,
    pub entries: Vec<SealEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SealEntry(pub String, pub String);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SealKeyInfo {
    pub public: String,
    pub created_at: Timestamp,
}

fn write_seal(bundler: &Bundler, out: &mut impl Write) -> Result<()> {
    let public = bundler
        .signing_key_info
        .signing_key
        .verifying_key()
        .to_bytes();
    let public = general_purpose::STANDARD.encode(public);
    let seal = Seal {
        // sealed_at: Timestamp::now().round(TimestampRound::new().smallest(Unit::Millisecond))?,
        id: bundler.app_info.id.clone(),
        owner: bundler.app_info.owner.clone(),
        key: SealKeyInfo {
            created_at: bundler.signing_key_info.created_at,
            public,
        },
        entries: bundler
            .entries
            .iter()
            .map(|entry| {
                SealEntry(
                    entry.name.clone(),
                    general_purpose::STANDARD.encode(&entry.hash),
                )
            })
            .collect(),
    };
    serde_json::to_writer_pretty(out, &seal)?;
    Ok(())
}

fn write_seal_signed<W: Seek + Write>(
    bundler: &Bundler,
    zip: &mut zip::ZipWriter<W>,
) -> Result<()> {
    zip.start_file("seal.json", zip_options())?;
    let mut hasher = Sha512::new();
    let mut tee = HashTee {
        writer: zip,
        hasher: &mut hasher,
    };
    write_seal(bundler, &mut tee)?;
    let sig = bundler
        .signing_key_info
        .signing_key
        .sign_prehashed(hasher, Some(TACA_RUNTIME_SIGNING_CONTEXT))?;
    zip.start_file("seal.sig", zip_options())?;
    writeln!(zip, "{}", general_purpose::STANDARD.encode(sig.to_bytes()))?;
    Ok(())
}

struct HashTee<'a, W: std::io::Write, H: Digest> {
    writer: &'a mut W,
    hasher: &'a mut H,
}

impl<W: std::io::Write, H: Digest> std::io::Write for HashTee<'_, W, H> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.hasher.update(buf);
        self.writer.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

pub const TACA_RUNTIME_SIGNING_CONTEXT: &[u8] = b"TacaRuntimeApp";
