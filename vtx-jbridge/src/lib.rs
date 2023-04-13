extern crate proc_macro;
use std::collections::HashMap;

use proc_macro::{Span, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::__private::ToTokens;
use syn::ext::IdentExt;
use syn::parse::Parse;
use syn::{braced, parse_macro_input, Block, Ident, LitStr, Token};
use vastatrix::class::method::{Descriptor, MethodType};

#[derive(Debug)]
pub(crate) struct ClassData {
    pub classpath: String,
    pub superclasspath: String,
    pub classname: String,
    pub methods:   ClassMethods,
    pub fields:    ClassFields,
}

#[derive(Debug)]
pub(crate) struct ClassMethods {
    pub methods: HashMap<String, Vec<MethodData>>,
}

#[derive(Debug)]
pub(crate) struct MethodData {
    pub javadesc: String,
    pub logic:    proc_macro2::TokenStream,
    pub instance: bool,
}

#[derive(Debug)]
pub(crate) struct ClassFields {
    pub fields: HashMap<String, Vec<FieldData>>,
}

#[derive(Debug)]
pub(crate) struct FieldData {
    pub javadesc: String,
    pub instance: bool,
}

pub(crate) mod keywords {
    syn::custom_keyword!(package);
    syn::custom_keyword!(public);
    syn::custom_keyword!(class);
    syn::custom_keyword!(field);
    syn::custom_keyword!(instance);
    syn::custom_keyword!(superclass);
}

impl Parse for ClassData {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut classpath = String::new();
        let mut methods = ClassMethods { methods: HashMap::new(), };
        let mut fields = ClassFields { fields: HashMap::new(), };
        let mut classname = String::new();
        let mut superclasspath = String::new();
        println!("{}", input);
        if input.peek(keywords::package) {
            input.parse::<keywords::package>()?; // parsing module

            loop {
                if input.peek(Token![;]) {
                    input.parse::<Token![;]>()?;
                    break;
                }
                if input.peek(Token![.]) {
                    input.parse::<Token![.]>()?;
                    classpath.push('/');
                    continue;
                }
                if input.peek(Ident::peek_any) {
                    let module = input.call(Ident::parse_any)?;
                    classpath.push_str(module.to_string().as_str());
                }
            }

            // parsing class signature - as of now only public class is supported

            if input.peek(keywords::public) {
                input.parse::<keywords::public>()?;

                if input.peek(keywords::class) {
                    input.parse::<keywords::class>()?;

                    if input.peek(Ident::peek_any) {
                        classname = input.call(Ident::parse_any)?.to_string();
                        classpath.push('/');
                        classpath.push_str(classname.as_str());
                    }
                }
            }

            let class_content;
            braced!(class_content in input);

           
            if class_content.peek(keywords::superclass) {
                class_content.parse::<keywords::superclass>()?;

                loop {
                    if class_content.peek(Token![;]) {
                        class_content.parse::<Token![;]>()?;
                        break;
                    }
                    if class_content.peek(Token![.]) {
                        class_content.parse::<Token![.]>()?;
                        superclasspath.push('/');
                        continue;
                    }
                    if class_content.peek(Ident::peek_any) {
                        let module = class_content.call(Ident::parse_any)?;
                        superclasspath.push_str(module.to_string().as_str());
                    }
                }
            }

            // fields go here
            loop {
                println!("Meep");
                if class_content.peek(keywords::field) {
                    println!("Meep2");
                    class_content.parse::<keywords::field>()?;
                    if class_content.peek(keywords::instance) {
                        println!("Meep3");
                        class_content.parse::<keywords::instance>()?;
                        if class_content.peek(LitStr) {
                            println!("Meep4");
                            let namelit = class_content.parse::<LitStr>()?;
                            class_content.parse::<Token![,]>()?;
                            println!("Meep5");
                            let desclit = class_content.parse::<LitStr>()?;
                            println!("wtf?? {}", input);
                            class_content.parse::<Token![;]>()?;
                            println!("Meep6");
                            if fields.fields.contains_key(&namelit.value()) {
                                fields.fields
                                      .get_mut(&namelit.value())
                                      .expect("could not get field!")
                                      .push(FieldData { javadesc: desclit.value(), instance: false, });
                            } else {
                                fields.fields.insert(namelit.value(), vec![FieldData { javadesc: desclit.value(), instance: false, }]);
                            }
                        }
                    }
                } else {
                    break;
                }
            }

            loop {
                // parsing methods -- only public methods are supported atm (also unsure if this
                // matters from a jvm standpoint or only the compiler standpoint atm)
                if class_content.peek(Token![static]) {
                    class_content.parse::<Token![static]>()?;
                    if class_content.peek(LitStr) {
                        let namelit = class_content.parse::<LitStr>()?;
                        class_content.parse::<Token![,]>()?;
                        let desclit = class_content.parse::<LitStr>()?;
                        let tokens = class_content.parse::<Block>()?;
                        if methods.methods.contains_key(&namelit.value()) {
                            methods.methods.get_mut(&namelit.value()).expect("could not get method!").push(MethodData { javadesc: desclit.value(),
                                                                                                                        logic:
                                                                                                                            tokens.to_token_stream()
                                                                                                                                  .into(),
                                                                                                                        instance: false, });
                        } else {
                            methods.methods.insert(namelit.value(), vec![MethodData { javadesc: desclit.value(),
                                                                                      logic:    tokens.to_token_stream().into(),
                                                                                      instance: false, }]);
                        }
                    }
                } else {
                    break;
                }
            }
        } else {
            return Err(input.error("Class macro must start with package statement!"));
        }
        println!("{}", classpath);
        Ok(ClassData { classpath, superclasspath, classname, methods, fields })
    }
}

