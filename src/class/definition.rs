use std::collections::VecDeque;

use broom::Handle;
use bytes::{Buf, Bytes};

use super::frame::Frame;
use super::method::Descriptor;
use crate::class::attribute::{Attribute, AttributeCommon};
use crate::class::method::{Argument, MethodType};
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
        println!("superclass index: {}", super_class);
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

    pub fn resolve_method(self, method_info: ConstantsPoolInfo, superclass: bool, class_in: Option<Class>, running_in: &mut Vastatrix)
                          -> (Frame, Descriptor) {
        let class_index: u16;
        let name_and_type: u16;
        if let ConstantsPoolInfo::MethodRef { class_index: cindex, name_and_type_index: ntindex, } = method_info {
            if superclass {
                class_index = class_in.as_ref().expect("superclass set without class_in?").super_class;
            } else {
                class_index = cindex;
            }
            name_and_type = ntindex;
        } else {
            panic!("method ref wasnt a method???");
        }
        let method_name: String;
        let method_desc: String;
        let name_and_type_pool = &self.constant_pool[name_and_type as usize - 1];
        if let ConstantsPoolInfo::NameAndType { name_index, descriptor_index, } = name_and_type_pool {
            let name_pool = &self.constant_pool[*name_index as usize - 1];
            let desc_pool = &self.constant_pool[*descriptor_index as usize - 1];
            if let ConstantsPoolInfo::Utf8 { length, bytes, } = name_pool {
                method_name = bytes.to_string();
            } else {
                panic!("method name was not a string!");
            }
            if let ConstantsPoolInfo::Utf8 { length, bytes, } = desc_pool {
                method_desc = bytes.to_string();
            } else {
                panic!("method desc was not a string!");
            }
        } else {
            panic!("nameandtype was not a nameandtype!");
        }
        let class: Class;
        let handle: Handle<VTXObject>;
        if superclass {
            if class_in.is_some() {
                let inclass = class_in.unwrap();
                let superclass_pool = &inclass.constant_pool[inclass.super_class as usize - 1];
                println!("superclass pool: {:?}", superclass_pool);
                if let ConstantsPoolInfo::Class { name_index, } = superclass_pool {
                    let superclass_name_pool = &inclass.constant_pool[*name_index as usize - 1];
                    if let ConstantsPoolInfo::Utf8 { length, bytes, } = superclass_name_pool {
                        handle = running_in.load_or_get_class_handle(bytes.to_string());
                        println!("new class: {}", bytes.to_string());
                        class = running_in.get_class(handle).clone();
                    } else {
                        panic!("please set class_in :(");
                    }
                } else {
                    panic!("please set class_in :(");
                }
            } else {
                panic!("please set class_in :(");
            }
        } else {
            let class_pool = &self.constant_pool[class_index as usize - 1];
            if let ConstantsPoolInfo::Class { name_index, } = &class_pool {
                let name_pool = &self.constant_pool[*name_index as usize - 1];
                if let ConstantsPoolInfo::Utf8 { length, bytes, } = name_pool {
                    handle = running_in.load_or_get_class_handle(bytes.to_string());
                    class = running_in.get_class(handle).clone();
                } else {
                    panic!();
                }
            } else {
                panic!();
            }
        }
        for method in &class.methods {
            let method_name_pool = &class.constant_pool[method.name_index as usize - 1];
            let method_desc_pool = &class.constant_pool[method.descriptor_index as usize - 1];
            let searching_name: String;
            let searching_desc: String;
            if let ConstantsPoolInfo::Utf8 { length, bytes, } = method_name_pool {
                searching_name = bytes.to_string();
            } else {
                panic!("method name was not a string!");
            }
            if let ConstantsPoolInfo::Utf8 { length, bytes, } = method_desc_pool {
                searching_desc = bytes.to_string();
            } else {
                panic!("method desc was not a string!");
            }
            println!("searching name: {}, for: {}", searching_name, method_name);
            println!("searching desc: {}, for: {}", searching_desc, method_desc);
            if searching_name == method_name && searching_desc == method_desc {
                let descriptor = Descriptor::new(method_desc.clone());
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
                        let locals: Vec<Argument> = vec![Argument::new(0, MethodType::Void); *max_locals as usize];
                        let stack: VecDeque<Argument> = vec![].into();
                        return (Frame { class_handle: handle, method: method_name.clone(), ip: 0, code: code.to_vec(), locals, stack }, descriptor);
                    }
                }
            }
        }
        return self.resolve_method(method_info, true, Some(class.clone()), running_in);
    }
}
