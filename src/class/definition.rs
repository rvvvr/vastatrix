use broom::Handle;
use bytes::{Buf, Bytes};

use super::frame::Frame;
use super::method::Descriptor;
use crate::class::attribute::{Attribute, AttributeCommon};
use crate::vastatrix::{VTXObject, Vastatrix};

#[derive(Debug, Clone)]
pub struct Class {
    pub magic:            u32,
    pub minor:            u16,
    pub major:            u16,
    pub constant_count:   u16,
    pub constant_pool:    Vec<ConstantsPoolInfo>,
    pub access_flags:     u16,
    pub this_class:       u16,
    pub super_class:      u16,
    pub interfaces_count: u16,
    pub interfaces:       Vec<u16>,
    pub fields_count:     u16,
    pub fields:           Vec<FieldInfo>,
    pub methods_count:    u16,
    pub methods:          Vec<MethodInfo>,
    pub attribute_count:  u16,
    pub attributes:       Vec<Attribute>,
    handle:               Option<Handle<VTXObject>>,
}

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum ConstantsPoolInfo {
    Utf8 { length: u16, bytes: String, } = 1,
    Integer { bytes: u32, } = 3,
    Float { bytes: u32, } = 4,
    Long { high_bytes: u32, low_bytes: u32, } = 5,
    Double { high_bytes: u32, low_bytes: u32, } = 6,
    Class { name_index: u16, } = 7,
    String { string_index: u16, } = 8,
    FieldRef { class_index: u16, name_and_type_index: u16, } = 9,
    MethodRef { class_index: u16, name_and_type_index: u16, } = 10,
    InterfaceMethodRef { class_index: u16, name_and_type_index: u16, } = 11,
    NameAndType { name_index: u16, descriptor_index: u16, } = 12,
    MethodHandle { reference_kind: u8, reference_index: u16, } = 15,
    MethodType { descriptor_index: u16, } = 16,
    Dynamic { bootstrap_method_attr_index: u16, name_and_type_index: u16, } = 17,
    InvokeDynamic { bootstrap_method_attr_index: u16, name_and_type_index: u16, } = 18,
    Module { name_index: u16, } = 19,
    Package { name_index: u16, } = 20,
}

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub access_flags:     u16,
    pub name_index:       u16,
    pub descriptor_index: u16,
    pub attribute_count:  u16,
    pub attribute_info:   Vec<Attribute>,
}

#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub access_flags:     u16,
    pub name_index:       u16,
    pub descriptor_index: u16,
    pub attribute_count:  u16,
    pub attribute_info:   Vec<Attribute>,
}

#[derive(Debug, Clone)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_length:     u32,
    pub info:                 Vec<u8>,
}

