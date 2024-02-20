use std::{fmt::Debug, io::Read};

use byteorder::{ReadBytesExt, BigEndian};

pub mod classpath;
pub mod classfile;
pub mod descriptor;

pub trait Class: Send + Debug {
    
}

#[derive(Debug)]
pub enum ConstantPoolInfo {
    Utf8 {
	string: String,
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
    Dummy,
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
    ConstantValue {
	constantvalue_index: usize,
    },
    Code {
	max_stack: usize,
	max_locals: usize,
	code_length: usize,
	code: Vec<u8>,
	exception_table: Vec<ExceptionTableEntry>,
	attributes: Vec<AttributeInfo>,
    },
    StackMapTable {
	entries: Vec<StackMapFrame>,
    },
    Exceptions {
	exception_index_table: Vec<u16>,
    },
    LineNumberTable {
	line_number_table: Vec<LineNumberTableEntry>,
    },
    SourceFile {
	sourcefile_index: usize,
    },
    NestHost {
	host_class_index: usize,
    },
    NestMembers {
	classes: Vec<usize>,
    },
    InnerClasses {
	classes: Vec<InnerClassEntry>,
    },
}

#[derive(Debug)]
pub struct ExceptionTableEntry {
    pub start_pc: usize,
    pub end_pc: usize,
    pub handler_pc: usize,
    pub catch_type: usize,
}

#[derive(Debug)]
pub enum StackMapFrame {
    Same,
    SameLocals1StackItem {
	stack: VerificationType,
    },
    SameLocals1StackItemExtended {
	offset_delta: usize,
	stack: VerificationType,
    },
    Chop {
	offset_delta: usize,
    },
    SameExtended {
	offset_delta: usize,
    },
    Append {
	offset_delta: usize,
	locals: Vec<VerificationType> //could be replaced with a smallvec/tinyvec
    },
    Full {
	offset_delta: usize,
	locals: Vec<VerificationType>,
	stack: Vec<VerificationType>,
    },
}

#[derive(Debug)]
pub enum VerificationType {
    Top,
    Integer,
    Float,
    Variable,
    Null,
    UninitializedThis,
    Object {
	cpool_index: usize,
    },
    Uninitialized {
	offset: usize,
    },
    Long,
    Double,
}

impl VerificationType {
    pub fn read_from(source: &mut impl Read) -> Self {
	let tag = source.read_u8().expect("should have been able to read!");
	match tag {
	    0 => Self::Top,
	    1 => Self::Integer,
	    2 => Self::Float,
	    3 => Self::Double,
	    4 => Self::Long,
	    5 => Self::Null,
	    6 => Self::UninitializedThis,
	    7 => {
		let cpool_index = source.read_u16::<BigEndian>().expect("should have been able to read!") as usize;
		Self::Object {
		    cpool_index,
		}
	    },
	    8 => {
		let offset = source.read_u16::<BigEndian>().expect("should have been able to read!") as usize;
		Self::Uninitialized {
		    offset,
		}
	    },
	    _ => unreachable!("assuming your class file is correct, this should never happen."),
	}
    }
}


#[derive(Debug)]
pub struct LineNumberTableEntry {
    pub start_pc: usize,
    pub line_number: usize,
}

#[derive(Debug)]
pub struct InnerClassEntry {
    pub inner_class_info_index: usize,
    pub outer_class_info_index: usize,
    pub inner_name_index: usize,
    pub inner_class_access_flags: u16,
}
