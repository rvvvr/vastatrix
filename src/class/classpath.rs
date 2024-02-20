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

    pub fn insert(&mut self, path: &str, class: Arc<dyn Class>) {
	self.root.insert(String::from(path), Arc::clone(&class));
    }

    pub fn resolve(&self, path: &str) -> Option<Arc<dyn Class>> {
	println!("{:?}", self.root.keys());
	self.root.get(path).cloned()
    }
}

