use std::io::BufRead;

use byteorder::ReadBytesExt;



#[derive(Debug)]
pub enum FieldDescriptor {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    ClassReference(String),
    Short,
    Boolean,
    ArrayReference(Box<FieldDescriptor>),
}

impl FieldDescriptor {
    pub fn read_from(reader: &mut impl BufRead) -> Option<Self> {
	let first = reader.read_u8().unwrap();
	match first {
	    b'B' => Some(Self::Byte),
	    b'C' => Some(Self::Char),
	    b'D' => Some(Self::Double),
	    b'F' => Some(Self::Float),
	    b'I' => Some(Self::Int),
	    b'J' => Some(Self::Long),
	    b'L' => Some(Self::read_class_reference(reader)),
	    b'S' => Some(Self::Short),
	    b'Z' => Some(Self::Boolean),
	    b'[' => Some(Self::ArrayReference(Box::new(Self::read_from(reader).unwrap()))),
	    _ => None
	}
    }

    fn read_class_reference(reader: &mut impl BufRead) -> Self {
	let mut bytes = Vec::new();
	reader.read_until(b';', &mut bytes).expect("Invalid class reference");
	Self::ClassReference(String::from_utf8(bytes).expect("Valid utf8? uhhh.. I think so!!!"))
    }
}

#[derive(Debug)]
pub struct MethodDescriptor {
    parameters: Vec<FieldDescriptor>,
    returns: Option<FieldDescriptor>,
}

impl MethodDescriptor {
    pub fn read_from(reader: &mut impl BufRead) -> Self {
	let first = reader.read_u8().expect("should be able to read...");
	if first != b'(' {
	    todo!("Malformed method descriptor!");
	}
	let mut parameters = Vec::new();
	while let Some(descriptor) = FieldDescriptor::read_from(reader) {
	    parameters.push(descriptor);
	};
	MethodDescriptor {
	    parameters,
	    returns: FieldDescriptor::read_from(reader),
	}
    }
}

