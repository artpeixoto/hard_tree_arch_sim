use std::fmt::Debug;
use crate::alu::{AluConfigBank, AluConfigBankWriter};
use crate::cpu_registers::{CpuRegisterBank, CpuRegisterBankReader, CpuRegisterBankWriter};
use crate::instruction::Instruction;
use crate::instruction_reader::{IncrementCmd, InstructionMemory, InstructionReader};
use crate::instruction_reader::IncrementCmd::NoIncrement;
use crate::main_memory::MainMemory;

#[derive( PartialEq, Eq, Copy, Clone, Debug, )]
pub enum ControllerState{
	Ready,
	Waiting,
}

pub struct Controller{
	cpu_registers_reader	: CpuRegisterBankReader,
	cpu_registers_writer	: CpuRegisterBankWriter,
	alu_bank_configurator	: AluConfigBankWriter,
	instruction_reader  	: InstructionReader,
	state					: ControllerState 
}

impl Controller{
	pub fn new(
		main_memory			: &mut MainMemory,
		instruction_memory	: &InstructionMemory,
		alu_config_bank		: &mut AluConfigBank,
		cpu_registers		: &mut CpuRegisterBank,
	) -> Self {
		let instruction_reader = InstructionReader::new(
			instruction_memory,
			cpu_registers,
			main_memory,	
		);
			
		let configurator = alu_config_bank.get_writer();
			
		Controller{
			cpu_registers_reader: cpu_registers.get_reader(),
			cpu_registers_writer: cpu_registers.get_writer(),
			alu_bank_configurator: configurator,
			instruction_reader,
			state: ControllerState::Ready
		}	
	}
}

impl Controller{
		
}
