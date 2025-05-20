use rust_hdl::prelude::*;
use crate::memory_primitives::register_bank::*;
use crate::word::{Word, WORD_SIZE};

pub const MAIN_MEMORY_SIZE: usize = 1024;
// very basic. ill make this better later, i guess
pub type MainMemory 		= RegisterBank<Word, WORD_SIZE, MAIN_MEMORY_SIZE>;
pub type MainMemoryReader 	= RegisterBankReader<Word, WORD_SIZE, MAIN_MEMORY_SIZE>;
pub type MainMemoryWriter = RegisterBankWriter<Word, WORD_SIZE, MAIN_MEMORY_SIZE>;
pub type MainMemoryRw = RegisterBankRw<Word, WORD_SIZE, MAIN_MEMORY_SIZE>;