impl Class {
    pub fn new(mut bytes: Bytes) -> Self {
        let magic = bytes.get_u32();
        println!("MAGIC: {:x}", magic);
        let minor = bytes.get_u16();
        println!("MINOR: {}", minor);
        let major = bytes.get_u16();
        println!("MAJOR: {}", major);
        let constant_count = bytes.get_u16() - 1;
        println!("CONSTANT COUNT: {}", constant_count);
        let mut constant_pool: Vec<ConstantsPoolInfo> = vec![];
        for _ in 0..constant_count {
            let tag = bytes.get_u8();
            println!("TAG NUMBER: {}", tag);
            constant_pool.push(match tag {
                             1 => {
                                 let length = bytes.get_u16();
                                 let bs = bytes.clone().take(length as usize);
                                 bytes.advance(length as usize);
                                 let str = std::str::from_utf8(bs.chunk()).unwrap().to_string();
                                 ConstantsPoolInfo::Utf8 { length, bytes: str }
                             },
                             3 => ConstantsPoolInfo::Integer { bytes: bytes.get_u32(), },
                             4 => ConstantsPoolInfo::Float { bytes: bytes.get_u32(), },
                             5 => ConstantsPoolInfo::Long { high_bytes: bytes.get_u32(), low_bytes: bytes.get_u32(), },
                             6 => ConstantsPoolInfo::Double { high_bytes: bytes.get_u32(), low_bytes: bytes.get_u32(), },
                             7 => ConstantsPoolInfo::Class { name_index: bytes.get_u16(), },
                             8 => ConstantsPoolInfo::String { string_index: bytes.get_u16(), },
                             9 => ConstantsPoolInfo::FieldRef { class_index: bytes.get_u16(), name_and_type_index: bytes.get_u16(), },
                             10 => ConstantsPoolInfo::MethodRef { class_index: bytes.get_u16(), name_and_type_index: bytes.get_u16(), },
                             11 =>
                                 ConstantsPoolInfo::InterfaceMethodRef { class_index: bytes.get_u16(), name_and_type_index: bytes.get_u16(), },
                             12 => ConstantsPoolInfo::NameAndType { name_index: bytes.get_u16(), descriptor_index: bytes.get_u16(), },
                             15 => ConstantsPoolInfo::MethodHandle { reference_kind: bytes.get_u8(), reference_index: bytes.get_u16(), },
                             16 => ConstantsPoolInfo::MethodType { descriptor_index: bytes.get_u16(), },
                             17 => ConstantsPoolInfo::Dynamic { bootstrap_method_attr_index: bytes.get_u16(),
                                                                name_and_type_index:         bytes.get_u16(), },
                             18 => ConstantsPoolInfo::InvokeDynamic { bootstrap_method_attr_index: bytes.get_u16(),
                                                                      name_and_type_index:         bytes.get_u16(), },
                             19 => ConstantsPoolInfo::Module { name_index: bytes.get_u16(), },
                             20 => ConstantsPoolInfo::Package { name_index: bytes.get_u16(), },
                             _ => panic!("invalid constant pool tag {}", tag),
                         });
            println!("CONSTANT: {:?}", constant_pool.last().unwrap());
        }
        println!("CONSTANT POOL: {:?}", constant_pool);
        let access_flags = bytes.get_u16();
        let this_class = bytes.get_u16();
        let super_class = bytes.get_u16();
        let interfaces_count = bytes.get_u16();
        let mut interfaces = vec![];
        for _ in 0..interfaces_count {
            interfaces.push(bytes.get_u16());
        }
        let fields_count = bytes.get_u16();
        let mut fields = vec![];
        for _ in 0..fields_count {
            let aflags = bytes.get_u16();
            let namedex = bytes.get_u16();
            let descdex = bytes.get_u16();
            let attribute_count = bytes.get_u16();
            let mut attribute_info = vec![];
            for _ in 0..attribute_count {
                let attribute_name_index = bytes.get_u16();
                let attribute_length = bytes.get_u32();
                let common = AttributeCommon { attribute_name_index, attribute_length };
                attribute_info.push(Attribute::parse(bytes.copy_to_bytes(attribute_length as usize),
                                                     constant_pool.clone(),
                                                     common,
                                                     crate::class::attribute::AttributeLocation::FieldInfo))
                // let mut info = vec![];
                // for _ in 0..attribute_length {
                //     info.push(bytes.get_u8());
                // }
                // attribute_info.push(AttributeInfo {attribute_name_index,
                // attribute_length, info});
            }
            fields.push(FieldInfo { access_flags: aflags, name_index: namedex, descriptor_index: descdex, attribute_count, attribute_info })
        }
        println!("fields: {:?}", fields);
        let methods_count = bytes.get_u16();
        let mut methods = vec![];
        for _ in 0..methods_count {
            let aflags = bytes.get_u16();
            let namedex = bytes.get_u16();
            let descdex = bytes.get_u16();
            let attribute_count = bytes.get_u16();
            let mut attribute_info = vec![];
            for _ in 0..attribute_count {
                let attribute_name_index = bytes.get_u16();
                let attribute_length = bytes.get_u32();
                let common = AttributeCommon { attribute_name_index, attribute_length };
                attribute_info.push(Attribute::parse(bytes.copy_to_bytes(attribute_length as usize),
                                                     constant_pool.clone(),
                                                     common,
                                                     crate::class::attribute::AttributeLocation::MethodInfo))
                // let mut info = vec![];
                // for _ in 0..attribute_length {
                //     info.push(bytes.get_u8());
                // }
                // attribute_info.push(AttributeInfo {attribute_name_index,
                // attribute_length, info});
            }
            println!("method info: {:?}", attribute_info);
            methods.push(MethodInfo { access_flags: aflags, name_index: namedex, descriptor_index: descdex, attribute_count, attribute_info })
        }
        let attribute_count = bytes.get_u16();
        let mut attributes = vec![];
        for _ in 0..attribute_count {
            let attribute_name_index = bytes.get_u16();
            let attribute_length = bytes.get_u32();
            let common = AttributeCommon { attribute_name_index, attribute_length };
            attributes.push(Attribute::parse(bytes.copy_to_bytes(attribute_length as usize),
                                             constant_pool.clone(),
                                             common,
                                             crate::class::attribute::AttributeLocation::ClassFile))
            // let mut info = vec![];
            // for _ in 0..attribute_length {
            //     info.push(bytes.get_u8());
            // }
            // attributes.push(AttributeInfo {attribute_name_index,
            // attribute_length, info});
        }
        Self { magic,
               minor,
               major,
               constant_count,
               constant_pool,
               access_flags,
               this_class,
               super_class,
               interfaces_count,
               interfaces,
               fields_count,
               fields,
               methods_count,
               methods,
               attribute_count,
               attributes,
               handle: None }
    }

