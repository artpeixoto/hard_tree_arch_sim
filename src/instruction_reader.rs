use rust_hdl::prelude::*;

use crate::{controller::ControllerInstruction, cpu_registers::{CpuRegisterBankReader, CpuRegisterBankWriter, CpuRegisterReader, CpuRegisterWriter}, main_memory::{MainMemory, MainMemoryReader}, memory_primitives::register::RegisterReader, word::ToWord};


#[derive(LogicBlock)]
pub struct InstructionReader{
	increment				: Signal<In, Bit>,

	instruction				: Signal<Out, ControllerInstruction>,
	program_counter_reader	: CpuRegisterReader,
	program_counter_writer  : CpuRegisterWriter,


	first_word_reader		: MainMemoryReader,
	second_word_reader		: MainMemoryReader,
}

impl Logic for InstructionReader{
	// #[hdl_gen]
	fn update(&mut self) {
		self.first_word_reader.address.next = self.program_counter_reader.value.val().into();
		self.second_word_reader.address.next = ((self.program_counter_reader.value.val().inner + 1));

		self.program_counter_writer.write_enable.next = false;

		if self.increment.val(){
			self.program_counter_writer.write_value.next = (self.program_counter_reader.value.val().to_i32() + 2).to_word();
			self.program_counter_writer.write_enable.next= true;
		}

		self.instruction.next = ((self.first_word_reader.value.val().inner.get_bits::<64>(0) << 32)
		| self.second_word_reader.value.val().inner.get_bits::<64>(0)).into();

	}
}

