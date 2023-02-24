use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::ops::Deref;

use broom::trace::Trace;
use broom::Handle;
use bytes::Bytes;
use zip::ZipArchive;

use crate::class::attribute::Attribute;
use crate::class::instance::Instance;
use crate::class::method::MethodType;
use crate::class::{Class, ConstantsPoolInfo};

#[derive(Debug)]
pub enum VTXObject {
    Class(Class),
    Instance(Instance),
    Array(Vec<MethodType>),
}

impl Trace<Self> for VTXObject {
    fn trace(&self, tracer: &mut broom::trace::Tracer<Self>) {
        match self {
            VTXObject::Class(_) => {},
            VTXObject::Instance(_) => {},
            VTXObject::Array(elements) => elements.trace(tracer),
        }
    }
}

pub struct Vastatrix {
    heap:             broom::Heap<VTXObject>,
    class_handles:    HashMap<String, Handle<VTXObject>>,
    instance_handles: Vec<Handle<VTXObject>>,
    archive:          ZipArchive<File>,
}

impl Vastatrix {
    pub fn new(archive: ZipArchive<File>) -> Self {
        Self { heap: broom::Heap::default(), class_handles: HashMap::new(), instance_handles: vec![], archive }
    }

    pub fn run(&mut self) { self.load(); }

    fn load(&mut self) {
        let archive = &mut self.archive;
        let mut manifest_file = archive.by_name("META-INF/MANIFEST.MF").expect("Jar has no manifest!");
        let mut manifest = String::new();
        manifest_file.read_to_string(&mut manifest).expect("Could not get contents of manifest");
        drop(manifest_file);
        drop(archive);
        for line in manifest.lines() {
            if line.starts_with("Main-Class") {
                let archive = &mut self.archive;
                let split: Vec<&str> = line.split(' ').collect();
                let class = split.get(1).unwrap();
                let class_vec: Vec<&str> = class.split('.').collect();
                let class_path = class_vec.join("/");
                let handle = self.load_or_get_class_handle(class_path);
                let class = self.get_class(handle);
                class.frame("main".to_string(), &mut (vec![] as Vec<i32>)).exec(self);
            }
        }
    }

    pub fn load_or_get_class_handle(&mut self, classpath: String) -> Handle<VTXObject> {
        if self.class_handles.contains_key(&classpath) {
            return *self.class_handles.get(&classpath).unwrap();
        }
        let archive = &mut self.archive;
        println!("{}", classpath.clone());
        let mut class_file = archive.by_name(&(classpath.clone() + ".class")).expect("Could not find class file!");
        let mut class_buf: Vec<u8> = vec![];
        class_file.read_to_end(&mut class_buf).unwrap();
        let bytes = Bytes::from(class_buf);
        let class = Class::new(bytes);
        let handle = self.heap.insert_temp(VTXObject::Class(class));
        self.class_handles.insert(classpath, handle);
        if let VTXObject::Class(cls) = self.heap.get_mut(handle).unwrap() {
            cls.set_handle(handle);
        }
        handle
    }

    pub fn get_class(&mut self, handle: Handle<VTXObject>) -> &mut Class {
        if let VTXObject::Class(class) = self.heap.get_mut(handle).unwrap() {
            return class;
        }
        panic!("could not get class!");
    }

    pub fn prepare_instance(&mut self, class: &mut Class) -> i32 {
        let mut instance = Instance::new();
        for field in &class.fields {
            let name = &class.constant_pool[field.name_index as usize - 1];
            if let ConstantsPoolInfo::Utf8 { length, bytes, } = name {
                println!("field name: {}", bytes);
                let default_value: Option<i32> = None;
                if field.access_flags & 0x0008 != 0 {
                    for attribute in &field.attribute_info {
                        if let Attribute::ConstantValue { common, constantvalue_index, } = attribute {
                            let constantvalue = &class.constant_pool[*constantvalue_index as usize - 1];
                            match constantvalue {
                                _ => panic!("constantvalue_index did not index a valid constant value!"),
                            }
                        }
                    }
                }
                instance.fields.insert(bytes.to_string(), default_value);
            }
        }
        let handle = self.heap.insert_temp(VTXObject::Instance(instance));
        self.instance_handles.push(handle);
        return self.instance_handles.len() as i32 - 1;
    }

    pub fn get_instance(&mut self, index: usize) -> &mut Instance {
        let handle = self.instance_handles.get(index).unwrap();
        let thing = self.heap.get_mut(handle).unwrap();
        if let VTXObject::Instance(instance) = thing {
            return instance;
        }
        panic!("couldn't get instance!");
    }
}
