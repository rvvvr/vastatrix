use std::{iter::Peekable, sync::Arc, collections::HashMap};

use crate::VastatrixError;

use super::Class;

//i wanted this to be a tree-like structure but i couldn't figure out the typeness of that.
//might come back to it at some point if i get an idea.
pub struct Classpath {
    root: HashMap<String, Arc<dyn Class>>,
}

impl Classpath {
    pub fn new() -> Self {
	Self {
	    root: HashMap::new(),
	}
    }

    pub fn insert(mut self, path: &str, class: Arc<dyn Class>) -> Result<(), VastatrixError> {
	self.root.insert(String::from(path), Arc::clone(&class));
	Ok(())
    }

}

