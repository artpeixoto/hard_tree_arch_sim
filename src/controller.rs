use std::fmt::Debug;
use itertools::Itertools;
use rust_hdl::prelude::*;
use crate::alu::{AluBankConfigurator, AluConfigBank};
use crate::cpu_registers::{CpuRegisterBank, CpuRegisterBankRw};
use crate::instruction::ControllerInstruction;
use crate::instruction_reader::{IncrementCmd, InstructionReader};
use crate::instruction_reader::IncrementCmd::NoIncrement;
use crate::main_memory::MainMemory;
use crate::memory_primitives::register::RegisterRwCommand;

#[derive( PartialEq, Eq, Copy, Clone, Debug, LogicState)]
pub enum ControllerState{
	Ready,
	Busy,
	Readying,
}


#[derive(LogicBlock)]
pub struct Controller{
	cpu_registers		 	: CpuRegisterBankRw,
	alu_bank_configurator	: AluBankConfigurator,
	instruction_reader  	: InstructionReader,
	state					: DFF<ControllerState>,
	clock					: Signal<In, Clock>
}
impl Controller{
	pub fn new(
		clock			: &Signal<In, Clock>,
		main_memory		: &mut MainMemory,
		alu_config_bank	: &mut AluConfigBank,
		cpu_registers	: &mut CpuRegisterBank,
	) -> Self {
		let instruction_reader = InstructionReader::new(
			cpu_registers,
			main_memory	
		);		
			
		let configurator = alu_config_bank.get_configurator();
		let clock =  clock.clone();
			
		Controller{
			cpu_registers: cpu_registers.get_reader_writer(),
			alu_bank_configurator: configurator,
			instruction_reader,
			clock,
			state: Default::default(),
		}	
	}
}
impl Logic for Controller {
	fn update(&mut self) {
		self.state.d.next 		= self.state.q.val();
		self.state.clock.next 	= self.clock.val();

		if self.state.q.val() == ControllerState::Readying {
			self.state.d.next = ControllerState::Ready;
		}

		match self.state.q.val() {
			ControllerState::Ready => {
				self.instruction_reader.increment.next = IncrementCmd::Increment;
				self.instruction_reader.increment_amount.next = 1_u32.to_bits();
			}
			ControllerState::Readying |
			ControllerState::Busy 	=> {
				self.instruction_reader.increment.next = NoIncrement;
			}
		}

		self.cpu_registers.command.next = RegisterRwCommand::Read;

		self.alu_bank_configurator.write_enable.next 	= false;
		self.alu_bank_configurator.target_all.next 		= false;

		match self.instruction_reader.instruction.val(){
			ControllerInstruction::SetAluConfig { alu_addr, alu_config } => {
				if self.state.q.val() == ControllerState::Ready {
					self.alu_bank_configurator.addr.next = alu_addr;
					self.alu_bank_configurator.config_value.next = alu_config;
					self.alu_bank_configurator.write_enable.next = true;
				} else {
					self.state.d.next = ControllerState::Ready;
				}
			}
			ControllerInstruction::SetLiteral { register_index, literal } => {
				if self.state.q.val() == ControllerState::Ready {
					self.cpu_registers.address.next = register_index;
					self.cpu_registers.value.next = literal;
					self.cpu_registers.command.next = RegisterRwCommand::Write;
				} else {
					self.state.d.next = ControllerState::Ready;
				}
			}

			// ControllerInstruction::PopStack { .. } => {}
			// ControllerInstruction::PushToStack { .. } => {}

			ControllerInstruction::WaitForActivationSignal { register_index } => {
				self.cpu_registers.address.next = register_index;
				self.cpu_registers.command.next = RegisterRwCommand::Read;

				match self.state.q.val() {
					ControllerState::Ready  => {
						self.state.d.next = ControllerState::Busy;
					},
					ControllerState::Busy => {
						if self.cpu_registers.value.val().into() {
							self.state.d.next = ControllerState::Readying;
						}
					}
					ControllerState::Readying => {

					}
				}
			}
			ControllerInstruction::Jump { relative, addr } => {
				match self.state.q.val() {
					ControllerState::Ready  => {
						self.state.d.next = ControllerState::Busy;
					},
					ControllerState::Busy => {
						self.instruction_reader.increment_amount.next = addr.inner;
						if relative{
							self.instruction_reader.increment.next = IncrementCmd::Increment;
						} else {
							self.instruction_reader.increment.next = IncrementCmd::Increment;
						}
						self.state.d.next = ControllerState::Readying;
					}
					ControllerState::Readying => { }
				}
			}
			ControllerInstruction::ResetAll => {
				self.alu_bank_configurator.config_value.next = Default::default();
				self.alu_bank_configurator.target_all.next = true;
			}
			ControllerInstruction::NoOp => { }
		}
	}
}
