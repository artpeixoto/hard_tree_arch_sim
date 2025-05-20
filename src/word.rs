use std::ops::{Add, BitAnd, BitOr, BitXor, Neg, Not, Sub};
use rust_hdl::prelude::*;

pub const WORD_SIZE					: usize = 32;

#[derive(Default, PartialOrd, PartialEq, Eq, Clone, Copy, Debug, LogicStruct)]
pub struct Word{
    pub inner: Bits<WORD_SIZE>,
}
impl BitXor for Word{
	type Output = Self;

	fn bitxor(self, rhs: Self) -> Self::Output {
		(self.inner ^ rhs.inner).into()
	}
}
pub trait ToWord{
	fn to_word(self) -> Word;
}
impl<T: Into<Word>> ToWord for T{
	fn to_word(self) -> Word {
		self.into()
	}
}

impl Neg for Word{
    type Output = Word;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        self.to_i32().neg().into()
    }
}
impl Add for Word{
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        (self.to_i32() + rhs.to_i32()).into()
    }
}
impl Sub for Word{
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        (self.to_i32() - rhs.to_i32()).to_word()
    }
}

impl BitOr for Word{
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        self.inner.bitor(rhs.inner).to_word()
    }
}

impl BitAnd for Word{
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        self.inner.bitand(rhs.inner).to_word()
    }
}
impl Not for Word{
    type Output = Self;
    #[inline(always)]
    fn not(self) -> Self::Output {
        self.inner.not().into()
    }
}

impl Into<Word> for Bits<WORD_SIZE>{
    #[inline(always)]
    fn into(self) -> Word {
        Word{inner: self}
    }
}
impl Into<Word> for i32{
    #[inline(always)]
    fn into(self) -> Word {
        Word{inner: self.to_signed_bits().inner()}
    }
}
impl Into<i32> for Word{
    #[inline(always)]
    fn into(self) -> i32 {
        self.to_i32()
    }
}

impl Into<Word> for bool{
    #[inline(always)]
    fn into(self) -> Word{
        match self{
            true => {
                // basically, 0xFF_FF_FF_FF
                // maybe i should use 0x1?
                !0.to_word()
				
            }
            false => {
                0.to_word()
            }
        }
    }
}

impl Into<bool> for Word{
    fn into(self) -> bool{
        self.inner.any() 
    }
}

impl Word{
    #[inline(always)]
    pub fn to_i32(self) -> i32{
        self.inner.get_bits::<{WORD_SIZE-1}>(0) .to_u32() as i32
            + {
            if self.inner.get_bit(WORD_SIZE - 1) {
                -2_i32.pow(WORD_SIZE as u32 - 1)
            } else {
                0
            }
        }
    }
}
