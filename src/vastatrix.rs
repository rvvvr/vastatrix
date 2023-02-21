use broom::trace::Trace;

use crate::class::{frame::Frame, Class, method::MethodType};

pub enum VTXObject {
    Class(Class),
    Array(Vec<MethodType>)
}

impl Trace<Self> for VTXObject {
    fn trace(&self, tracer: &mut broom::trace::Tracer<Self>) {
        match self {
            VTXObject::Class(_) => {},
            VTXObject::Array(elements) => {
                elements.trace(tracer)
            }
        }
    }
}

pub struct Vastatrix {
    pub heap: broom::Heap<VTXObject>,
}

impl Vastatrix {
    pub fn new() -> Self {
        Self {
            heap: broom::Heap::default(),
        }
    }
}