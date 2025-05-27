use crate::Cli;
use anyhow::{Context, Result, anyhow};
use base64::engine::{Engine, general_purpose};
use directories::ProjectDirs;
use ed25519_dalek::SigningKey;
use jiff::{Timestamp, TimestampRound, Unit};
use log::info;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use std::fs::{
    self, {DirBuilder, File, OpenOptions},
};
use std::io::{Read, Seek, Write};
#[cfg(unix)]
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::Path;
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

fn ensure_private_key(app_info: &AppInfo) -> Result<SigningKeyInfo> {
    let dirs = ProjectDirs::from("", "", "Taca").ok_or_else(|| anyhow!("no data dir"))?;
    // Make private keys dir.
    let mut path = dirs.data_local_dir().join("keys");
    let mut builder = DirBuilder::new();
    // For now on windows, rely on local data dir being available only to user.
    // TODO Use windows crate and ACLs to ensure?
    #[cfg(not(windows))]
    {
        builder.mode(0o700);
    }
    builder.recursive(true).create(&path)?;
    // Read or write private key file.
    path.push(format!("{}.json", app_info.owner.replace('/', "%")));
    let key_info = match () {
        _ if path.exists() => {
            // Read the locally saved key for this owner.
            let file = File::open(&path)?;
            let save_key_info: PrivateKeyInfo = serde_json::from_reader(file)?;
            assert_eq!(&app_info.owner, &save_key_info.owner);
            let bytes = general_purpose::STANDARD.decode(save_key_info.private)?;
            let signing_key = SigningKey::from_bytes(<&[u8; 32]>::try_from(&bytes[..])?);
            SigningKeyInfo {
                signing_key,
                created_at: save_key_info.created_at,
            }
        }
        _ => {
            // Make a new key for this owner.
            // TODO If network connected, see if a known public key already exists?
            let mut secure_rng = OsRng;
            let signing_key = SigningKey::generate(&mut secure_rng);
            let created_at =
                Timestamp::now().round(TimestampRound::new().smallest(Unit::Millisecond))?;
            let private = general_purpose::STANDARD.encode(signing_key.to_bytes());
            let save_key_info = PrivateKeyInfo {
                owner: app_info.owner.clone(),
                private,
                created_at,
            };
            let mut builder = OpenOptions::new();
            #[cfg(not(windows))]
            {
                builder.mode(0o600);
            }
            let file = builder
                .create_new(true)
                .write(true)
                .open(&path)
                .with_context(|| format!("couldn't write: {}", path.display()))?;
            serde_json::to_writer_pretty(file, &save_key_info)?;
            info!("Writing new key for {} at: {:?}", &app_info.owner, &path);
            SigningKeyInfo {
                signing_key,
                created_at,
            }
        }
    };
    Ok(key_info)
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Seal {
    // If we don't timestamp, we get the same seal file and sig for same content and key.
    // sealed_at: Timestamp,
    id: String,
    // TODO Version number.
    owner: String,
    key: SealKeyInfo,
    entries: Vec<SealEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SealEntry(String, String);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct SealKeyInfo {
    public: String,
    created_at: Timestamp,
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

const TACA_RUNTIME_SIGNING_CONTEXT: &[u8] = b"TacaRuntimeApp";
