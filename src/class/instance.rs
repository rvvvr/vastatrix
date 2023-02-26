use std::collections::HashMap;

use super::method::Argument;

#[derive(Debug)]
pub struct Instance {
    pub fields: HashMap<String, Argument>,
}

impl Instance {
    pub fn new() -> Self { Self { fields: HashMap::new(), } }
}
