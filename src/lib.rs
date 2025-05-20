#![allow(unused_parens)]

pub mod memory_primitives;
pub mod cpu_registers;
pub mod controller;
pub mod alu;
pub mod word;
pub mod instruction_reader;
pub mod main_memory;

use std::{array, ops::BitOr};
use std::sync::LazyLock;
use alu::{AluAddr, AluConfig};
use controller::ControllerInstruction;
use cpu_registers::{CpuRegistersAddress, CpuRegisterBankReader};
use memory_primitives::register_bank::{RegisterBank, RegisterBankReader, RegisterBankWriter};
use rust_hdl::prelude::*;
use word::{ToWord, Word};


pub static PROGRAM_COUNTER_REGISTER	
    : LazyLock<CpuRegistersAddress> = LazyLock::new(||Bits::from(63));
pub static STACK_POINTER_REGISTER  	
    : LazyLock<CpuRegistersAddress> = LazyLock::new(||Bits::from(62));
pub static RETURN_POINTER_REGISTER  
    : LazyLock<CpuRegistersAddress> = LazyLock::new(||Bits::from(61));




