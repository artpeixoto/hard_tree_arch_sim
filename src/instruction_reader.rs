use std::ops::Deref;
use std::sync::Arc;
use bevy::ecs::relationship::Relationship;
use bevy::prelude::Component;
use crate::{instruction::{Instruction, CONTROLLER_INSTRUCTION_SIZE}, cpu_registers::{CpuRegisterReader, CpuRegisterWriter}, main_memory::{MainMemory}, memory_primitives::register::RegisterReader, PROGRAM_COUNTER_REGISTER_ADDR, ClockTransition, Step};
use crate::cpu_registers::CpuRegisterBank;
use crate::main_memory::MainMemoryIo;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
pub enum IncrementCmd{
	NoIncrement,
	Increment,
	GoTo,
}
#[derive(Component)]
pub struct InstructionMemory(
	pub Arc<Vec<Instruction>>,
);
impl InstructionMemory{
	pub fn new(program: Vec<Instruction>) -> Self {
		Self(Arc::new(program))
	}
}
pub struct InstructionReader{
	program_counter_reader	: CpuRegisterReader,
	program_counter_writer  : CpuRegisterWriter,
	data_memory				: MainMemoryIo,
	increment_cmd			: IncrementCmd,
	instruction_memory		: Arc<Vec<Instruction>>,
}


impl InstructionReader{
	pub fn new (
		instruction_memory	: &InstructionMemory,
		cpu_registers		: &mut CpuRegisterBank,
		main_memory			: &mut MainMemory,
	) -> InstructionReader {
		Self{
			instruction_memory		: instruction_memory.0.clone(),
			program_counter_reader	: cpu_registers.get_specific_reader(PROGRAM_COUNTER_REGISTER_ADDR),
			program_counter_writer	: cpu_registers.get_specific_writer(PROGRAM_COUNTER_REGISTER_ADDR),
			increment_cmd			: IncrementCmd::NoIncrement,
			data_memory				: main_memory.get_io()
		}
	}
}

impl InstructionReader{
	pub fn set_increment_cmd(&mut self, cmd: IncrementCmd){
		self.increment_cmd = cmd;
	}

	pub fn get_current_instruction<'a>(&'a self) -> impl Deref<Target=Instruction> + 'a{
		let addr = *self.program_counter_reader.read() as usize;
		self.instruction_memory.get(addr).unwrap()
	}
}

impl ClockTransition for InstructionReader{
	fn step(&mut self, step: &Step) {
		todo!()
	}
}