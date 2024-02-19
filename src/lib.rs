#![deny(clippy::panic)]

use std::{sync::mpsc::Sender, path::PathBuf, io};

use class::classpath::Classpath;
use jar::{JarFile, JarError};
use thiserror::Error;

pub mod vastatrick;
pub mod jar;
pub mod class;

pub struct Vastatrix {
    classpath: Classpath,
}

impl Vastatrix {
    pub fn new() -> Self {
	Self {
	    classpath: Classpath::new(),
	}
    }

    pub fn go(&self, opt: LaunchOptions) -> Result<(), VastatrixError> {
	match opt.how {
	    HowLaunch::JarFile(path) => {
		let jar = JarFile::new(path)?;
		
	    }
	}
	Ok(())
    }
}

pub struct LaunchOptions {
    how: HowLaunch,
}

pub enum HowLaunch {
    JarFile(PathBuf),
}

#[derive(Error, Debug)]
//generally reserved for fatal errors that mean vastatrix has to shut down. will generally try to handle error time itself.
pub enum VastatrixError {
    #[error("Given classpath was incomplete!")]
    ClasspathIncomplete,
    #[error("This should never ever be returned.")]
    Impossible,
    #[error("Jar failed! {0:?}")]
    JarError(#[from] JarError),
    #[error("IO failed! {0:?}")]
    IOError(#[from] io::Error),
}

unsafe impl Send for VastatrixError {}

pub struct VastatrixRequest {
    responder: Sender<VastatrixResponse>,
    kind: VastatrixRequestKind,
}

unsafe impl Send for VastatrixRequest {}

pub enum VastatrixRequestKind {
    
}

pub enum VastatrixResponse {
    
}
