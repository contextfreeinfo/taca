use crate::Cli;
use anyhow::Result;
use ed25519_dalek::{Signature, SigningKey};
use rand::rngs::OsRng;
use sha2::{Digest, Sha512};
use std::fs::{self, File};
use std::io::{Read, Seek, Write};
use std::path::Path;
use zip::{ZipWriter, write::SimpleFileOptions};

struct Bundler<'a> {
    sigs: Vec<(String, Signature)>,
    root: &'a Path,
    signing_key: SigningKey,
}

pub fn build(cli: Cli) {
    bundle(cli.build.as_ref().unwrap()).unwrap();
}

fn bundle(src: &Path) -> Result<()> {
    // TODO Delete if already present?
    let dest = src.with_extension("taca");
    let zip_file = File::create(dest)?;
    let mut zip = ZipWriter::new(zip_file);
    let mut secure_rng = OsRng;
    let signing_key = SigningKey::generate(&mut secure_rng);
    let mut bundler = Bundler {
        sigs: vec![],
        root: src,
        signing_key,
    };
    // TODO Verify required contents.
    add_dir_to_zip(&mut bundler, &mut zip, src)?;
    zip.finish()?;
    dbg!(&bundler.sigs);
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
        if path.is_file() {
            // TODO Specially process some files like app.json.
            // TODO Automate things like public key.
            // TODO Sign individual file contents instead of all at end?
            // TODO Automate componentization?
            let sig = add_signed_file(zip, name, &path, &bundler.signing_key)?;
            bundler.sigs.push((name.to_string(), sig));
        } else if path.is_dir() {
            zip.add_directory(name.to_string() + "/", SimpleFileOptions::default())?;
            add_dir_to_zip(bundler, zip, &path)?;
        }
    }
    Ok(())
}

fn add_signed_file<W: Seek + Write>(
    zip: &mut zip::ZipWriter<W>,
    name: &str,
    path: &Path,
    signing_key: &SigningKey,
) -> Result<Signature> {
    zip.start_file(name, SimpleFileOptions::default())?;
    let mut file = File::open(path)?;
    let mut hasher = Sha512::new();
    let mut buf = [0u8; 8192];
    loop {
        let len = file.read(&mut buf)?;
        if len == 0 {
            break;
        }
        hasher.update(&buf[..len]);
        zip.write_all(&buf[..len])?;
    }
    // let hash = hasher.finalize();
    Ok(signing_key.sign_prehashed(hasher, Some(TACA_RUNTIME_SIGNING_CONTEXT))?)
}

const TACA_RUNTIME_SIGNING_CONTEXT: &[u8] = b"TacaRuntimeApp";