    pub fn set_handle(&mut self, handle: Handle<VTXObject>) { self.handle = Some(handle); }

    pub fn resolve(constant_pool: Vec<ConstantsPoolInfo>, index: u16) -> Result<String, ()> {
        if let ConstantsPoolInfo::Utf8 { bytes, .. } = &constant_pool[index as usize - 1] { Ok(bytes.to_string()) } else { Err(()) }
    }

    pub fn resolve_method(self, method_info: ConstantsPoolInfo, superclass: bool, class_in: Option<Self>, running_in: &mut Vastatrix)
                          -> (Frame, Descriptor) {
        let class_indx: u16;
        let name_and_type_indx: u16;
        if let ConstantsPoolInfo::MethodRef { class_index, name_and_type_index, } = method_info {
            class_indx = class_index;
            name_and_type_indx = name_and_type_index;
        } else {
            panic!("Method info passed was a {:?} instead!", method_info);
        }
        let class_pool = &self.constant_pool[class_indx as usize - 1];
        let name_and_type_pool = &self.constant_pool[name_and_type_indx as usize - 1];
        let mut class: Self;
        if let ConstantsPoolInfo::Class { name_index, } = class_pool {
            let class_name = &self.constant_pool[*name_index as usize - 1];
            if let ConstantsPoolInfo::Utf8 { length, bytes, } = class_name {
                if class_in.is_some() {
                    class = class_in.unwrap();
                    if superclass {
                        let superclass_pool = &self.constant_pool[class.super_class as usize - 1];
                        if let ConstantsPoolInfo::Class { name_index, } = superclass_pool {
                            let superclass_name_pool = &self.constant_pool[*name_index as usize - 1];
                            if let ConstantsPoolInfo::Utf8 { length, bytes, } = superclass_name_pool {
                                let handle = running_in.load_or_get_class_handle(bytes.to_string());
                                class = running_in.get_class(handle).clone();
                            }
                        }
                    }
                } else {
                    let handle = running_in.load_or_get_class_handle(bytes.to_string());
                    class = running_in.get_class(handle).clone();
                }
            } else {
                panic!("fuck");
            }
        } else {
            panic!("fuck");
        }
        let name: String;
        let desc: Descriptor;
        if let ConstantsPoolInfo::NameAndType { name_index, descriptor_index, } = name_and_type_pool {
            let name_pool = &self.constant_pool[*name_index as usize - 1];
            if let ConstantsPoolInfo::Utf8 { length, bytes, } = name_pool {
                name = bytes.to_string();
            } else {
                panic!("resolver name was not a utf8");
            }
            let desc_pool = &self.constant_pool[*descriptor_index as usize - 1];
            if let ConstantsPoolInfo::Utf8 { length, bytes, } = desc_pool {
                desc = Descriptor::new(bytes.to_string());
            } else {
                panic!("resolver desc was not a utf8");
            }
        } else {
            panic!("method nameandtype was not that");
        }
        for method in &class.methods {
            let method_name_pool = &class.constant_pool[method.name_index as usize - 1];
            let method_descriptor_pool = &class.constant_pool[method.descriptor_index as usize - 1];
            let method_name: String;
            let method_desc: Descriptor;
            if let ConstantsPoolInfo::Utf8 { length, bytes, } = method_name_pool {
                method_name = bytes.to_string();
            } else {
                panic!("method name was not a utf8!");
            }
            if let ConstantsPoolInfo::Utf8 { length, bytes, } = method_descriptor_pool {
                method_desc = Descriptor::new(bytes.to_string());
            } else {
                panic!("method descriptor was not a utf8!");
            }
            if name == method_name && desc == method_desc {
                for attribute in &method.attribute_info {
                    if let Attribute::Code { common,
                                             max_stack,
                                             max_locals,
                                             code_length,
                                             code,
                                             exception_table_length,
                                             exception_table,
                                             attribute_count,
                                             attribute_info, } = attribute
                    {
                        return (Frame { class_handle: class.handle.unwrap(), method: name, ip: 0, code: code.to_vec(), locals: vec![], stack: vec![].into() },
                                desc);
                    }
                }
            }
        }
        return self.resolve_method(method_info, true, Some(class.clone()), running_in);
    }
}
