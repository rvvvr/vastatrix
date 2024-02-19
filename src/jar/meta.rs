use std::io::Read;

use super::JarError;

pub struct JarMeta {

}

impl JarMeta {
    pub fn from_manifest(manifest: impl Read) -> Result<Self, JarError> {
	Ok(Self {
	    
	})
    }
}
