use std::collections::VecDeque;

use broom::Handle;
use bytes::{Buf, Bytes};

use super::attribute::Attribute;
use super::frame::Frame;
use super::method::Descriptor;
use super::{Class, ConstantsPoolInfo, FieldInfo, MethodInfo};
use crate::class::attribute::AttributeCommon;
use crate::class::method::{Argument, MethodType};
use crate::vastatrix::{VTXObject, Vastatrix};

#[derive(Debug, Clone)]
pub struct ClassFile {
    magic:            u32,
    minor:            u16,
    major:            u16,
    constant_count:   u16,
    constant_pool:    Vec<ConstantsPoolInfo>,
    access_flags:     u16,
    this_class:       u16,
    super_class:      u16,
    interfaces_count: u16,
    interfaces:       Vec<u16>,
    fields_count:     u16,
    fields:           Vec<FieldInfo>,
    methods_count:    u16,
    methods:          Vec<MethodInfo>,
    attribute_count:  u16,
    attributes:       Vec<Attribute>,
    handle:           Option<Handle<VTXObject>>,
}

impl ClassFile {
    pub fn new(mut bytes: Bytes) -> Self {
        let magic = bytes.get_u32();
        trace!("MAGIC: {:x}", magic);
        let minor = bytes.get_u16();
        trace!("MINOR: {}", minor);
        let major = bytes.get_u16();
        trace!("MAJOR: {}", major);
        let constant_count = bytes.get_u16() - 1;
        trace!("CONSTANT COUNT: {}", constant_count);
        let mut constant_pool: Vec<ConstantsPoolInfo> = vec![];
        for _ in 0..constant_count {
            let tag = bytes.get_u8();
            trace!("TAG NUMBER: {}", tag);
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
            trace!("CONSTANT: {:?}", constant_pool.last().unwrap());
        }
        trace!("CONSTANT POOL: {:?}", constant_pool);
        let access_flags = bytes.get_u16();
        let this_class = bytes.get_u16();
        let super_class = bytes.get_u16();
        trace!("superclass index: {}", super_class);
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
        trace!("fields: {:?}", fields);
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
            trace!("method info: {:?}", attribute_info);
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
}

impl Class for ClassFile {
    fn set_handle(&mut self, handle: Handle<VTXObject>) { self.handle = Some(handle); }

    fn resolve(&self, constant_pool: Vec<ConstantsPoolInfo>, index: u16) -> Result<String, ()> {
        if let ConstantsPoolInfo::Utf8 { bytes, .. } = &constant_pool[index as usize - 1] { Ok(bytes.to_string()) } else { Err(()) }
    }

