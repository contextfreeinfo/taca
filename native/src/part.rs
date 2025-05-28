use anyhow::{Result, anyhow};
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

    fn verify(&self) -> Result<()> {
        let mut archive = self.archive.borrow_mut();
        // Check only good entries.
        for index in 0..archive.len() {
            let entry = archive.by_index(index)?;
            if !is_safe_file(&entry) {
                return Err(anyhow!(format!("bad entry: {:?}", entry.name())));
            }
        }
        // Check and read seal.
        // TODO
        // Check total entry count.
        // TODO
        // Check entry hashes.
        // TODO
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
