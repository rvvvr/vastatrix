pub mod classpath;
pub mod classfile;

pub trait Class: Send {
    
}

#[derive(Debug)]
pub enum ConstantPoolInfo {
    Utf8 {
	data: Vec<u8>,
    },
    Integer {
	int: u32,
    },
    Float {
	float: f32,
    },
    Long {
	long: u64,
    },
    Double {
	double: f64,
    },
    Class {
	name_index: usize,
    },
    String {
	string_index: usize,
    },
    Fieldref {
	class_index: usize,
	name_and_type_index: usize,
    },
    Methodref {
	class_index: usize,
	name_and_type_index: usize,
    },
    InterfaceMethodref {
	class_index: usize,
	name_and_type_index: usize,
    },
    NameAndType {
	name_index: usize,
	descriptor_index: usize,
    },
    MethodHandle {
	reference_kind: u8,
	reference_index: usize,
    },
    MethodType {
	descriptor_index: usize,
    },
    Dynamic {
	bootstrap_method_attr_index: usize,
	name_and_type_index: usize,
    },
    InvokeDynamic {
	bootstrap_method_attr_index: usize,
	name_and_type_index: usize,
    },
    Module {
	name_index: usize,
    },
    Package {
	name_index: usize,
    },
}

#[derive(Debug)]
pub struct FieldInfo {
    access_flags: u16,
    name_index: usize,
    descriptor_index: usize,
    attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
pub struct MethodInfo {
    access_flags: u16,
    name_index: usize,
    descriptor_index: usize,
    attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
pub enum AttributeInfo {
    
}
