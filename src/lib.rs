#![allow(unused_parens)]
#![deny(unconditional_recursion)]

#![feature(const_type_name)]
#![feature(bigint_helper_methods)]
#![feature(strict_overflow_ops)]
#![feature(mixed_integer_ops_unsigned_sub)]
#![feature(unique_rc_arc)]
#![feature(let_chains)]
#![feature(coroutines)]

extern crate core;

pub mod memory_primitives;
pub mod word;
pub mod application;
pub mod tools;

pub const PROGRAM_COUNTER_REGISTER_ADDR: usize = 63;

pub type Step = u32;


pub trait Draw{
	fn draw(&self);
}