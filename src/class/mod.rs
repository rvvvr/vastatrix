pub mod attribute;
pub mod classfile;
pub mod definition;
pub mod frame;
pub mod instance;
pub mod method;

pub use definition::*;

pub fn resolve_static(constant_pool: Vec<ConstantsPoolInfo>, index: u16) -> Result<String, ()> {
    if let ConstantsPoolInfo::Utf8 { bytes, .. } = &constant_pool[index as usize] {
        return Ok(bytes.to_string());
    }
    Err(())
}
