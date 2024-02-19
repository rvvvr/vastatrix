use std::{fs::File, path::PathBuf, io};

use zip::{ZipArchive, result::ZipError};
use thiserror::Error;

use crate::class::classpath::Classpath;

use self::meta::JarMeta;

pub mod meta;

pub struct JarFile {
    archive: ZipArchive<File>,
}

impl JarFile {
    pub fn new(path: PathBuf) -> Result<Self, JarError> {
	let file = File::open(path)?;
	Ok(Self {
	    archive: ZipArchive::new(file)?,
	})
    }

    pub fn grab_meta(&mut self) -> Result<JarMeta, JarError> {
	let manifest = self.archive.by_name("META-INF/MANIFEST.MF")?;

	Ok(JarMeta::from_manifest(manifest)?)
    }

    pub fn load_into_classpath(&self, classpath: &mut Classpath) -> Result<(), JarError> {
	for file_name in self.archive.file_names() {
	    if file_name.ends_with(".class") 
	}
	Ok(())
    }
}

#[derive(Debug, Error)]
pub enum JarError {
    #[error("IO Failed! {0:?}")]
    IOError(#[from] io::Error),
    #[error("Zip failed! {0:?}")]
    ZipError(#[from] ZipError),
}
