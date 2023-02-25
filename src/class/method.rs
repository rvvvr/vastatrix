use broom::trace::{Trace, Tracer};

use crate::vastatrix::VTXObject;

#[derive(Debug)]
pub struct Descriptor {
    pub types:   Vec<MethodType>,
    pub returns: Option<MethodType>,
}

#[derive(Debug, PartialEq)]
pub enum MethodType {
    Void,
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    ClassReference,
    Short,
    Boolean,
    ArrayReference(Box<MethodType>),
}

impl Trace<VTXObject> for MethodType {
    fn trace(&self, tracer: &mut Tracer<VTXObject>) {
        match self {
            Self::ArrayReference(elements) => elements.trace(tracer),
            _ => {},
        }
    }
}

impl Descriptor {
    pub fn new(desc: String) -> Self {
        let mut inarg = false;
        let mut types = vec![];
        let mut returns = None;
        for cha in desc.chars() {
            // will probably convert this to something to do with nom
            match cha {
                '(' => {
                    inarg = true;
                },
                ')' => {
                    inarg = false;
                },
                'L' => {
                    panic!("no classes.")
                },
                'I' =>
                    if inarg {
                        types.push(MethodType::Int);
                    } else {
                        if let None = returns {
                            returns = Some(MethodType::Int);
                        } else {
                            panic!("can only return one type!");
                        }
                    },
                'B' =>
                    if inarg {
                        types.push(MethodType::Byte);
                    } else {
                        if let None = returns {
                            returns = Some(MethodType::Byte);
                        } else {
                            panic!("can only return one type!");
                        }
                    },
                'C' =>
                    if inarg {
                        types.push(MethodType::Char);
                    } else {
                        if let None = returns {
                            returns = Some(MethodType::Char);
                        } else {
                            panic!("can only return one type!");
                        }
                    },
                'D' =>
                    if inarg {
                        types.push(MethodType::Double);
                    } else {
                        if let None = returns {
                            returns = Some(MethodType::Double);
                        } else {
                            panic!("can only return one type!");
                        }
                    },
                'F' =>
                    if inarg {
                        types.push(MethodType::Float);
                    } else {
                        if let None = returns {
                            returns = Some(MethodType::Float);
                        } else {
                            panic!("can only return one type!");
                        }
                    },
                'J' =>
                    if inarg {
                        types.push(MethodType::Long);
                    } else {
                        if let None = returns {
                            returns = Some(MethodType::Long);
                        } else {
                            panic!("can only return one type!");
                        }
                    },
                'S' =>
                    if inarg {
                        types.push(MethodType::Short);
                    } else {
                        if let None = returns {
                            returns = Some(MethodType::Short);
                        } else {
                            panic!("can only return one type!");
                        }
                    },
                'Z' =>
                    if inarg {
                        types.push(MethodType::Boolean);
                    } else {
                        if let None = returns {
                            returns = Some(MethodType::Boolean);
                        } else {
                            panic!("can only return one type!");
                        }
                    },
                '[' => {
                    panic!("no varargs yet");
                },
                _ => {},
            }
        }
        Self { types, returns }
    }
}

impl PartialEq<Descriptor> for Descriptor {
    fn eq(&self, other: &Descriptor) -> bool { self.types == other.types && self.returns == other.returns }
}
