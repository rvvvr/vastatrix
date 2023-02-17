use bytes::{Bytes, Buf};

use super::{Class, ConstantsPoolInfo};


#[derive(Debug, Clone)]
pub enum Attribute {
    ConstantValue { common: AttributeCommon, constantvalue_index: u16 },
    Code { common: AttributeCommon, max_stack: u16, max_locals: u16, code_length: u32, code: Vec<u8>, exception_table_length: u16, exception_table: Vec<ExceptionTableEntry>, attribute_count: u16, attribute_info: Vec<Attribute> },
    StackMapTable { common: AttributeCommon, number_of_entries: u16, entries: Vec<StackMapFrame> },
    Exceptions { common: AttributeCommon, number_of_exceptions: u16, exception_index_table: Vec<u16> },
    InnerClasses { common: AttributeCommon, number_of_classes: u16, classes: Vec<InnerClassesEntry> },
    EnclosingMethod { common: AttributeCommon, class_index: u16, method_index: u16 },
    Synthetic { common: AttributeCommon },
    Signature { common: AttributeCommon, signature_index: u16 }, // 4.7.9.1 will probably be important for this later.
    SourceFile { common: AttributeCommon, sourcefile_index: u16 },
    SourceDebugExtension { common: AttributeCommon, debug_extension: Vec<u8> },
    LineNumberTable { common: AttributeCommon, line_number_table_length: u16, line_number_table: Vec<LineNumberTableEntry> },
    LocalVariableTable { common: AttributeCommon, local_variable_table_length: u16, local_variable_table: Vec<LocalVariableTableEntry> },
    LocalVariableTypeTable { common: AttributeCommon, local_variable_type_table_length: u16, local_variable_type_table: Vec<LocalVariableTypeTableEntry> },
    Deprecated { common: AttributeCommon },
    RuntimeVisibleAnnotations { common: AttributeCommon, num_annotations: u16, annotations: Vec<Annotation> },
    RuntimeInvisibleAnnotations { common: AttributeCommon, num_annotations: u16, annotations: Vec<Annotation> },
    RuntimeVisibleParameterAnnotations { common: AttributeCommon, num_parameters: u8, parameter_annotations: Vec<ParameterAnnotation> },
    RuntimeInvisibleParameterAnnotations { common: AttributeCommon, num_parameters: u8, parameter_annotations: Vec<ParameterAnnotation> },
    RuntimeVisibleTypeAnnotations { common: AttributeCommon, num_annotations: u16, annotations: Vec<TypeAnnotation> },
    RuntimeInvisibleTypeAnnotations { common: AttributeCommon, num_annotations: u16, annotations: Vec<TypeAnnotation> },
    AnnotationDefault { common: AttributeCommon, /* element_value: ElementValue */ },
    BootstrapMethods { common: AttributeCommon, num_bootstrap_methods: u16, bootstrap_methods: Vec<BootstrapMethod> },
    MethodParameters { common: AttributeCommon, parameters_count: u8, parameters: Vec<Parameter> },
    Module { common: AttributeCommon, module_name_index: u16, module_flags: u16, module_version_index: u16, requires_count: u16, requires: Vec<ModuleRequires>, exports_count: u16, exports: Vec<ModuleExports>, opens_count: u16, opens: Vec<ModuleOpens>, uses_count: u16, uses_index: Vec<u16>, provides_count: u16, provides: Vec<ModuleProvides> },
    ModulePackages { common: AttributeCommon, package_count: u16, package_index: Vec<u16> },
    ModuleMainClass { common: AttributeCommon, main_class_index: u16 },
    NestHost { common: AttributeCommon, host_class_index: u16 },
    NestMembers { common: AttributeCommon, number_of_classes: u16, classes: Vec<u16> },
    Record { common: AttributeCommon, components_count: u16, components: Vec<RecordComponentInfo> },
    PermittedSubclasses { common: AttributeCommon, number_of_classes: u16, classes: Vec<u16> }
}

