use std::collections::HashMap;

#[derive(Debug)]
pub struct Instance {
    pub fields: HashMap<String, Option<i32>>,
}

impl Instance {
    pub fn new() -> Self { Self { fields: HashMap::new(), } }
}
