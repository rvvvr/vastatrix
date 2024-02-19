use byteorder::{ReadBytesExt, BigEndian};

use crate::VastatrixError;

use super::{Class, ConstantPoolInfo, FieldInfo, MethodInfo, AttributeInfo};
use thiserror::Error;

use std::io::{Read, self};

#[derive(Debug, Default)]
pub struct ClassFile {
    major_version: u16,
    minor_version: u16,
    constant_pool: Vec<ConstantPoolInfo>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<u16>,
    fields: Vec<FieldInfo>,
    methods: Vec<MethodInfo>,
    attributes: Vec<AttributeInfo>,
    
}

impl ClassFile {
    pub fn read_from(source: &mut impl Read) -> Result<Self, ClassError> {
	let major_version = source.read_u16::<BigEndian>()?;
	let minor_version = source.read_u16::<BigEndian>()?;
	let constant_pool_count = source.read_u16::<BigEndian>()?;
	let mut constant_pool: Vec<ConstantPoolInfo> = Vec::new();
	Self::fill_constant_pool(&mut constant_pool, source, constant_pool_count);
	let access_flags = source.read_u16::<BigEndian>()?;
	let this_class = source.read_u16::<BigEndian>()?;
	let super_class = source.read_u16::<BigEndian>()?;
	let interfaces_count = source.read_u16::<BigEndian>()?;
	let mut interfaces: Vec<u16> = Vec::new();
	for _ in 0..interfaces_count {
	    interfaces.push(source.read_u16::<BigEndian>()?);
	}
	let fields_count = source.read_u16::<BigEndian>()?;
	let mut fields: Vec<FieldInfo> = Vec::new();
	for _ in 0..fields_count {
	    let access_flags = source.read_u16::<BigEndian>()?;
	    let name_index = source.read_u16::<BigEndian>()? as usize;
	    let descriptor_index = source.read_u16::<BigEndian>()? as usize;
	    let attributes_count = source.read_u16::<BigEndian>()? as usize;
	    let mut attributes: Vec<AttributeInfo> = Vec::new();
	}
	Ok(Self::default())
    }

    fn fill_constant_pool(constant_pool: &mut Vec<ConstantPoolInfo>, source: &mut impl Read, count: u16) -> Result<(), ClassError> {
	for _ in 0..count {
	    let tag = source.read_u8()?;
	    match tag {
		1 => {
		    let length = source.read_u16::<BigEndian>()?;
		    let mut data = Vec::new();
		    source.read_exact(&mut data[0..(length as usize)])?;
		    constant_pool.push(ConstantPoolInfo::Utf8 {
			data
		    });
		},
		3 => {
		    let int = source.read_u32::<BigEndian>()?;
		    constant_pool.push(ConstantPoolInfo::Integer {
			int
		    });
		},
		4 => {
		    let float = source.read_f32::<BigEndian>()?;
		    constant_pool.push(ConstantPoolInfo::Float {
			float
		    });
		},
		5 => {
		    let long = source.read_u64::<BigEndian>()?;
		    constant_pool.push(ConstantPoolInfo::Long {
			long
		    });
		},
		6 => {
		    let double = source.read_f64::<BigEndian>()?;
		    constant_pool.push(ConstantPoolInfo::Double {
			double
		    });
		},
		7 => {
		    let name_index = source.read_u16::<BigEndian>()? as usize; 
		    constant_pool.push(ConstantPoolInfo::Class {
			name_index
		    });
		},
		8 => {
		    let string_index = source.read_u16::<BigEndian>()? as usize;
		    constant_pool.push(ConstantPoolInfo::String {
			string_index
		    });
		},
		9 => {
		    let class_index = source.read_u16::<BigEndian>()? as usize;
		    let name_and_type_index = source.read_u16::<BigEndian>()? as usize;
		    constant_pool.push(ConstantPoolInfo::Fieldref {
			class_index,
			name_and_type_index,
		    });
		}
		10 => {
		    let class_index = source.read_u16::<BigEndian>()? as usize;
		    let name_and_type_index = source.read_u16::<BigEndian>()? as usize;
		    constant_pool.push(ConstantPoolInfo::Methodref {
			class_index,
			name_and_type_index,
		    });
		}
		11 => {
		    let class_index = source.read_u16::<BigEndian>()? as usize;
		    let name_and_type_index = source.read_u16::<BigEndian>()? as usize;
		    constant_pool.push(ConstantPoolInfo::InterfaceMethodref {
			class_index,
			name_and_type_index,
		    });
		},
		12 => {
		    let name_index = source.read_u16::<BigEndian>()? as usize;
		    let descriptor_index = source.read_u16::<BigEndian>()? as usize;
		    constant_pool.push(ConstantPoolInfo::NameAndType {
			name_index,
			descriptor_index,
		    });
		},
		15 => {
		    let reference_kind = source.read_u8()?;
		    let reference_index = source.read_u16::<BigEndian>()? as usize;
		    constant_pool.push(ConstantPoolInfo::MethodHandle {
			reference_kind,
			reference_index,
		    });
		},
		16 => {
		    let descriptor_index = source.read_u16::<BigEndian>()? as usize;
		    constant_pool.push(ConstantPoolInfo::MethodType {
			descriptor_index
		    });
		},
		17 => {
		    let bootstrap_method_attr_index = source.read_u16::<BigEndian>()? as usize;
		    let name_and_type_index = source.read_u16::<BigEndian>()? as usize;
		    constant_pool.push(ConstantPoolInfo::Dynamic {
			bootstrap_method_attr_index,
			name_and_type_index
		    });
		},
		18 => {
		    let bootstrap_method_attr_index = source.read_u16::<BigEndian>()? as usize;
		    let name_and_type_index = source.read_u16::<BigEndian>()? as usize;
		    constant_pool.push(ConstantPoolInfo::InvokeDynamic {
			bootstrap_method_attr_index,
			name_and_type_index
		    });
		},
		19 => {
		    let name_index = source.read_u16::<BigEndian>()? as usize;
		    constant_pool.push(ConstantPoolInfo::Module {
			name_index
		    });
		},
		20 => {
		    let name_index = source.read_u16::<BigEndian>()? as usize;
		    constant_pool.push(ConstantPoolInfo::Package {
			name_index
		    });
		}
		_ => return Err(ClassError::NonexistentConstantPoolTag(tag)),
	    }
	}
	Ok(())	
    }

    fn fill_attributes(attributes: &mut Vec<AttributeInfo>, source: &mut impl Read, count: u16) -> Result<(), ClassError> {
	
	Ok(())
    }
}

impl Class for ClassFile {
    
}
    
unsafe impl Send for ClassFile {}

#[derive(Debug, Error)]
pub enum ClassError {
    #[error("IO failed! {0:?}")]
    IOError(#[from] io::Error),
    #[error("Constant pool tag {0} does not exist!")]
    NonexistentConstantPoolTag(u8),
}
