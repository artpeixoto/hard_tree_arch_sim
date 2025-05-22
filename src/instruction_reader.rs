use rust_hdl::prelude::*;

use crate::{instruction::{ControllerInstruction, CONTROLLER_INSTRUCTION_SIZE}, cpu_registers::{CpuRegisterBankReader, CpuRegisterBankWriter, CpuRegisterReader, CpuRegisterWriter}, main_memory::{MainMemory, MainMemoryReader}, memory_primitives::register::RegisterReader, word::ToWord, PROGRAM_COUNTER_REGISTER_ADDR};
use crate::cpu_registers::CpuRegisterBank;
use crate::tools::sign::SignedCast;
use crate::tools::to_i32::ToI32;
use crate::word::WORD_SIZE;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, LogicState)]
pub enum IncrementCmd{
	NoIncrement,
	Increment,
	GoTo,
}

#[derive(LogicBlock)]
pub struct InstructionReader{
	pub increment			: Signal<In, IncrementCmd>,
	pub increment_amount	: Signal<In, Bits<{ WORD_SIZE }>>,

	pub instruction			: Signal<Out, ControllerInstruction>,

	program_counter_reader	: CpuRegisterReader,
	program_counter_writer  : CpuRegisterWriter,

	first_word_reader		: MainMemoryReader,
	second_word_reader		: MainMemoryReader,
}

impl InstructionReader{
	pub fn new(
		cpu_registers	: &mut CpuRegisterBank,
		main_memory		: &mut MainMemory,
	) -> InstructionReader{
		Self{
			increment				: Default::default(),
			increment_amount		: Default::default(),
			instruction				: Default::default(),
			program_counter_reader	: cpu_registers.get_specific_reader(PROGRAM_COUNTER_REGISTER_ADDR),
			program_counter_writer	: cpu_registers.get_specific_writer(PROGRAM_COUNTER_REGISTER_ADDR),
			first_word_reader		: main_memory.get_reader(),
			second_word_reader		: main_memory.get_reader(),
		}
	}
}

impl Logic for InstructionReader{
	// #[hdl_gen]
	fn update(&mut self) {
		self.first_word_reader.address.next = self.program_counter_reader.value.val().into();
		self.second_word_reader.address.next = ((self.program_counter_reader.value.val().inner + 1));

		match self.increment.val(){
			IncrementCmd::NoIncrement => {
				self.program_counter_writer.write_enable.next = false;
			},
			IncrementCmd::Increment => {
				self.program_counter_writer.write_enable.next = true;
				self.program_counter_writer.write_value.next =
					(	2 * self.increment_amount.val().to_i32()

						+ self.program_counter_reader.value.val().to_i32()
					)
					.to_word()
				;
			},
			IncrementCmd::GoTo => {
				self.program_counter_writer.write_enable.next = true;
				self.program_counter_writer.write_value.next =
					(2 * self.increment_amount.val().to_i32()).to_word();
			},
		}

		self.instruction.next =
			( (bit_cast::<CONTROLLER_INSTRUCTION_SIZE, 32>(self.first_word_reader.value.val().inner) << 32)
			| (bit_cast::<CONTROLLER_INSTRUCTION_SIZE, 32>(self.second_word_reader.value.val().inner))
			)
			.into();
	}
}

