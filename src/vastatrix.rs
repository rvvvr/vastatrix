use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;

use broom::trace::Trace;
use broom::Handle;
use bytes::Bytes;
use libloading::Library;
use zip::ZipArchive;

use crate::class::attribute::Attribute;
use crate::class::classfile::{BytecodeFrame, ClassFile};
use crate::class::frame::Frame;
use crate::class::instance::Instance;
use crate::class::method::{Argument, MethodType};
use crate::class::{Class, ConstantsPoolInfo};
use crate::loading;

#[derive(Debug)]
pub enum VTXObject {
    Class(Box<dyn Class>),
    Instance(Instance),
    Array((MethodType, Vec<Argument>)),
}

impl Trace<Self> for VTXObject {
    fn trace(&self, tracer: &mut broom::trace::Tracer<Self>) {
        match self {
            VTXObject::Class(_) => {},
            VTXObject::Instance(_) => {},
            VTXObject::Array(elements) => elements.1.trace(tracer),
        }
    }
}

pub struct Vastatrix {
    heap:             broom::Heap<VTXObject>,
    class_handles:    HashMap<String, Handle<VTXObject>>,
    instance_handles: Vec<Handle<VTXObject>>,
    archive:          ZipArchive<File>,
    std:              Library,
}

impl Vastatrix {
    pub fn new(archive: ZipArchive<File>) -> Self {
        let mut heap = broom::Heap::default();
        let lib = unsafe { Library::new("./vtx-std/target/debug/libvtx_std.so").unwrap() };
        let mut class_handles = HashMap::new();
        Self { heap, class_handles, instance_handles: vec![], archive, std: lib }
    }

    pub fn run(&mut self) { self.load(); }

    fn load(&mut self) {
        let std = loading::load_classes_from_std(&self.std);
        for classpath in std.keys() {
            trace!("loading: {}", classpath);
            let handle = self.heap.insert_temp(VTXObject::Class(std.get(classpath).unwrap().to_owned()));
            self.class_handles.insert(classpath.to_string(), handle);
        }
        let archive = &mut self.archive;
        let mut manifest_file = archive.by_name("META-INF/MANIFEST.MF").expect("Jar has no manifest!");
        let mut manifest = String::new();
        manifest_file.read_to_string(&mut manifest).expect("Could not get contents of manifest");
        drop(manifest_file);
        drop(archive);
        for line in manifest.lines() {
            if line.starts_with("Main-Class") {
                let split: Vec<&str> = line.split(' ').collect();
                let class = split.get(1).unwrap();
                let class_vec: Vec<&str> = class.split('.').collect();
                let class_path = class_vec.join("/");
                let handle = self.load_or_get_class_handle(class_path);
                let class = self.get_class(handle);
                let maindesc = "([Ljava/lang/String;)V".to_string();
                let mut method_info = None;
                for method in &class.get_methods() {
                    let name_pool = &class.get_constant_pool()[method.name_index as usize];
                    let desc_pool = &class.get_constant_pool()[method.descriptor_index as usize];
                    let name: String;
                    let desc: String;
                    if let ConstantsPoolInfo::Utf8 { bytes, .. } = name_pool {
                        name = bytes.to_string();
                    } else {
                        panic!("name was not a utf8!");
                    }

                    if let ConstantsPoolInfo::Utf8 { bytes, .. } = desc_pool {
                        desc = bytes.to_string();
                    } else {
                        panic!("name was not a utf8!");
                    }
                    trace!("name: {}, desc: {}", name, desc);
                    if name == "main".to_string() && desc == maindesc {
                        method_info = Some(method.clone());
                        break;
                    }
                }
                if method_info.is_none() {
                    panic!("could not find main!");
                }
                for attribute in &method_info.unwrap().attribute_info {
                    if let Attribute::Code { max_locals, code, .. } = attribute {
                        let locals: Vec<Argument> = vec![Argument::new(0, MethodType::Void); *max_locals as usize];
                        let stack: VecDeque<Argument> = vec![].into();
                        let mut frame = BytecodeFrame { class_handle: handle, method: "main".to_string(), ip: 0, code: code.to_vec(), locals, stack };
                        drop(class);
                        frame.exec(vec![], self);
                        break;
                    }
                }
            }
        }
    }

    pub fn load_or_get_class_handle(&mut self, classpath: String) -> Handle<VTXObject> {
        if self.class_handles.contains_key(&classpath) {
            return *self.class_handles.get(&classpath).unwrap();
        }
        let archive = &mut self.archive;
        info!("LOADING CLASS: {}", classpath.clone());
        let mut class_file = archive.by_name(&(classpath.clone() + ".class")).expect("Could not find class file!");
        let mut class_buf: Vec<u8> = vec![];
        class_file.read_to_end(&mut class_buf).unwrap();
        let bytes = Bytes::from(class_buf);
        let class = ClassFile::new(bytes);
        let handle = self.heap.insert_temp(VTXObject::Class(Box::new(class)));
        self.class_handles.insert(classpath, handle);
        if let VTXObject::Class(cls) = self.heap.get_mut(handle).unwrap() {
            cls.set_handle(handle);
        }
        handle
    }

    pub fn get_class(&mut self, handle: Handle<VTXObject>) -> Box<dyn Class + 'static> {
        if let VTXObject::Class(class) = self.heap.get(handle).unwrap() {
            return class.clone();
        }
        panic!("could not get class!");
    }

    pub fn prepare_instance(&mut self, class: &mut Box<dyn Class>) -> u32 {
        let mut instance = Instance::new();
        for field in &class.get_fields() {
            let name = &class.get_constant_pool()[field.name_index as usize];
            if let ConstantsPoolInfo::Utf8 { bytes, .. } = name {
                trace!("field name: {}", bytes);
                if field.access_flags & 0x0008 != 0 {
                    for attribute in &field.attribute_info {
                        if let Attribute::ConstantValue { constantvalue_index, .. } = attribute {
                            let constantvalue = &class.get_constant_pool()[*constantvalue_index as usize];
                            match constantvalue {
                                _ => panic!("constantvalue_index did not index a valid constant value!"),
                            }
                        }
                    }
                }
                instance.fields.insert(bytes.to_string(), Argument::new(0, MethodType::Void));
            }
        }
        let handle = self.heap.insert_temp(VTXObject::Instance(instance));
        self.instance_handles.push(handle);
        return self.instance_handles.len() as u32 - 1;
    }

    pub fn get_instance(&mut self, index: usize) -> &mut Instance {
        let handle = self.instance_handles.get(index).unwrap();
        let thing = self.heap.get_mut(handle).unwrap();
        if let VTXObject::Instance(instance) = thing {
            return instance;
        }
        panic!("couldn't get instance!");
    }

    pub fn create_array(&mut self, array: Vec<Argument>, of: MethodType) -> u32 {
        let handle = self.heap.insert_temp(VTXObject::Array((of, array)));
        self.instance_handles.push(handle);
        return self.instance_handles.len() as u32 - 1;
    }

    pub fn get_array(&mut self, index: usize) -> &mut (MethodType, Vec<Argument>) {
        println!("{}, {:?}", index, self.instance_handles);
        let handle = self.instance_handles.get(index).unwrap();
        if let VTXObject::Array(elements) = self.heap.get_mut(handle).unwrap() {
            return elements;
        } else {
            panic!("not an array!");
        }
    }
}
