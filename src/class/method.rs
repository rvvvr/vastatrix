use std::any::Any;
use std::fmt::Debug;
use std::ops::AddAssign;

use broom::trace::{Trace, Tracer};
use dyn_clone::{clone_trait_object, DynClone};

use crate::vastatrix::VTXObject;

#[derive(Debug)]
pub struct Descriptor {
    pub types:   Vec<MethodType>,
    pub returns: Option<MethodType>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MethodType {
    Void,
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    ClassReference { classpath: String, },
    Short,
    Boolean,
    ArrayReference,
}

impl Trace<VTXObject> for MethodType {
    fn trace(&self, _tracer: &mut Tracer<VTXObject>) {
        match self {
            _ => {},
        }
    }
}

impl Descriptor {
    pub fn new(desc: String) -> Self {
        let mut inarg = false;
        let mut inclass = false;
        let mut types = vec![];
        let mut returns = None;
        for cha in desc.chars() { 
                    if cha == ';' {
                        inclass = false;
                        continue;
                    }
                    if inclass {
                        if inarg {
                            let length = types.len();
                            if let MethodType::ClassReference { ref mut classpath } = types.get_mut(length - 1).unwrap() {
                                classpath.push(cha);
                            }
                        } else {
                            if let Some(MethodType::ClassReference { ref mut classpath }) = returns {
                                classpath.push(cha); 
                            }
                        }
                        continue;
                    }
            
            // will probably convert this to something to do with nom
            match cha {
                '(' => {
                    inarg = true;
                },
                ')' => {
                    inarg = false;
                },
                'L' =>
                    if inarg {
                        types.push(MethodType::ClassReference { classpath: "".to_string(), });
                        inclass = true;
                    } else {
                        returns = Some(MethodType::ClassReference { classpath: "".to_string(), });
                        inclass = true;
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
                'V' =>
                    if inarg {
                        panic!("no void in args!!");
                    } else {
                        if let None = returns {
                            returns = Some(MethodType::Void);
                        } else {
                            panic!("can only return one type!");
                        }
                    },
                '[' =>
                    if inarg {
                        types.push(MethodType::ArrayReference)
                    } else {
                        if let None = returns {
                            returns = Some(MethodType::ArrayReference);
                            break;
                        } else {
                            panic!("can only return one type!");
                        }
                    },
                _ => {
                },
            }
         
           

        }
        Self { types, returns }
    }
}

impl PartialEq<Descriptor> for Descriptor {
    fn eq(&self, other: &Descriptor) -> bool { self.types == other.types && self.returns == other.returns }
}

pub trait Num: DynClone + Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_usize(&self) -> usize;
}

clone_trait_object!(Num);

impl Num for u32 {
    fn as_any(&self) -> &dyn Any { self }

    fn as_usize(&self) -> usize { *self as usize }
}
impl Num for i32 {
    fn as_any(&self) -> &dyn Any { self }

    fn as_usize(&self) -> usize { *self as usize }
}
impl Num for f32 {
    fn as_any(&self) -> &dyn Any { self }

