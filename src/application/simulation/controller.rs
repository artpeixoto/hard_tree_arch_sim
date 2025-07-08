use std::fmt::Debug;
use std::ops::Deref;
use crate::application::simulation::alu::{AluAddress, AluOperation, ALU_COUNT};
use crate::application::simulation::cpu_registers::{CpuRegisterAddress, CpuRegisterBank, CpuRegisterDataReader, CpuRegisterDataWriter};
use crate::application::simulation::instruction::Instruction;
use crate::application::simulation::instruction_reader::{IncrementCmd, InstructionMemory, InstructionReader};
use crate::application::simulation::instruction_reader::IncrementCmd::{GoTo, Increment, NoIncrement};
use crate::application::simulation::main_memory::MainMemory;
use crate::{Step};
use crate::word::ToBool;

#[derive( PartialEq, Eq, Copy, Clone, Debug, )]
pub enum ControllerExecutionState {
	Running,
	WaitingForActivation,
}
pub struct Controller{
	pub cpu_registers_reader	: CpuRegisterDataReader,
	pub cpu_registers_writer	: CpuRegisterDataWriter,
	pub alu_config_writer		: AluConfigWriter	,
	state						: ControllerExecutionState,
	pub instruction_reader  	: InstructionReader,
	previous_instruction		: Option<Instruction>,
}


impl Controller{
	pub fn new(
		instruction_memory	: &InstructionMemory,
	) -> Self {
		let instruction_reader = InstructionReader::new(
			instruction_memory,
		);
			
		let configurator = AluConfigWriter::Deactivated;
			
		Controller{
			previous_instruction: None,
			cpu_registers_reader: CpuRegisterDataReader::new(),
			cpu_registers_writer: CpuRegisterDataWriter::new(),
			alu_config_writer   : configurator,
			instruction_reader,
			state				: ControllerExecutionState::Running
		}	
	}

	pub fn reset_outputs(&mut self){
		self.alu_config_writer 	  = AluConfigWriter::Deactivated;
		self.cpu_registers_writer = CpuRegisterDataWriter::Deactivated;
	}
	
	pub fn execute(&mut self) {
		match self.state {
			ControllerExecutionState::Running => {
				let current_instruction = *self.instruction_reader.read();
				match current_instruction {
					Instruction::SetAluConfig {  alu_config, alu_addr, } => {
						self.alu_config_writer = AluConfigWriter::WritingToSingle{
							target: alu_addr,
							op: alu_config
						};

						self.instruction_reader.set_increment_cmd(Increment);
					}
					Instruction::SetLiteral {  literal , register,} => {
						self.cpu_registers_writer.set_connection(Some(register));
						self.cpu_registers_writer.write(literal);
						self.instruction_reader.set_increment_cmd(Increment);
					}
					Instruction::WaitForActivationSignal { register_index } => {
						self.state =  ControllerExecutionState::WaitingForActivation;
						self.cpu_registers_reader.set_connection(Some(register_index));
						self.instruction_reader.set_increment_cmd(NoIncrement);
					}
					Instruction::Jump { addr } => {
						self.instruction_reader.set_increment_cmd(GoTo(addr));
					}
					Instruction::ResetAll => {
						self.alu_config_writer = AluConfigWriter::WritingToAll {op:
						AluOperation::NoOp};
						self.instruction_reader.set_increment_cmd(Increment);
					}
					Instruction::NoOp => {
						self.instruction_reader.set_increment_cmd(Increment);
					}
				}
			}
			ControllerExecutionState::WaitingForActivation => {
				let is_activated = self.cpu_registers_reader.read().unwrap().to_bool();
				if is_activated {
					self.instruction_reader.set_increment_cmd(Increment);
					self.state =  ControllerExecutionState::Running;
				} else {
					self.instruction_reader.set_increment_cmd(NoIncrement);
				}
			}
		}

		self.instruction_reader.step();
	}
}

pub enum AluConfigWriter{
	Deactivated,
	WritingToSingle{
		target	: AluAddress,
		op		: AluOperation,
	},
	WritingToAll{
		op		: AluOperation,
	}
}


