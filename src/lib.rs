use std::{sync::{Arc, mpsc::{Sender, Receiver, self}}, path::PathBuf, io, thread, process};

use class::{classpath::Classpath, Class};
use jar::{JarFile, JarError};
use thiserror::Error;
use vastatrick::Vastatrick;

pub mod vastatrick;
pub mod jar;
pub mod class;

pub struct Vastatrix {
    classpath: Classpath,
    request_reciever: Receiver<VastatrixRequest>,
    request_sender: Sender<VastatrixRequest>,
}

impl Vastatrix {
    pub fn new() -> Self {
	let (request_sender, request_reciever) = mpsc::channel();
	Self {
	    classpath: Classpath::new(),
	    request_reciever,
	    request_sender,
	}
    }

    pub fn go(&mut self, opt: LaunchOptions) -> Result<i32, VastatrixError> {
	match opt.how {
	    HowLaunch::JarFile(path) => {
		let mut jar = JarFile::new(path)?;
		let meta = jar.grab_meta()?.clone();
		jar.load_into_classpath(&mut self.classpath)?;
		let vastatrick_sender = self.request_sender.clone();
		thread::spawn(move || {
		    let mut main_vastatrick = Vastatrick::new(vastatrick_sender);
		    main_vastatrick.run_main_class(meta.main_class.unwrap());
		});
	    }
	}
	Ok(self.listen())
    }

    fn listen(&self) -> i32{
	loop {
	    let request = self.request_reciever.recv().unwrap();
	    match request.kind {
		VastatrixRequestKind::Exit(code) => {
		    process::exit(code);
		},
		VastatrixRequestKind::ResolveClass(classpath) => {
		    let class = self.classpath.resolve(classpath.as_str());
		    request.responder.send(VastatrixResponse::ResolvedClass(class)).unwrap();
		}
	    }
	}
    }
}

pub struct LaunchOptions {
    how: HowLaunch,
}

impl LaunchOptions {
    pub fn new(how: HowLaunch) -> Self {
	Self {
	    how,
	}
    }
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

#[derive(Debug)]
pub struct VastatrixRequest {
    responder: Arc<Sender<VastatrixResponse>>,
    kind: VastatrixRequestKind,
}

unsafe impl Send for VastatrixRequest {}

#[derive(Debug)]
pub enum VastatrixRequestKind {
    ResolveClass(String),
    Exit(i32),
}

#[derive(Debug)]
pub enum VastatrixResponse {
    ResolvedClass(Option<Arc<dyn Class>>),
    Nothing,
}
