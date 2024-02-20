use byteorder::{ReadBytesExt, BigEndian};

use crate::VastatrixError;

use super::{Class, ConstantPoolInfo, FieldInfo, MethodInfo, AttributeInfo, ExceptionTableEntry, LineNumberTableEntry, StackMapFrame, VerificationType, InnerClassEntry};
use thiserror::Error;

use std::io::{Read, self, BufRead};

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
	assert_eq!(0xCAFEBABE, source.read_u32::<BigEndian>()?);
	//unsure if these versions will be used, keeping them around for now though.
	let major_version = source.read_u16::<BigEndian>()?;
	let minor_version = source.read_u16::<BigEndian>()?;
	let constant_pool_count = source.read_u16::<BigEndian>()? - 1;
	let mut constant_pool: Vec<ConstantPoolInfo> = vec![ConstantPoolInfo::Dummy];
	Self::fill_constant_pool(&mut constant_pool, source, constant_pool_count)?;
	let access_flags = source.read_u16::<BigEndian>()?;
	let this_class = source.read_u16::<BigEndian>()?;
	let super_class = source.read_u16::<BigEndian>()?;
	let interfaces_count = source.read_u16::<BigEndian>()? as usize;
	let mut interfaces: Vec<u16> = Vec::with_capacity(interfaces_count);
	for _ in 0..interfaces_count {
	    interfaces.push(source.read_u16::<BigEndian>()?);
	}
	let fields_count = source.read_u16::<BigEndian>()? as usize;
	let mut fields: Vec<FieldInfo> = Vec::with_capacity(fields_count);
	for _ in 0..fields_count {
	    let access_flags = source.read_u16::<BigEndian>()?;
	    let name_index = source.read_u16::<BigEndian>()? as usize;
	    let descriptor_index = source.read_u16::<BigEndian>()? as usize;
	    let attributes_count = source.read_u16::<BigEndian>()? as usize;
	    let mut attributes: Vec<AttributeInfo> = Vec::with_capacity(attributes_count);
	    Self::fill_attributes(&mut attributes, &constant_pool, source, attributes_count)?;
	    fields.push(FieldInfo {
		access_flags,
		name_index,
		descriptor_index,
		attributes,
	    })
	}
	let methods_count = source.read_u16::<BigEndian>()? as usize;
	let mut methods: Vec<MethodInfo> = Vec::with_capacity(methods_count);
	for _ in 0..methods_count {
	    let access_flags = source.read_u16::<BigEndian>()?;
	    let name_index = source.read_u16::<BigEndian>()? as usize;
	    let descriptor_index = source.read_u16::<BigEndian>()? as usize;
	    let attributes_count = source.read_u16::<BigEndian>()? as usize;
	    let mut attributes: Vec<AttributeInfo> = Vec::with_capacity(attributes_count);
	    Self::fill_attributes(&mut attributes, &constant_pool, source, attributes_count)?;
	    methods.push(MethodInfo {
		access_flags,
		name_index,
		descriptor_index,
		attributes,
	    });
	}
	let attributes_count = source.read_u16::<BigEndian>()? as usize;
	let mut attributes: Vec<AttributeInfo> = Vec::with_capacity(attributes_count);
	Self::fill_attributes(&mut attributes, &constant_pool, source, attributes_count)?;
	Ok(Self {
	    major_version,
	    minor_version,
	    constant_pool,
	    access_flags,
	    this_class,
	    super_class,
	    interfaces,
	    fields,
	    methods,
	    attributes
	})
	    
    }

    fn fill_constant_pool(constant_pool: &mut Vec<ConstantPoolInfo>, source: &mut impl Read, count: u16) -> Result<(), ClassError> {
	for _ in 0..count {
	    let tag = source.read_u8()?;
	    match tag {
		1 => {
		    let length = source.read_u16::<BigEndian>()? as usize;
		    let mut data = vec![0; length];
		    source.read_exact(&mut data[0..(length)])?;
		    let mut string = String::with_capacity(data.capacity());
		    let mut data_iter = data.iter();
		    while let Some(byte) = data_iter.next() {
			if *byte & 0b10000000 == 0 {
			    string.push(char::from_u32(*byte as u32).ok_or(ClassError::MalformedUTF8Codepoint(*byte as u32))?);
			} else {
			    let x = *byte;
			    let y = *data_iter.next().ok_or(ClassError::MalformedUTF8Codepoint(x as u32))?;
			    if x & 0b11100000 == 0b11000000 && y & 0b11000000 == 0b10000000 {
				let codepoint = (((x & 0x1f) as u32) << 6) + ((y & 0x3f) as u32);
				string.push(char::from_u32(codepoint).ok_or(ClassError::MalformedUTF8Codepoint(codepoint))?);
			    } else {
				let z = *data_iter.next().ok_or(ClassError::IncompleteUTF8Codepoint(0))?;
				if x & 0b11110000 == 0b11100000 && y & 0b11000000 == 0b10000000 && z & 0b11000000 == 0b10000000 {
				    let codepoint = (((x & 0xf) as u32) << 12) + (((y & 0x3f) as u32) << 6) + ((z & 0x3f) as u32);
				    string.push(char::from_u32(*byte as u32).ok_or(ClassError::MalformedUTF8Codepoint(codepoint))?);
				} else {
				    //really don't care about characters > FFFF rn to be honest...
				    unimplemented!("supplementary characters in utf constant");
				}
			    }
			}
		    }
		    constant_pool.push(ConstantPoolInfo::Utf8 {
			string
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
		    constant_pool.extend([ConstantPoolInfo::Long {
			long
		    }, ConstantPoolInfo::Dummy]);
		},
		6 => {
		    let double = source.read_f64::<BigEndian>()?;
		    constant_pool.extend([ConstantPoolInfo::Double {
			double
		    }, ConstantPoolInfo::Dummy]);
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

    fn fill_attributes(attributes: &mut Vec<AttributeInfo>, constant_pool: &Vec<ConstantPoolInfo>, source: &mut impl Read, count: usize) -> Result<(), ClassError> {
	for _ in 0..count {
	    let constant_pool_index = source.read_u16::<BigEndian>()? as usize;
	    let _ = source.read_u32::<BigEndian>()?;
	    match Self::resolve_utf8_string(constant_pool, constant_pool_index).as_str() {
		"ConstantValue" => {
		    let constantvalue_index = source.read_u16::<BigEndian>()? as usize;
		    attributes.push(AttributeInfo::ConstantValue {
			constantvalue_index,
		    });
		},
		"Code" => {
		    let max_stack = source.read_u16::<BigEndian>()? as usize;
		    let max_locals = source.read_u16::<BigEndian>()? as usize;
		    let code_length = source.read_u32::<BigEndian>()? as usize;
		    let mut code = vec![0; code_length];
		    source.read_exact(&mut code).expect("Not enough buffer for code!");
		    let exception_table_length = source.read_u16::<BigEndian>()? as usize;
		    let mut exception_table: Vec<ExceptionTableEntry> = Vec::with_capacity(exception_table_length);
		    for _ in 0..exception_table_length {
			let start_pc = source.read_u16::<BigEndian>()? as usize;
			let end_pc = source.read_u16::<BigEndian>()? as usize;
			let handler_pc = source.read_u16::<BigEndian>()? as usize;
			let catch_type = source.read_u16::<BigEndian>()? as usize;
			exception_table.push(ExceptionTableEntry {
			    start_pc,
			    end_pc,
			    handler_pc,
			    catch_type,
			});
		    }
		    let attributes_count = source.read_u16::<BigEndian>()? as usize;
		    let mut code_attributes: Vec<AttributeInfo> = Vec::with_capacity(attributes_count);
		    Self::fill_attributes(&mut code_attributes, constant_pool, source, attributes_count)?;
		    attributes.push(AttributeInfo::Code {
			max_stack,
			max_locals,
			code_length,
			code,
			exception_table,
			attributes: code_attributes,
		    });
		},
		"LineNumberTable" => {
		    let line_number_table_length = source.read_u16::<BigEndian>()? as usize;
		    let mut line_number_table: Vec<LineNumberTableEntry> = Vec::with_capacity(line_number_table_length);
		    for _ in 0..line_number_table_length {
			let start_pc = source.read_u16::<BigEndian>()? as usize;
			let line_number = source.read_u16::<BigEndian>()? as usize;
			line_number_table.push(LineNumberTableEntry {
			    start_pc,
			    line_number,
			});
		    }
		},
	    "StackMapTable" => {
		let number_of_entries = source.read_u16::<BigEndian>()? as usize;
		let mut entries: Vec<StackMapFrame> = Vec::with_capacity(number_of_entries);
		for _ in 0..number_of_entries {
		    let frame_type = source.read_u8()?;
		    match frame_type {
			0..=63 => {
			    entries.push(StackMapFrame::Same);
			},
			64..=127 => {
			    let stack = VerificationType::read_from(source);
			    entries.push(StackMapFrame::SameLocals1StackItem {
				stack,
			    });
			},
			247 => {
			    let offset_delta = source.read_u16::<BigEndian>()? as usize;
			    let stack = VerificationType::read_from(source);
			    entries.push(StackMapFrame::SameLocals1StackItemExtended {
				offset_delta,
				stack,
			    });
			},
			248..=250 => {
			    let offset_delta = source.read_u16::<BigEndian>()? as usize;
			    entries.push(StackMapFrame::Chop {
				offset_delta,
			    });
			},
			251 => {
			    let offset_delta = source.read_u16::<BigEndian>()? as usize;
			    entries.push(StackMapFrame::SameExtended {
				offset_delta,
			    });
			},
			tag @ 252..=254 => {
			    let offset_delta = source.read_u16::<BigEndian>()? as usize;
			    let locals_count = tag as usize - 251;
			    let mut locals: Vec<VerificationType> = Vec::with_capacity(locals_count);
			    for _ in 0..locals_count {
				locals.push(VerificationType::read_from(source));
			    }
			    entries.push(StackMapFrame::Append {
				offset_delta,
				locals,
			    });
			},
			255 => {
			    let offset_delta = source.read_u16::<BigEndian>()? as usize;
			    let number_of_locals = source.read_u16::<BigEndian>()? as usize;
			    let mut locals: Vec<VerificationType> = Vec::with_capacity(number_of_locals);
			    for _ in 0..number_of_locals {
				locals.push(VerificationType::read_from(source));
			    }
			    let number_of_stack_items = source.read_u16::<BigEndian>()? as usize;
			    let mut stack: Vec<VerificationType> = Vec::with_capacity(number_of_stack_items);
			    for _ in 0..number_of_stack_items {
				stack.push(VerificationType::read_from(source));
			    }
			    entries.push(StackMapFrame::Full {
				offset_delta,
				locals,
				stack,
			    });
			},
			a => unimplemented!("Stack map table tag {a}"),
		    }
		}
		attributes.push(AttributeInfo::StackMapTable {
		    entries,
		});
	    },
		"SourceFile" => {
		    let sourcefile_index = source.read_u16::<BigEndian>()? as usize;
		    attributes.push(AttributeInfo::SourceFile {
			sourcefile_index,
		    });
		},
		"NestHost" => {
		    let host_class_index = source.read_u16::<BigEndian>()? as usize;
		    attributes.push(AttributeInfo::NestHost {
			host_class_index,
		    });
		},
		"NestMembers" => {
		    let number_of_classes = source.read_u16::<BigEndian>()? as usize;
		    let mut classes: Vec<usize> = Vec::with_capacity(number_of_classes);
		    for _ in 0..number_of_classes {
			classes.push(source.read_u16::<BigEndian>()? as usize);
		    }
		    attributes.push(AttributeInfo::NestMembers {
			classes,
		    });
		},
		"InnerClasses" => {
		    let number_of_classes = source.read_u16::<BigEndian>()? as usize;
		    let mut classes: Vec<InnerClassEntry> = Vec::with_capacity(number_of_classes);
		    for _ in 0..number_of_classes {
			let inner_class_info_index = source.read_u16::<BigEndian>()? as usize;
			let outer_class_info_index = source.read_u16::<BigEndian>()? as usize;
			let inner_name_index = source.read_u16::<BigEndian>()? as usize;
			let inner_class_access_flags = source.read_u16::<BigEndian>()?;
			classes.push(InnerClassEntry {
			    inner_class_info_index,
			    outer_class_info_index,
			    inner_name_index,
			    inner_class_access_flags,
			});
		    }
		    attributes.push(AttributeInfo::InnerClasses {
			classes,
		    });
		},
		a => unimplemented!("Attribute name {a}"),
	    }
	}
	Ok(())
    }

    fn resolve_utf8_string(constant_pool: &Vec<ConstantPoolInfo>, index: usize) -> String {
	if let Some(ConstantPoolInfo::Utf8 { string }) = constant_pool.get(index) {
	    string.to_string()
	} else {
	    String::new()
	}
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
    #[error("Malformed UTF8 codepoint: {0:x}")]
    MalformedUTF8Codepoint(u32),
    #[error("Incomplete UTF8 codeoint: {0:x}")]
    IncompleteUTF8Codepoint(u32),
}
