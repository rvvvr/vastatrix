use std::io::{Read, BufReader, BufRead};

use super::JarError;

#[derive(Clone)]
pub struct JarMeta {
    pub main_class: Option<String>,
}

impl JarMeta {
    pub fn from_manifest(manifest: &mut impl BufRead) -> Result<Self, JarError> {
	let lines = manifest.lines();
	let mut main_class = None;
	for line in lines {
	    let real = line?;
	    if real.starts_with("Main-Class") {
		let class = real.split(":").nth(1).unwrap().trim();
		main_class = Some(class.to_string());
	    }
	}
	Ok(Self {
	    main_class
	})
    }
}