#[derive(Debug, Clone)]
pub struct ExceptionTableEntry {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(Debug, Clone)]
pub struct InnerClassesEntry {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: u16,
}

#[derive(Debug, Clone)]
pub struct LineNumberTableEntry {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Debug, Clone)]
pub struct LocalVariableTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

#[derive(Debug, Clone)]
pub struct LocalVariableTypeTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub signature_index: u16,
    pub index: u16,
}

#[derive(Debug, Clone)]
pub struct Annotation {
    pub type_index: u16,
    pub num_element_value_pairs: u16,
    // fuck this
}

#[derive(Debug, Clone)]
pub struct ParameterAnnotation {
    pub num_annotations: u16,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone)]
pub struct TypeAnnotation {
    //todo
}

#[derive(Debug, Clone)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: u16,
    pub num_bootstrap_arguments: u16,
    pub bootstrap_arguments: Vec<u16>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name_index: u16,
    pub access_flags: u16,
}

#[derive(Debug, Clone)]
pub struct ModuleRequires {
    pub requires_index: u16,
    pub requires_flags: u16,
    pub requires_version_index: u16,
}

#[derive(Debug, Clone)]
pub struct ModuleExports {
    pub exports_index: u16,
    pub exports_flags: u16,
    pub exports_to_count: u16,
    pub exports_to_index: Vec<u16>
}

#[derive(Debug, Clone)]
pub struct ModuleOpens {
    pub opens_index: u16,
    pub opens_flags: u16,
    pub opens_to_count: u16,
    pub opens_to_index: Vec<u16>
}

#[derive(Debug, Clone)]
pub struct ModuleProvides {
    pub provides_index: u16,
    pub provides_with_count: u16,
    pub provides_with_index: Vec<u16>,
}

#[derive(Debug, Clone)]
pub struct RecordComponentInfo {
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone)]
pub struct AttributeCommon {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
}

#[derive(Debug, Clone)]
pub enum AttributeLocation {
    ClassFile,
    FieldInfo,
    MethodInfo,
    RecordComponentInfo,
    Code
}

#[derive(Debug, Clone)]
pub enum StackMapFrame {
    SameFrame,
    SameLocals1StackItemFrame,
    SameLocals1StackItemFrameExtended,
    ChopFrame,
    SameFrameExtended,
    AppendFrame,
    FullFrame
}

#[derive(Debug, Clone)]
pub enum VerificationTypeInfo {
    //TODO: this doesn't matter yet.
}

#[derive(Debug, Clone)]
pub enum VerificationType {
    //TODO: this also doesnt matter yet
    ItemTop,
    ItemInteger,
    ItemFloat,
    ItemDouble,
    ItemLong,
    ItemNull,
    ItemUninitializedThis,
    ItemObject,
    ItemUninitialized,
}