#[proc_macro]
pub fn class(input: TokenStream) -> TokenStream {
    let thing = parse_macro_input!(input as ClassData);
    let classpath = thing.classpath;
    let classname = Ident::new(thing.classname.as_str(), Span::call_site().into());
    // there's gotta be a better way to do this...

    // let mut out = "".to_string();
    // out.push_str("use vastatrix::class::definition::Class;\n");
    // out.push_str("use vastatrix::class::frame::Frame;\n");
    // out.push_str(format!("pub struct {} {{\n", thing.classname.as_str()).as_str());
    // out.push_str("  pub handle: Option<Handle<VTXObject>>");
    // out.push_str("}\n");
    // out.push_str(format!("impl Class for {} {{\n", thing.classname.as_str()).as_str());
    // println!("SHMEEP {}", out);

    // there in fact is a better way to do this!
    let mut constants_pool = quote! {ConstantsPoolInfo::Dummy};
    let mut constants_pool_count = 1u16;
    let this_class_index: u16;
    // generate this_class constants_pool members
    {
        this_class_index = constants_pool_count + 1;
        let name_index = this_class_index + 1;
        let classpath_length = classpath.len() as u16;
        constants_pool.append_all(vec![quote! {
                                           , ConstantsPoolInfo::Class {
                                               name_index: #name_index,
                                           }
                                       },
                                       quote! {
                                           , ConstantsPoolInfo::Utf8 {
                                               length: #classpath_length,
                                               bytes: #classpath.to_string(),
                                           }
                                       }]);
        constants_pool_count += 2;
    }
    let super_class_index: u16;
    // generate super_class constants_pool members
    {
        if classpath == "java/lang/Object" {
            super_class_index = 0;
        } else if thing.superclasspath != String::new() {
            super_class_index = constants_pool_count + 1;
            let name_index = super_class_index + 1;
            let superclass = thing.superclasspath;
            let superclass_length = superclass.len() as u16;
            constants_pool.append_all(vec![quote! {
                                                , ConstantsPoolInfo::Class {
                                                    name_index: #name_index,
                                                }
                                            },
                                            quote! {
                                                , ConstantsPoolInfo::Utf8 {
                                                    length: #superclass_length,
                                                    bytes: #superclass.to_string(),
                                                }
                                            }]);
            constants_pool_count += 2;
        } else {
            // TODO: Implement superclasses
            super_class_index = constants_pool_count + 1;
            let name_index = super_class_index + 1;
            let superclass = "java/lang/Object";
            let superclass_length = superclass.len() as u16;
            constants_pool.append_all(vec![quote! {
                                               , ConstantsPoolInfo::Class {
                                                   name_index: #name_index,
                                               }
                                           },
                                           quote! {
                                               , ConstantsPoolInfo::Utf8 {
                                                   length: #superclass_length,
                                                   bytes: #superclass.to_string(),
                                               }
                                           }]);
            constants_pool_count += 2;
        }
    }
    // generate field_info structures and constants_pool members
    let mut field_count = 0u16;
    let mut fields = quote! {};
    println!("FIELDS: {:?}", thing.fields.fields);
    {
        for fieldname in thing.fields.fields.keys() {
            let name_index = constants_pool_count;
            let name_length = fieldname.len() as u16;
            constants_pool.append_all(vec![quote! {
                              , ConstantsPoolInfo::Utf8 {
                                  length: #name_length,
                                  bytes: #fieldname.to_string(),
                              }
                          }]);
            constants_pool_count += 1;
            for field in thing.fields.fields.get(fieldname).expect("could not get field for generating!") {
                let desc_index = constants_pool_count;
                let desc = &field.javadesc;
                let desc_length = desc.len() as u16;
                constants_pool.append_all(vec![quote! {
                                  , ConstantsPoolInfo::Utf8 {
                                      length: #desc_length,
                                      bytes: #desc.to_string(),
                                  }
                              }]);
                fields.append_all(vec![quote! {
                          FieldInfo {
                              access_flags:     0,
                              name_index:       #name_index,
                              descriptor_index: #desc_index,
                              attribute_count:  0,
                              attribute_info:   vec![],
                          },

                      }]);
                constants_pool_count += 1;
                field_count += 1;
            }
        }
    }

    // generate method_info structures and constants_pool members
    let mut method_count = 0u16;
    let mut methods = quote! {};
    let mut big_match_arms = quote! {};
    let mut method_frames = quote! {};
    {
        for methodname in thing.methods.methods.keys() {
            let mut rust_friendly_methodname = rustify(methodname.to_string());
            let name_index = constants_pool_count;
            let name_length = methodname.len() as u16;
            constants_pool.append_all(vec![quote! {
                              , ConstantsPoolInfo::Utf8 {
                                  length: #name_length,
                                  bytes: #methodname.to_string(),
                              }
                          }]);
            constants_pool_count += 1;
            let mut little_match_arms = quote! {};
            for method in thing.methods.methods.get(methodname).expect("could not get method for generating!") {
                let logic = method.logic.clone();
                let desc_index = constants_pool_count;
                let desc = &method.javadesc;
                let descriptor = Descriptor::new(desc.to_string());
                let mut rfname = rust_friendly_methodname.clone();
                for t in descriptor.types {
                    if t == MethodType::ArrayReference {
                        rfname.push_str("Array");
                        continue;
                    }
                    if let MethodType::ClassReference { ref classpath, } = t {
                        rfname.push_str(classpath.replace("/", "").as_str());
                        continue;
                    }
                    rfname.push_str(format!("{:?}", t).as_str());
                }
                let returns = descriptor.returns.expect("could not get descriptor returns!");
                if let MethodType::ClassReference { ref classpath, } = returns {
                    rfname.push_str(format!("Ret{}", classpath.replace("/", "")).as_str());
                } else {
                    rfname.push_str(format!("Ret{:?}", returns).as_str());
                }
                rfname.push_str("Frame");
                let rf_ident = Ident::new(rfname.as_str(), Span::call_site().into());

                let desc_length = desc.len() as u16;
                constants_pool.append_all(vec![quote! {
                                  , ConstantsPoolInfo::Utf8 {
                                      length: #desc_length,
                                      bytes: #desc.to_string(),
                                  }
                              }]);
                methods.append_all(vec![quote! {
                          MethodInfo {
                              access_flags: 0u16,
                              name_index: #name_index,
                              descriptor_index: #desc_index,
                              attribute_count: 0u16,
                              attribute_info: vec![],
                          },
                       }]);

                constants_pool_count += 1;
                method_count += 1;
                little_match_arms.append_all(vec![quote! {
                                     #desc => {
                                         return Some(Box::new(#rf_ident::new()));
                                     },
                                 }]);
                method_frames.append_all(vec![quote! {
                                 #[derive(Debug)]
                                 pub struct #rf_ident {

                                 }

                                 impl #rf_ident {
                                     pub fn new() -> Self {
                                         return Self {};
                                     }
                                 }

                                 impl Frame for #rf_ident {
                                     fn exec(&mut self, args: Vec<Argument>, running_in: &mut Vastatrix) -> Argument {
                                         #logic
                                     }
                                 }
                             }]);
                println!("d");
            }
            big_match_arms.append_all(vec![quote! {
                              #methodname => {
                                  match desc.as_str() {
                                      #little_match_arms
                                      _ => {
                                          panic!("method {} was found, but not with descriptor {}!", name, desc);
                                      }
                                  }
                              }
                          }]);
            println!("e");
        }
    }
    let method_match = quote! {
        match name.as_str() {
            #big_match_arms
            _ => {
                panic!("method name {} not found!", name);
            }
        }
    };
    println!("f");
    println!("{}", method_match);
    constants_pool_count -= 1;
    let out = quote! {
        use vastatrix::{class::{definition::{Class, FieldInfo, MethodInfo}, frame::Frame, method::{Descriptor, Argument, MethodType}, ConstantsPoolInfo, attribute::Attribute}, vastatrix::Vastatrix, vastatrix::VTXObject};
        use broom::Handle;
        #[derive(Debug, Clone)]
        pub struct #classname {
            pub handle: Option<Handle<VTXObject>>,
        }
        impl Class for #classname {
            fn set_handle(&mut self, handle: Handle<VTXObject>) {
                self.handle = Some(handle);
            }
            fn get_handle(&self) -> Handle<VTXObject> {
                self.handle.expect("handle not defined!")
            }
            fn get_magic(&self) -> u32 {
                0xCAFEBABEu32
            }
            fn get_major(&self) -> u16 {
                61u16
            }
            fn get_minor(&self) -> u16 {
                0u16
            }
            fn get_constant_count(&self) -> u16 {
                #constants_pool_count
            }
            fn get_constant_pool(&self) -> Vec<ConstantsPoolInfo> {
                vec![#constants_pool]
            }
            fn get_access_flags(&self) -> u16 {
                0u16 //not quite sure how access flags work yet, and I'm not parsing them anyways.
                     //TODO: Come back to this.
            }
            fn get_this_class(&self) -> u16 {
                #this_class_index
            }
            fn get_super_class(&self) -> u16 {
                #super_class_index
            }
            fn get_interface_count(&self) -> u16 {
                0u16
            }
            fn get_interfaces(&self) -> Vec<u16> {
                vec![]
            }
            fn get_field_count(&self) -> u16 {
                #field_count
            }
            fn get_fields(&self) -> Vec<FieldInfo> {
                vec![#fields]
            }
            fn get_method_count(&self) -> u16 {
                #method_count
            }
            fn get_methods(&self) -> Vec<MethodInfo> {
                vec![#methods]
            }
            fn get_attribute_count(&self) -> u16 {
                0u16
            }
            fn get_attributes(&self) -> Vec<Attribute> {
                vec![]
            }
            fn resolve(&self, constant_pool: Vec<ConstantsPoolInfo>, index: u16) -> Result<String, ()> {
                if let ConstantsPoolInfo::Utf8 { bytes, .. } = &constant_pool[index as usize] { Ok(bytes.to_string()) } else { Err(()) }
            }
            fn resolve_method(&self, method_info: ConstantsPoolInfo, superclass: bool, class_in: Option<Box<&dyn Class>>, running_in: &mut Vastatrix)
                      -> (Box<dyn Frame>, Descriptor) {
                panic!("I dont think this should ever be called?? This may change, however.");
            }
            fn create_frame(&self, name: String, desc: String) -> Option<Box<dyn Frame>> {
                #method_match
            }
        }
        #method_frames
    };

    println!("{}", out.to_string());

    out.into()
}

fn rustify(s: String) -> String {
    if s == "<init>" {
        return "StaticInit".to_string();
    } else {
        return s;
    }
}
