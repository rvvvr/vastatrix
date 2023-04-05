mod object;
mod string;
mod system;

use object::Object;
use vastatrix::class::Class;
use std::collections::HashMap;

#[no_mangle]
pub fn expose_classes() -> HashMap<String, Box<dyn Class>> {
    let mut out: HashMap<String, Box<dyn Class>> = HashMap::new();
    out.insert("java/lang/Object".to_string(), Box::new(Object {handle: None}));
    out.insert("java/lang/String".to_string(), Box::new(crate::string::jString {handle: None}));
    out.insert("java/lang/System".to_string(), Box::new(crate::system::System {handle: None}));
    return out;
}
