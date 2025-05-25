use crate::Cli;
use std::fs::{self, File};
use std::io::{Read, Seek, Write};
use std::path::Path;
use zip::{ZipWriter, write::SimpleFileOptions};

pub fn build(cli: Cli) {
    bundle(cli.build.as_ref().unwrap()).unwrap();
}

fn bundle(src: &Path) -> zip::result::ZipResult<()> {
    let dest = src.with_extension("taca");
    let zip_file = File::create(dest)?;
    let mut zip = ZipWriter::new(zip_file);
    // TODO Verify required contents.
    add_dir_to_zip(&mut zip, &src, &src)?;
    zip.finish()?;
    Ok(())
}

fn add_dir_to_zip<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    path: &Path,
    base: &Path,
) -> zip::result::ZipResult<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let name = path.strip_prefix(base).unwrap().to_str().unwrap();
        if path.is_file() {
            // TODO Specially process some files like app.json.
            // TODO Automate things like public key.
            // TODO Sign individual file contents instead of all at end?
            // TODO Automate componentization?
            zip.start_file(name, SimpleFileOptions::default())?;
            let mut file = File::open(&path)?;
            std::io::copy(&mut file, zip)?;
        } else if path.is_dir() {
            zip.add_directory(name.to_string() + "/", SimpleFileOptions::default())?;
            add_dir_to_zip(zip, &path, base)?;
        }
    }
    Ok(())
}