    fn as_usize(&self) -> usize { *self as usize }
}

#[derive(Debug, Clone)]
pub struct Argument {
    value: Box<dyn Num>,
    is:    MethodType,
}

impl Argument {
    pub fn new(value: impl Num + 'static, is: MethodType) -> Self { Self { value: Box::new(value), is } }

    pub fn value_ref(&mut self) -> u32 {
        if let MethodType::ClassReference { .. } = self.is {
            return *self.value.as_any().downcast_ref::<u32>().unwrap();
        } else if let MethodType::ArrayReference = self.is {
            return *self.value.as_any().downcast_ref::<u32>().unwrap();
        }
        panic!("value was not a ref! was a {:?}", self.is);
    }

    pub fn is(&self, other: MethodType) -> bool {
        self.is == other
    }

    pub fn void(&self) -> bool { return self.is == MethodType::Void }

    pub fn wrapping_iadd(self, rhs: Self) -> Self {
        if self.is != MethodType::Int || self.is != rhs.is {
            panic!("incompatible iadd types!");
        }
        let a = self.value.as_any().downcast_ref::<i32>().unwrap();
        let b = rhs.value.as_any().downcast_ref::<i32>().unwrap();
        return Argument::new(a.wrapping_add(*b), MethodType::Int);
    }

    pub fn wrapping_isub(self, rhs: Self) -> Self {
        if self.is != MethodType::Int || self.is != rhs.is {
            panic!("incompatible iadd types!");
        }
        let a = self.value.as_any().downcast_ref::<i32>().unwrap();
        let b = rhs.value.as_any().downcast_ref::<i32>().unwrap();
        return Argument::new(a.wrapping_sub(*b), MethodType::Int);
    }

    pub fn wrapping_imul(self, rhs: Self) -> Self {
        if self.is != MethodType::Int || self.is != rhs.is {
            panic!("incompatible iadd types!");
        }
        let a = self.value.as_any().downcast_ref::<i32>().unwrap();
        let b = rhs.value.as_any().downcast_ref::<i32>().unwrap();
        return Argument::new(a.wrapping_mul(*b), MethodType::Int);
    }

    pub fn wrapping_idiv(self, rhs: Self) -> Self {
        if self.is != MethodType::Int || self.is != rhs.is {
            panic!("incompatible iadd types!");
        }
        let a = self.value.as_any().downcast_ref::<i32>().unwrap();
        let b = rhs.value.as_any().downcast_ref::<i32>().unwrap();
        return Argument::new(a.wrapping_div(*b), MethodType::Int);
    }
}

impl PartialEq<Argument> for Argument {
    fn eq(&self, other: &Argument) -> bool {
        if self.is != other.is {
            return false;
        }
        match self.is {
            MethodType::Int => {
                return self.value.as_any().downcast_ref::<i32>().unwrap() == other.value.as_any().downcast_ref::<i32>().unwrap();
            },
            MethodType::Float => {
                return self.value.as_any().downcast_ref::<f32>().unwrap() == other.value.as_any().downcast_ref::<f32>().unwrap();
            },
            _ => return false,
        }
    }
}

impl PartialOrd<Argument> for Argument {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.is != other.is {
            panic!("Comparing incompatible types!");
        }
        match self.is {
            MethodType::Int => {
                let s = self.value.as_usize();
                let o = other.value.as_usize();
                return s.partial_cmp(&o);
            },
            MethodType::Float => {
                let s = self.value.as_any().downcast_ref::<f32>().unwrap();
                let o = other.value.as_any().downcast_ref::<f32>().unwrap();
                return s.partial_cmp(o);
            },
            _ => panic!("cannot compare these types!"),
        }
    }

    fn lt(&self, other: &Self) -> bool { matches!(self.partial_cmp(other), Some(core::cmp::Ordering::Less)) }

    fn le(&self, other: &Self) -> bool { matches!(self.partial_cmp(other), Some(core::cmp::Ordering::Less | core::cmp::Ordering::Equal)) }

    fn gt(&self, other: &Self) -> bool { matches!(self.partial_cmp(other), Some(core::cmp::Ordering::Greater)) }

    fn ge(&self, other: &Self) -> bool { matches!(self.partial_cmp(other), Some(core::cmp::Ordering::Greater | core::cmp::Ordering::Equal)) }
}

impl AddAssign<i32> for Argument {
    fn add_assign(&mut self, rhs: i32) {
        if self.is != MethodType::Int {
            panic!("Adding int to {:?}", self.is)
        }
        let total = self.value.as_any().downcast_ref::<i32>().unwrap() + rhs;
        self.value = Box::new(total as i32);
    }
}

impl Into<i32> for Argument {
    fn into(self) -> i32 { return *self.value.as_any().downcast_ref::<i32>().unwrap(); }
}

impl Into<usize> for Argument {
    fn into(self) -> usize { return self.value.as_usize(); }
}

impl Trace<VTXObject> for Argument {
    fn trace(&self, tracer: &mut Tracer<VTXObject>) { return; }
}
