use bytes::Bytes;

use super::Class;


pub enum Attribute {
    ConstantValue { common: AttributeCommon, constantvalue_index: u16 },
    Code { common: AttributeCommon, }
}

pub struct AttributeCommon {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
}

pub enum AttributeLocation {
    ClassFile,
    FieldInfo,
    MethodInfo,
    RecordComponentInfo,
    Code
}

impl Attribute {
    pub fn parse(bytes: Bytes, class: Class, location: AttributeLocation) -> Attribute {

    }
}