    fn resolve_method(&self, method_info: ConstantsPoolInfo, superclass: bool, class_in: Option<Box<&dyn Class>>, running_in: &mut Vastatrix)
                      -> (Box<(dyn Frame + 'static)>, Descriptor) {
        let class_index: u16;
        let name_and_type: u16;
        if let ConstantsPoolInfo::MethodRef { class_index: cindex, name_and_type_index: ntindex, } = method_info {
            if superclass {
                class_index = class_in.as_ref().expect("superclass set without class_in?").get_super_class();
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
            if let ConstantsPoolInfo::Utf8 { bytes, .. } = name_pool {
                method_name = bytes.to_string();
            } else {
                panic!("method name was not a string!");
            }
            if let ConstantsPoolInfo::Utf8 { bytes, .. } = desc_pool {
                method_desc = bytes.to_string();
            } else {
                panic!("method desc was not a string!");
            }
        } else {
            panic!("nameandtype was not a nameandtype!");
        }
        let handle: Handle<VTXObject>;
        let class = if superclass {
            if class_in.is_some() {
                let inclass = class_in.unwrap();
                let superclass_pool = &inclass.get_constant_pool()[inclass.get_super_class() as usize - 1];
                trace!("superclass pool: {:?}", superclass_pool);
                if let ConstantsPoolInfo::Class { name_index, } = superclass_pool {
                    let superclass_name_pool = &inclass.get_constant_pool()[*name_index as usize - 1];
                    if let ConstantsPoolInfo::Utf8 { bytes, .. } = superclass_name_pool {
                        handle = running_in.load_or_get_class_handle(bytes.to_string());
                        trace!("new class: {}", bytes.to_string());
                        running_in.get_class(handle)
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
                if let ConstantsPoolInfo::Utf8 { bytes, .. } = name_pool {
                    handle = running_in.load_or_get_class_handle(bytes.to_string());
                    running_in.get_class(handle)
                } else {
                    panic!();
                }
            } else {
                panic!();
            }
        };
        let frame_maybe = class.create_frame(method_name, method_desc.clone());
        if frame_maybe.is_some() {
            return (frame_maybe.unwrap(), Descriptor::new(method_desc));
        }
        return self.resolve_method(method_info, true, Some(Box::new(class.as_ref())), running_in);
    }

    fn get_handle(&self) -> Handle<VTXObject> { self.handle.unwrap() }

    fn get_magic(&self) -> u32 { self.magic }

    fn get_minor(&self) -> u16 { self.minor }

    fn get_major(&self) -> u16 { self.major }

    fn get_constant_count(&self) -> u16 { self.constant_count }

    fn get_constant_pool(&self) -> Vec<ConstantsPoolInfo> { self.constant_pool.clone() }

    fn get_access_flags(&self) -> u16 { self.access_flags }

    fn get_this_class(&self) -> u16 { self.this_class }

    fn get_super_class(&self) -> u16 { self.super_class }

    fn get_interface_count(&self) -> u16 { self.interfaces_count }

    fn get_interfaces(&self) -> Vec<u16> { self.interfaces.clone() }

    fn get_field_count(&self) -> u16 { self.fields_count }

    fn get_fields(&self) -> Vec<FieldInfo> { self.fields.clone() }

    fn get_method_count(&self) -> u16 { self.methods_count }

    fn get_methods(&self) -> Vec<MethodInfo> { self.methods.clone() }

    fn get_attribute_count(&self) -> u16 { self.attribute_count }

    fn get_attributes(&self) -> Vec<Attribute> { self.attributes.clone() }

    fn create_frame(&self, name: String, desc: String) -> Option<Box<dyn Frame>> {
        for method in &self.methods {
            let method_name_pool = &self.constant_pool[method.name_index as usize - 1];
            let method_desc_pool = &self.constant_pool[method.descriptor_index as usize - 1];
            let method_name = if let ConstantsPoolInfo::Utf8 { bytes, .. } = method_name_pool {
                bytes.to_string()
            } else {
                panic!("method name was not a string!");
            };
            let method_desc = if let ConstantsPoolInfo::Utf8 { bytes, .. } = method_desc_pool {
                bytes.to_string()
            } else {
                panic!("method name was not a string!");
            };
            if method_name == name && method_desc == desc {
                for attribute in &method.attribute_info {
                    if let Attribute::Code { max_locals, code, .. } = attribute {
                        let locals: Vec<Argument> = vec![Argument::new(0, MethodType::Void); *max_locals as usize];
                        let stack: VecDeque<Argument> = vec![].into();
                        return Some(Box::new(BytecodeFrame { class_handle: self.handle.unwrap(),
                                                             method: method_name,
                                                             ip: 0,
                                                             code: code.to_vec(),
                                                             locals,
                                                             stack }));
                    }
                }
            }
        }
        return None;
    }
}

#[derive(Debug)]
pub struct BytecodeFrame {
    pub class_handle: Handle<VTXObject>,
    pub method:       String,
    pub ip:           u32,
    pub code:         Vec<u8>,
    pub locals:       Vec<Argument>,
    pub stack:        VecDeque<Argument>,
}

impl Frame for BytecodeFrame {
    fn exec(&mut self, args: Vec<Argument>, running_in: &mut Vastatrix) -> Argument {
        // either its a 32 bit int or its a void, type checking should catch this (in
        // the future, for now i'm just relying on the compiler) would rather
        // not do JIT yet...
        trace!("Method: {}, locals len: {}", self.method, self.locals.len());
        for index in 0..args.len() {
            self.locals[index] = args[index].clone();
        }
        loop {
            let op = self.code[self.ip as usize];
            let class = running_in.get_class(self.class_handle);
            let this_class = &class.get_constant_pool()[class.get_this_class() as usize - 1];
            if let ConstantsPoolInfo::Class { name_index, } = this_class {
                let name = &class.get_constant_pool()[*name_index as usize - 1];
                if let ConstantsPoolInfo::Utf8 { bytes, .. } = name {
                    debug!("class: {}, method: {}, opcode: 0x{:x}, current stack:{:?}", bytes.to_string(), self.method, op, self.stack);
                }
            }
            drop(this_class);
            match op {
                0x2 => {
                    // iconst_m1
                    self.stack.push_back(Argument::new(-1, MethodType::Int));
                },
                0x3 => {
                    // iconst_0
                    self.stack.push_back(Argument::new(0, MethodType::Int));
                },
                0x4 => {
                    // iconst_1
                    self.stack.push_back(Argument::new(1, MethodType::Int));
                },
                0x5 => {
                    // iconst_2
                    self.stack.push_back(Argument::new(2, MethodType::Int));
                },
                0x6 => {
                    // iconst_3
                    self.stack.push_back(Argument::new(3, MethodType::Int));
                },
                0x7 => {
                    self.stack.push_back(Argument::new(4, MethodType::Int));
                },
                0x8 => {
                    self.stack.push_back(Argument::new(5, MethodType::Int));
                },
                0x12 => {
                    let index = self.code[self.ip as usize + 1];
                    let class = running_in.get_class(self.class_handle);
                    let constant = &class.get_constant_pool()[index as usize - 1];
                    match constant {
                        ConstantsPoolInfo::Integer { bytes, } => {
                            self.stack.push_back(Argument::new(*bytes as i32, MethodType::Int));
                        },
                        a => {
                            panic!("BAD! {:?}", a);
                        },
                    }
                },
                0x15 => {
                    self.ip += 1;
                    self.stack.push_back(self.locals[self.code[self.ip as usize] as usize].clone());
                },
                0x1A => {
                    // iload_0
                    self.stack.push_back(self.locals[0].clone());
                },
                0x1B => {
                    // iload_1
                    self.stack.push_back(self.locals[1].clone());
                },
                0x1C => {
                    self.stack.push_back(self.locals[2].clone());
                },
                0x1D => {
                    self.stack.push_back(self.locals[3].clone());
                },
                0x2A => {
                    self.stack.push_back(self.locals[0].clone());
                },
                0x36 => {
                    // istore     index
                    let value = self.stack.pop_front().unwrap();
                    self.ip += 1;
                    self.locals[self.code[self.ip as usize] as usize] = value;
                },
                0x3B => {
                    // istore_0
                    let value = self.stack.pop_front().unwrap();
                    self.locals[0] = value;
                },
                0x3C => {
                    let value = self.stack.pop_front().unwrap();
                    self.locals[1] = value;
                },
                0x3D => {
                    let value = self.stack.pop_front().unwrap();
                    self.locals[2] = value;
                },
                0x3E => {
                    let value = self.stack.pop_front().unwrap();
                    self.locals[3] = value;
                },
                0x4B => {
                    let value = self.stack.pop_front().unwrap();
                    trace!("LOCALS LENGTH: {}", self.locals.len());
                    self.locals[0] = value;
                },
                0x57 => {
                    self.stack.pop_front().unwrap();
                },
                0x59 => {
                    let value = &self.stack[0];
                    self.stack.push_back(value.clone());
                },
                0x60 => {
                    // iadd
                    let a = self.stack.pop_front().unwrap();
                    let b = self.stack.pop_front().unwrap();
                    self.stack.push_back(b.wrapping_iadd(a));
                },
                0x64 => {
                    // isub
                    let a = self.stack.pop_front().unwrap();
                    let b = self.stack.pop_front().unwrap();
                    self.stack.push_back(b.wrapping_isub(a));
                },
                0x68 => {
                    // imul
                    let a = self.stack.pop_front().unwrap();
                    let b = self.stack.pop_front().unwrap();
                    self.stack.push_back(b.wrapping_imul(a));
                },
                0x6C => {
                    // idiv
                    let a = self.stack.pop_front().unwrap();
                    let b = self.stack.pop_front().unwrap();
                    self.stack.push_back(b.wrapping_idiv(a));
                },
                0x84 => {
                    let index = self.code[(self.ip + 1) as usize];
                    let cons_t = self.code[(self.ip + 2) as usize];
                    self.locals[index as usize] += cons_t as i32;
                    self.ip += 2;
                },
                0xA7 => {
                    let branchbyte1 = self.code[(self.ip + 1) as usize];
                    trace!("{}", branchbyte1);
                    let branchbyte2 = self.code[(self.ip + 2) as usize];
                    trace!("{}", branchbyte2);
                    self.ip = self.ip.checked_add_signed((((((branchbyte1 as u16) << 8) | branchbyte2 as u16) - 1) as i16).into()).unwrap();
                    trace!("{:?}", self.code);
                },
                0xAC => {
                    // ireturn
                    let v = self.stack.pop_front().unwrap();
                    return v;
                },
                0xA2 => {
                    // if_icmpge    brancbyte1      branchbyte2
                    let value1 = self.stack.pop_front().unwrap();
                    let value2 = self.stack.pop_front().unwrap();
                    let branchbyte1 = self.code[(self.ip + 1) as usize];
                    let branchbyte2 = self.code[(self.ip + 2) as usize];
                    if value1 >= value2 {
                        self.ip += (((branchbyte1 as u32) << 8) | branchbyte2 as u32) - 1;
                    } else {
                        self.ip += 2;
                    }
                },
                0xB1 => {
                    return Argument::new(0, MethodType::Void);
                },
                0xB4 => {
                    let indexbyte1 = self.code[(self.ip + 1) as usize];
                    let indexbyte2 = self.code[(self.ip + 2) as usize];
                    let mut objectref = self.stack.pop_front().unwrap();
                    let this_class = running_in.get_class(self.class_handle).clone();
                    let instance = running_in.get_instance(objectref.value_ref() as usize);
                    let field_info = &this_class.get_constant_pool()[(((indexbyte1 as usize) << 8) | indexbyte2 as usize) - 1];
                    if let ConstantsPoolInfo::FieldRef { name_and_type_index, ..} = field_info {
                        //let class = &this_class.get_constant_pool()[*class_index as usize - 1];
                        /*if let ConstantsPoolInfo::Class { name_index } = class {
                            let class_name = &this_class.constant_pool[*name_index as usize - 1];
                            if let ConstantsPoolInfo::Utf8 { length, bytes } = class_name {
                                let class_handle = running_in.load_or_get_class_handle(bytes.to_string());
                                let that_class = running_in.get_class(class_handle); // don't know if i need this right now.
                            }
                        }*/
                        let name_and_type = &this_class.get_constant_pool()[*name_and_type_index as usize - 1];
                        if let ConstantsPoolInfo::NameAndType { name_index, .. } = name_and_type {
                            let name = &this_class.get_constant_pool()[*name_index as usize - 1];
                            if let ConstantsPoolInfo::Utf8 { bytes, .. } = name {
                                self.stack.push_back(instance.fields.get(&bytes.to_string()).expect("a").clone());
                            }
                        }
                    }
                    self.ip += 2;
                },
                0xB5 => {
                    let indexbyte1 = self.code[(self.ip + 1) as usize];
                    let indexbyte2 = self.code[(self.ip + 2) as usize];
                    let mut objectref = self.stack.pop_front().unwrap();
                    let value = self.stack.pop_front().unwrap();
                    let this_class = running_in.get_class(self.class_handle).clone();
                    let instance = running_in.get_instance(objectref.value_ref() as usize);
                    let field_info = &this_class.get_constant_pool()[(((indexbyte1 as usize) << 8) | indexbyte2 as usize) - 1];
                    if let ConstantsPoolInfo::FieldRef { name_and_type_index, .. } = field_info {
                        //let class = &this_class.get_constant_pool()[*class_index as usize - 1];
                        /*if let ConstantsPoolInfo::Class { name_index } = class {
                            let class_name = &this_class.constant_pool[*name_index as usize - 1];
                            if let ConstantsPoolInfo::Utf8 { length, bytes } = class_name {
                                let class_handle = running_in.load_or_get_class_handle(bytes.to_string());
                                let that_class = running_in.get_class(class_handle); // don't know if i need this right now.
                            }
                        }*/
                        let name_and_type = &this_class.get_constant_pool()[*name_and_type_index as usize - 1];
                        if let ConstantsPoolInfo::NameAndType { name_index, .. } = name_and_type {
                            let name = &this_class.get_constant_pool()[*name_index as usize - 1];
                            if let ConstantsPoolInfo::Utf8 { bytes, .. } = name {
                                instance.fields.insert(bytes.to_string(), value);
                            }
                        }
                    }
                    self.ip += 2;
                },
                0xB6 => {
                    let indexbyte1 = self.code[(self.ip + 1) as usize];
                    let indexbyte2 = self.code[(self.ip + 2) as usize];
                    let this_class = running_in.get_class(self.class_handle).clone();
                    let objectref = self.stack.pop_front().unwrap();
                    let method_info = &this_class.get_constant_pool()[(((indexbyte1 as usize) << 8) | indexbyte2 as usize) - 1];
                    if let ConstantsPoolInfo::MethodRef { .. } = method_info {
                        let (mut method, desc) = this_class.resolve_method(method_info.clone(), false, None, running_in);
                        let mut args: Vec<Argument> = vec![objectref];
                        for _ in desc.types {
                            args.push(self.stack.pop_front().unwrap());
                        }
                        let back = method.exec(args, running_in);
                        if !back.void() {
                            self.stack.push_back(back);
                        }
                    }

                    self.ip += 2;
                },
                0xB7 => {
                    let indexbyte1 = self.code[(self.ip + 1) as usize];
                    let indexbyte2 = self.code[(self.ip + 2) as usize];
                    let this_class = running_in.get_class(self.class_handle).clone();
                    let objectref = self.stack.pop_front().unwrap();
                    let method_info = &this_class.get_constant_pool()[(((indexbyte1 as usize) << 8) | indexbyte2 as usize) - 1];
                    if let ConstantsPoolInfo::MethodRef { .. } = method_info {
                        let (mut method, desc) = this_class.resolve_method(method_info.clone(), false, None, running_in);
                        let mut args: Vec<Argument> = vec![objectref];
                        for _ in desc.types {
                            args.push(self.stack.pop_front().unwrap());
                        }
                        let back = method.exec(args, running_in);
                        if !back.void() {
                            self.stack.push_back(back);
                        }
                    }
                    self.ip += 2;
                },
                0xB8 => {
                    let indexbyte1 = self.code[(self.ip + 1) as usize];
                    let indexbyte2 = self.code[(self.ip + 2) as usize];
                    trace!("byte1: {}, byte2: {}, final: {}", indexbyte1, indexbyte2, (((indexbyte1 as usize) << 8) | indexbyte2 as usize) - 1);
                    let this_class = running_in.get_class(self.class_handle).clone();
                    let method_info = &this_class.get_constant_pool()[(((indexbyte1 as usize) << 8) | indexbyte2 as usize) - 1]; // i have to asssume that indices in terms of the internals of the jvm start at 1, otherwise i have no idea why i'd have to subtract 1 here.
                    if let ConstantsPoolInfo::MethodRef { .. } = method_info {
                        let (mut method, desc) = this_class.resolve_method(method_info.clone(), false, None, running_in);
                        let mut args: Vec<Argument> = vec![];
                        for _ in desc.types {
                            args.push(self.stack.pop_front().unwrap());
                        }
                        let back = method.exec(args, running_in);
                        if !back.void() {
                            self.stack.push_back(back);
                        }
                    } else {
                        panic!("invokestatic was not a method reference!");
                    }
                    self.ip += 2;
                },
                0xBB => {
                    let indexbyte1 = self.code[(self.ip + 1) as usize];
                    let indexbyte2 = self.code[(self.ip + 2) as usize];
                    let this_class = running_in.get_class(self.class_handle).clone();
                    let class_info = &this_class.get_constant_pool()[(((indexbyte1 as usize) << 8) | indexbyte2 as usize) - 1];
                    if let ConstantsPoolInfo::Class { name_index, } = class_info {
                        let name = &this_class.get_constant_pool()[*name_index as usize - 1];
                        if let ConstantsPoolInfo::Utf8 { bytes, .. } = name {
                            let handle = running_in.load_or_get_class_handle(bytes.to_string());
                            let mut class = running_in.get_class(handle).clone();
                            self.stack.push_back(Argument::new(running_in.prepare_instance(&mut class),
                                                               MethodType::ClassReference { classpath: bytes.to_string(), }));
                        }
                    }
                    self.ip += 2;
                },
                _ => {
                    panic!("Unimplemented opcode: 0x{:x}", op);
                },
            }
            self.ip += 1;
        }
    }
}