impl Attribute {
    pub fn parse(mut bytes: Bytes, constants: Vec<ConstantsPoolInfo>, common: AttributeCommon, location: AttributeLocation) -> Attribute {
        match (Class::resolve(constants.clone(), common.attribute_name_index).unwrap().as_str()) {
            "ConstantValue" => {
                match location {
                    AttributeLocation::FieldInfo => {},
                    _ => panic!("ConstantValue attribute is not allowed in {:?}", location),
                } //this could be a macro maybe?
                let constantvalue_index = bytes.get_u16();
                Attribute::ConstantValue { common, constantvalue_index }
            },
            "Code" => {
                match location {
                    AttributeLocation::MethodInfo => {},
                    _ => panic!("Code attribute is not allowed in {:?}", location),
                } //this could be a macro maybe?
                let max_stack = bytes.get_u16();
                let max_locals = bytes.get_u16();
                let code_length = bytes.get_u32();
                let code = bytes.clone().take(code_length as usize).chunk().to_vec();
                bytes.advance(code_length as usize);
                let exception_table_length = bytes.get_u16();
                let mut exception_table = vec![];
                for _ in 0..exception_table_length {
                    let start_pc = bytes.get_u16();
                    let end_pc = bytes.get_u16();
                    let handler_pc = bytes.get_u16();
                    let catch_type = bytes.get_u16();
                    exception_table.push(ExceptionTableEntry{ start_pc, end_pc, handler_pc, catch_type });
                }
                let attribute_count = bytes.get_u16();
                let mut attribute_info = vec![];
                for _ in 0..attribute_count {
                    let attribute_name_index = bytes.get_u16();
                    let attribute_length = bytes.get_u32();
                    attribute_info.push(Attribute::parse(bytes.copy_to_bytes(attribute_length as usize), constants.clone(), AttributeCommon { attribute_name_index, attribute_length}, AttributeLocation::Code))
                }
                Attribute::Code { common, max_stack, max_locals, code_length, code, exception_table_length, exception_table, attribute_count, attribute_info }
            },
            "StackMapTable" => {
                match location {
                    AttributeLocation::Code => {},
                    _ => panic!("StackMapTable attribute is not allowed in {:?}", location),
                } //this could be a macro maybe?
                todo!("not all the required data types are implemented properly for this");
            },
            "Exceptions" => {
                match location {
                    AttributeLocation::MethodInfo => {},
                    _ => panic!("Exceptions attribute is not allowed in {:?}", location),
                } //this could be a macro maybe?
                let number_of_exceptions = bytes.get_u16();
                let mut exception_index_table = vec![];
                for _ in 0..number_of_exceptions {
                    exception_index_table.push(bytes.get_u16())
                }
                Attribute::Exceptions { common, number_of_exceptions, exception_index_table }

            }
            "SourceFile" => {
                match location {
                    AttributeLocation::ClassFile => {},
                    _ => panic!("SourceFile attribute is not allowed in {:?}", location),
                } //this could be a macro maybe?
                let sourcefile_index = bytes.get_u16();
                Attribute::SourceFile { common, sourcefile_index }
            },
            "InnerClasses" => {
                match location {
                    AttributeLocation::ClassFile => {},
                    _ => panic!("InnerClasses attribute is not allowed in {:?}", location),
                } //this could be a macro maybe?
                let number_of_classes = bytes.get_u16();
                let mut classes = vec![];
                for _ in 0..number_of_classes {
                    let inner_class_info_index = bytes.get_u16();
                    let outer_class_info_index = bytes.get_u16();
                    let inner_name_index = bytes.get_u16();
                    let inner_class_access_flags = bytes.get_u16();
                    classes.push(InnerClassesEntry {
                        inner_class_access_flags, inner_class_info_index, outer_class_info_index, inner_name_index
                    })
                }
                Attribute::InnerClasses { common, number_of_classes, classes }
            },
            "Synthetic" => {
                match location {
                    AttributeLocation::ClassFile | AttributeLocation::FieldInfo | AttributeLocation::MethodInfo => {},
                    _ => panic!("Code attribute is not allowed in {:?}", location),
                } //this could be a macro maybe?
                Attribute::Synthetic { common }
            },
            "Signature" => {
                match location {
                    AttributeLocation::Code => {panic!("Code attribute is not allowed in {:?}", location)},
                    _ => {},
                } //this could be a macro maybe?
                let signature_index = bytes.get_u16();
                Attribute::Signature { common, signature_index }
            }
            "EnclosingMethod" => {
                match location {
                    AttributeLocation::ClassFile => {},
                    _ => panic!("InnerClasses attribute is not allowed in {:?}", location),
                } //this could be a macro maybe?
                let class_index = bytes.get_u16();
                let method_index = bytes.get_u16();
                Attribute::EnclosingMethod { common, class_index, method_index }
            },
            "SourceDebugExtension" => {
                match location {
                    AttributeLocation::ClassFile => {},
                    _ => panic!("InnerClasses attribute is not allowed in {:?}", location),
                } //this could be a macro maybe?
                let debug_extension = bytes.clone().take(common.attribute_length as usize).chunk().to_vec();
                bytes.advance(common.attribute_length as usize);
                Attribute::SourceDebugExtension { common, debug_extension }
            },
            "LineNumberTable" => {
                match location {
                    AttributeLocation::Code => {},
                    _ => panic!("InnerClasses attribute is not allowed in {:?}", location),
                } //this could be a macro maybe?
                let line_number_table_length = bytes.get_u16();
                let mut line_number_table = vec![];
                for _ in 0..line_number_table_length {
                    let start_pc = bytes.get_u16();
                    let line_number = bytes.get_u16();
                    line_number_table.push(LineNumberTableEntry { start_pc, line_number });
                }
                Attribute::LineNumberTable { common, line_number_table_length, line_number_table }
            },
            "LocalVariableTable" => {
                match location {
                    AttributeLocation::Code => {},
                    _ => panic!("InnerClasses attribute is not allowed in {:?}", location),
                } //this could be a macro maybe?
                let local_variable_table_length = bytes.get_u16();
                let mut local_variable_table = vec![];
                for _ in 0..local_variable_table_length {
                    let start_pc = bytes.get_u16();
                    let length = bytes.get_u16();
                    let name_index = bytes.get_u16();
                    let descriptor_index = bytes.get_u16();
                    let index = bytes.get_u16();
                    local_variable_table.push(LocalVariableTableEntry { start_pc, length, name_index, descriptor_index, index });
                }
                Attribute::LocalVariableTable { common, local_variable_table_length, local_variable_table }
            },
            "LocalVariableTypeTable" => {
                match location {
                    AttributeLocation::Code => {},
                    _ => panic!("InnerClasses attribute is not allowed in {:?}", location),
                } //this could be a macro maybe?
                let local_variable_type_table_length = bytes.get_u16();
                let mut local_variable_type_table = vec![];
                for _ in 0..local_variable_type_table_length {
                    let start_pc = bytes.get_u16();
                    let length = bytes.get_u16();
                    let name_index = bytes.get_u16();
                    let signature_index = bytes.get_u16();
                    let index = bytes.get_u16();
                    local_variable_type_table.push(LocalVariableTypeTableEntry { start_pc, length, name_index, signature_index, index });
                }
                Attribute::LocalVariableTypeTable { common, local_variable_type_table_length, local_variable_type_table }
            },
            "Deprecated" => {
                Attribute::Deprecated { common }
            },
            "RuntimeVisibleAnnotations" => {
                todo!("the data types for annotations aren't yet implemented properly");
            },
            "RuntimeVisibleParameterAnnotations" => {
                todo!("the data types for annotations aren't yet implemented properly");
            },
            "RuntimeInvisibleParameterAnnotations" => {
                todo!("the data types for annotations aren't yet implemented properly");
            },
            "RuntimeVisibleTypeAnnotations" => {
                todo!("the data types for annotations aren't yet implemented properly");
            },
            "RuntimeInvisibleTypeAnnotations" => {
                todo!("the data types for annotations aren't yet implemented properly");
            },
            "AnnotationDefault" => {
                todo!("the data types for annotations aren't yet implemented properly");
            },
            "BootstrapMethods" => {
                let num_bootstrap_methods = bytes.get_u16();
                let mut bootstrap_methods = vec![];
                for _ in 0..num_bootstrap_methods {
                    let bootstrap_method_ref = bytes.get_u16();
                    let num_bootstrap_arguments = bytes.get_u16();
                    let mut bootstrap_arguments = vec![];
                    for _ in 0..num_bootstrap_arguments {
                        bootstrap_arguments.push(bytes.get_u16());
                    }
                    bootstrap_methods.push(BootstrapMethod { bootstrap_method_ref, num_bootstrap_arguments, bootstrap_arguments });
                }
                Attribute::BootstrapMethods { common, num_bootstrap_methods, bootstrap_methods }
            }
            a => {
                panic!("unknown attribute type: {}", a);
            }
        }
    }
}