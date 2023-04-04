use super::method::Argument;
use crate::vastatrix::{Vastatrix};

pub trait Frame: core::fmt::Debug {
    fn exec(&mut self, args: Vec<Argument>, running_in: &mut Vastatrix) -> Argument;
}