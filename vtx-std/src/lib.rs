mod exception;
mod object;
mod string;
mod system;
mod throwable;
mod runtimeexception;
mod illegalstateexception;
mod integer;
mod class;
mod method;
mod path;

use std::collections::HashMap;

use object::Object;
use vastatrix::class::Class;

#[no_mangle]
pub fn expose_classes() -> HashMap<String, Box<dyn Class>> {
    let mut out: HashMap<String, Box<dyn Class>> = HashMap::new();
    out.insert("java/lang/Object".to_string(), Box::new(crate::object::Object { handle: None, }));
    out.insert("java/lang/String".to_string(), Box::new(crate::string::jString { handle: None, }));
    out.insert("java/lang/System".to_string(), Box::new(crate::system::System { handle: None, }));
    out.insert("java/lang/Throwable".to_string(), Box::new(crate::throwable::Throwable { handle: None, }));
    out.insert("java/lang/Exception".to_string(), Box::new(crate::exception::Exception { handle: None, }));
    out.insert("java/lang/RuntimeException".to_string(), Box::new(crate::runtimeexception::RuntimeException { handle: None, }));
    out.insert("java/lang/IllegalStateException".to_string(), Box::new(crate::illegalstateexception::IllegalStateException { handle: None, }));
    out.insert("java/lang/Integer".to_string(), Box::new(crate::integer::Integer { handle: None }));
    out.insert("java/lang/Class".to_string(), Box::new(crate::class::jClass { handle: None }));
    out.insert("java/lang/reflect/Method".to_string(), Box::new(crate::method::Method { handle: None }));
    out.insert("java/nio/file/Path".to_string(), Box::new(crate::path::Path { handle: None }));
    return out;
}
