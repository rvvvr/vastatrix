use std::collections::VecDeque;
use std::fmt::Debug;

use broom::Handle;
use bytes::{Buf, Bytes};
use dyn_clone::{clone_trait_object, DynClone};

use super::frame::{BytecodeFrame, Frame};
use super::method::Descriptor;
use crate::class::attribute::{Attribute, AttributeCommon};
use crate::class::method::{Argument, MethodType};
use crate::vastatrix::{VTXObject, Vastatrix};

pub trait Class: DynClone + Debug {
    fn set_handle(&mut self, handle: Handle<VTXObject>);
    fn get_handle(&self) -> Handle<VTXObject>;
    fn get_magic(&self) -> u32;
    fn get_minor(&self) -> u16;
    fn get_major(&self) -> u16;
    fn get_constant_count(&self) -> u16;
    fn get_constant_pool(&self) -> Vec<ConstantsPoolInfo>;
    fn get_access_flags(&self) -> u16;
    fn get_this_class(&self) -> u16;
    fn get_super_class(&self) -> u16;
    fn get_interface_count(&self) -> u16;
    fn get_interfaces(&self) -> Vec<u16>;
    fn get_field_count(&self) -> u16;
    fn get_fields(&self) -> Vec<FieldInfo>;
    fn get_method_count(&self) -> u16;
    fn get_methods(&self) -> Vec<MethodInfo>;
    fn get_attribute_count(&self) -> u16;
    fn get_attributes(&self) -> Vec<Attribute>;
    fn resolve(&self, constant_pool: Vec<ConstantsPoolInfo>, index: u16) -> Result<String, ()>;
    fn resolve_method(&self, method_info: ConstantsPoolInfo, superclass: bool, class_in: Option<Box<&dyn Class>>, running_in: &mut Vastatrix)
                      -> (Box<dyn Frame>, Descriptor);
    fn create_frame(&self, name: String, desc: String) -> Option<Box<dyn Frame>>;
}

clone_trait_object!(Class);

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
