use std::{fs::File, path::PathBuf, io::{self, Read, BufReader}, sync::Arc};

use zip::{ZipArchive, result::ZipError};
use thiserror::Error;

use crate::class::{classpath::Classpath, classfile::{ClassFile, ClassError}};

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
	
	JarMeta::from_manifest(&mut BufReader::new(manifest))
    }

    pub fn load_into_classpath(&mut self, classpath: &mut Classpath) -> Result<(), JarError> {
	for i in 0..self.archive.len() {
	    let mut file = self.archive.by_index(i)?;
	    let file_name = file.name().to_string();
	    if file_name.ends_with(".class") {
		println!("reading from {}", file_name);
		let classfile = ClassFile::read_from(&mut file)?;
		let path = &file_name.replace("/", ".")[0..file_name.len() - 6];
		classpath.insert(path, Arc::new(classfile));
	    }
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
    #[error("Weird classfile name: {0}")]
    WeirdName(String),
    #[error("Class conversion failed! {0:?}")]
    ClassError(#[from] ClassError),
}
