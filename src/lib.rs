#![allow(unused_parens)]
#![feature(const_type_name)]

pub mod memory_primitives;
pub mod cpu_registers;
pub mod instruction;
pub mod alu;
pub mod controller;
pub mod word;
pub mod instruction_reader;
pub mod main_memory;
mod cpu;
mod sim;



pub const PROGRAM_COUNTER_REGISTER_ADDR: usize = 63;


pub type Step = u32;
pub trait ClockTransition {
	fn step(&mut self, step: &Step);
}
