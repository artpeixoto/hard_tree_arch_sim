use std::mem::transmute;
use rust_hdl::prelude::{Bits, Signed};
use crate::word::Word;

pub trait ToI32{
    fn to_i32(self) -> i32;
}
impl<const N: usize> ToI32 for Signed<N> {
    fn to_i32(self) -> i32 {
        unsafe { transmute(self.inner().to_u32()) }
    }
}

impl<const N: usize> ToI32 for Bits<N> {
    fn to_i32(self) -> i32 {
        unsafe { transmute(self.to_u32()) }
    }
}