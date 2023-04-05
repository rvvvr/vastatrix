use std::collections::HashMap;

use libloading::Library;

use crate::class::Class;

pub fn load_classes_from_std(lib: &Library) -> HashMap<String, Box<dyn Class>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern "C" fn() -> HashMap<String, Box<dyn Class>>> = lib.get(b"expose_classes").unwrap();
        return func();
    }
}
