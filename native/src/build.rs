use crate::Cli;
use anyhow::Result;
use base64::engine::{Engine, general_purpose};
use ed25519_dalek::SigningKey;
use jiff::{Timestamp, TimestampRound, Unit};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use std::fs::{self, File};
use std::io::{Read, Seek, Write};
use std::path::Path;
use zip::{ZipWriter, write::SimpleFileOptions};

struct Bundler<'a> {
    entries: Vec<Entry>,
    root: &'a Path,
    signing_key: &'a SigningKey,
    signing_key_created_at: Timestamp,
}

#[derive(Debug)]
struct Entry {
    name: String,
    hash: Vec<u8>,
}

pub fn build(cli: Cli) {
    bundle(cli.build.as_ref().unwrap()).unwrap();
}

fn bundle(src: &Path) -> Result<()> {
    let dest = src.with_extension("taca");
    // Overwrites if present.
    let zip_file = File::create(dest)?;
    let mut zip = ZipWriter::new(zip_file);
    let mut secure_rng = OsRng;
    // TODO Reuse or store key in private file, with created timestamp.
    let signing_key = SigningKey::generate(&mut secure_rng);
    let signing_key_created_at =
        Timestamp::now().round(TimestampRound::new().smallest(Unit::Millisecond))?;
    // TODO Option to export/import key with strength-checked passphrase.
    let mut bundler = Bundler {
        entries: vec![],
        root: src,
        signing_key: &signing_key,
        signing_key_created_at,
    };
    // TODO Verify required contents.
    add_dir_to_zip(&mut bundler, &mut zip, src)?;
    write_list_signed(&bundler, &mut zip)?;
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

// fn add_pubkey_to_json<W: Seek + Write>(
//     bundler: &mut Bundler,
//     zip: &mut ZipWriter<W>,
//     path: &Path,
// ) -> Result<()> {
//     let json_str = fs::read_to_string(path)?;
//     let mut value: Value = serde_json::from_str(&json_str)?;
//     let pubkey_bytes = bundler.signing_key.verifying_key().to_bytes();
//     let pubkey_b64 = general_purpose::STANDARD.encode(pubkey_bytes);
//     // TODO Also key creation date?
//     // TODO As sub object with value, createdAt, and maybe algo?
//     if let Value::Object(map) = &mut value {
//         map.insert("publicKey".to_string(), json!(pubkey_b64));
//     }
//     let content = serde_json::to_string_pretty(&value)?;
//     let cursor = Cursor::new(content.as_bytes());
//     add_signed_file(bundler, zip, "app.json", cursor)?;
//     Ok(())
// }

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

// fn key_path_for(owner: &str) -> PathBuf {
//     let mut path = dirs::data_local_dir().unwrap_or_else(|| ".".into());
//     path.push("myapp");
//     fs::create_dir_all(&path).unwrap();
//     path.push(owner.replace('/', "_") + ".key");
//     path
// }

// fn save_key(owner: &str, key: &SigningKey) -> std::io::Result<()> {
//     let path = key_path_for(owner);
//     fs::write(path, key.to_bytes())
// }

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Seal {
    sealed_at: Timestamp,
    key: SealKeyInfo,
    entries: Vec<SealEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SealEntry(String, String);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct SealKeyInfo {
    created_at: Timestamp,
    public: String,
}

fn write_list(bundler: &Bundler, out: &mut impl Write) -> Result<()> {
    let public = bundler.signing_key.verifying_key().to_bytes();
    let public = general_purpose::STANDARD.encode(public);
    let seal = Seal {
        sealed_at: Timestamp::now().round(TimestampRound::new().smallest(Unit::Millisecond))?,
        key: SealKeyInfo {
            created_at: bundler.signing_key_created_at,
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

fn write_list_signed<W: Seek + Write>(
    bundler: &Bundler,
    zip: &mut zip::ZipWriter<W>,
) -> Result<()> {
    zip.start_file("seal.json", zip_options())?;
    let mut hasher = Sha512::new();
    let mut tee = HashTee {
        writer: zip,
        hasher: &mut hasher,
    };
    write_list(bundler, &mut tee)?;
    let sig = bundler
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
