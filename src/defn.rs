use std::array;
use std::collections::hash_map::Keys;
use std::num::NonZero;
use std::ops::{Add, BitAnd, BitOr, Neg, Not, Sub};
use std::sync::LazyLock;
use rust_hdl::prelude::*;
use rust_hdl::sim::sdr_sdram::bank::MemoryBank;
use rust_hdl::widgets::edge_ff::EdgeDFF;
use rust_hdl::widgets::fifo::fifo_logic::FIFOWriteLogic;

pub const ALU_INDEX_SIGNAL_SIZE		: usize = clog2(ALU_COUNT);
pub const ALU_COUNT			 		: usize = 32;
pub const REGISTER_INDEX_SIGNAL_SIZE: usize = clog2(REGISTER_COUNT);
pub const REGISTER_COUNT	 		: usize = 64;
pub const INSTRUCTION_SIZE   		: usize = 64;
pub const DATA_SIZE					: usize = 32;



pub static PROGRAM_COUNTER_REGISTER	: LazyLock<RegisterIndex> = LazyLock::new(||Bits::from(63));
pub static CONTROLLER_ACTIVATION	: LazyLock<RegisterIndex> = LazyLock::new(||Bits::from(62));
pub static STACK_POINTER_REGISTER  	: LazyLock<RegisterIndex> = LazyLock::new(||Bits::from(61));
pub static RETURN_POINTER_REGISTER  : LazyLock<RegisterIndex> = LazyLock::new(||Bits::from(60));


pub struct Controller{
	instruction_reader
}

#[derive(LogicBlock)]
pub struct AlusConfigurator{
	pub clock		: Signal<In, Clock>,
	pub enable		: Signal<In, bool>,
	pub alu_index	: Signal<In, AluIndex>,
	alus			: []
}


impl Synth for ControllerCmd{
	const BITS: usize = 64;

	fn descriptor() -> TypeDescriptor {
		TypeDescriptor{
			name: "ControllerCmd".to_string(),
			kind: TypeKind::Bits(64),
		}
	}

	fn vcd(self) -> VCDValue {
		todo!()
	}

	fn verilog(self) -> VerilogLiteral {
		todo!()
	}
}

#[derive( PartialEq, Copy, Clone, Debug,Eq,Default)]
pub enum ControllerCmd{
	SetAluConfig{
		alu			: AluIndex,
		alu_config	: AluConfig
	},
	SetLiteral{
		register_index  : RegisterIndex,
		literal			: Word,
	},
	PopStack{
		register_index	: RegisterIndex,
	},
	PushToStack{
		register_index	: RegisterIndex,
	},
	ResetAll,

	#[default]
	NoOp,
}


pub type AluIndex					= Bits<ALU_INDEX_SIGNAL_SIZE>;
pub type AluIndexSignal<Dir>		= Signal<Dir, Bits<ALU_INDEX_SIGNAL_SIZE>>;
pub type RegisterIndex  			= Bits<REGISTER_INDEX_SIGNAL_SIZE>;
pub type RegisterIndexSignal<Dir>  	= Signal<Dir, Bits<REGISTER_INDEX_SIGNAL_SIZE>>;

#[derive(Clone, PartialEq, Eq, Debug, Copy, Default,LogicStruct )]
pub struct AluConfig {
	execution_signal_input	: RegisterIndex,
	execution_signal_output	: RegisterIndex,
	data_input_0			: RegisterIndex,
	data_input_1			: RegisterIndex,
	main_data_output		: RegisterIndex,
	aux_data_output			: RegisterIndex,
	aux_output_activated	: bool,
	operation				: AluOperation
}

pub const ALU_CONFIG_SIGNAL_SIZE: usize = AluOperation::BITS + (6 * RegisterIndex::BITS) + 1;

pub type AluConfigSignal<Dir> = Signal<Dir, AluConfig>;

#[derive(Clone, PartialEq, Eq, Debug, Copy, Default, LogicState)]
pub enum AluOperation{
	#[default]
	NoOp,
	Eq,
	Mov,

	Not, And, Or, Xor,
	ShiftLeft, ShiftRight,
	SelectPart,
	Add, Sub,
	Mul, Div, Rem,
	Neg,

	ReadFromMem, WriteToMem,
}
#[derive(LogicBlock)]
pub struct Registers {
	clock		: Signal<In, Clock>,
	registers	: Box<[ DFF<Word>; REGISTER_COUNT]>,
	writers		: Box<[ (Signal<InOut, Word>); REGISTER_COUNT ]>,
}

impl Logic for Registers{
	fn update(&mut self) {
		self.clock.next = self.clock.val();
		for  i in 0..REGISTER_COUNT {
			self.registers[i].clock.next = self.clock.val();
			if self.writers[i].is_driving_tristate()	{
				self.registers[i].d.next = self.writers[i].val();
			} else {
				self.registers[i].d.next = self.registers[i].q.val();
			}
		}
	}
}

impl Registers{
	pub fn get_reader(&mut self) -> RegisterDataReader {
		let mut register_inputs =
			array::from_fn::< Signal<In, Word>, REGISTER_COUNT,_>(|_| Default::default());
		for i in 0..REGISTER_COUNT {
			self.registers[i.clone()].q.join(&mut register_inputs[i])
		}

		RegisterDataReader {
			registers		: register_inputs,
			register_index	: Default::default(),
			value			: Default::default(),
		}
	}
}

#[derive(LogicBlock)]
pub struct RegisterDataReader {
	register_index 	: Signal<In, RegisterIndex>,
	registers		: [Signal<In, Word>; REGISTER_COUNT],
	value: Signal<Out, Word>,
}

impl Logic for RegisterDataReader {
	#[hdl_gen]
	fn update(&mut self) {
		self.value.next = self.registers[self.register_index.val().index()].val();
	}
}
#[derive(LogicBlock)]
pub struct RegisterActivationReader {
	pub register_index	: Signal<In, RegisterIndex>,
	pub value			: Signal<Out, Word>,
	inner				: RegisterDataReader,
}
impl Logic for RegisterActivationReader {
	fn update(&mut self) {
		self.inner.register_index.next = self.register_index.val();
		self.value.next = self.inner.value.val();
	}
}

#[derive(LogicBlock)]
pub struct RegisterDataWriter{
	pub enable			: Signal<In, bool>,
	pub register_index	: Signal<In, RegisterIndex>,
	pub value			: Signal<In, Word>,
	registers			: [ TristateBuffer<Word>; REGISTER_COUNT],
}
impl Logic for RegisterDataWriter{
	#[hdl_gen]
	fn update(&mut self) {
		for i in  0..REGISTER_COUNT {
			self.registers[i].write_enable.next = false;
		}

		if self.enable.val(){
			self.registers[self.register_index.val().index()].write_enable.next = true;
			self.registers[self.register_index.val().index()].write_data.next= self.value.val();
		}
	}
}
impl Registers {
	pub fn get_writer(&mut self) -> RegisterDataWriter {
		let mut register_buffers =
			array::from_fn::<TristateBuffer<Word>, REGISTER_COUNT, _>(|_| Default::default());

		for i in 0..REGISTER_COUNT {
			self.writers[i].join(&mut register_buffers[i].bus);
		}
		RegisterDataWriter{
			registers: register_buffers,
			value: Default::default(),
			enable: Default::default(),
			register_index:  Default::default(),
		}
	}
}

#[derive(LogicBlock)]
pub struct AluConfigMem{
	clock					: Signal<In, Clock>,
	enable_write			: Signal<In, bool>,
	input					: Signal<In, AluConfig>,
	value					: DFF<AluConfig>,
	output					: Signal<Out, AluConfig>,
	is_ready				: Signal<Out, bool>,
}
pub struct AluConfigMemWriter{
	// clock				: Signal<In, Clock>,
	write				: Signal<In, bool>,
	value				: Signal<In, AluConfig>,
}
impl AluConfigMem{
	pub fn get_config_writer(&self) -> AluConfigMemWriter{
		AluConfigMemWriter{
			write: self.enable_write.clone();
			value: Default::default(),
		}
	}
}

impl Logic for AluConfigMem{
	fn update(&mut self) {
		dff_setup!(self, clock, value);
		if self.clock.pos_edge() && self.enable_write.val(){
			self.value.d.next = self.input.val();
		}
		self.is_ready.next = !self.enable_write.val();
		self.output.next = self.value.q.val();
	}
}


#[derive(Default, PartialOrd, PartialEq, Eq, Clone, Copy, Debug, LogicStruct)]
pub struct Word{
	pub inner: Bits<DATA_SIZE>,
}
impl Neg for Word{
	type Output = Word;
	#[inline(always)]
	fn neg(self) -> Self::Output {
		self.to_i32().neg().into()
	}
}
impl Add for Word{
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: Self) -> Self::Output {
		(self.to_i32() + rhs.to_i32())
	}
}
impl Sub for Word{
	type Output = Self;
	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output {
		(self.to_i32() - rhs.to_i32())
	}
}

impl BitOr for Word{
	type Output = Self;
	#[inline(always)]
	fn bitor(self, rhs: Self) -> Self::Output {
		self.inner.bitor(rhs.inner)
	}
}
impl BitAnd for Word{
	type Output = Self;
	#[inline(always)]
	fn bitand(self, rhs: Self) -> Self::Output {
		self.inner.bitand(rhs.inner)
	}
}
impl Not for Word{
	type Output = Self;
	#[inline(always)]
	fn not(self) -> Self::Output {
		self.inner.not().into()
	}
}

impl Into<Word> for Bits<DATA_SIZE>{
	#[inline(always)]
	fn into(self) -> Word {
		Word{inner: self}
	}
}
impl Into<Word> for i32{
	#[inline(always)]
	fn into(self) -> Word {
		Word{inner: self.to_signed_bits().inner()}
	}
}
impl Into<i32> for Word{
	#[inline(always)]
	fn into(self) -> i32 {
		self.to_i32()
	}
}

impl Into<Word> for bool{
	#[inline(always)]
	fn into(self) -> Word{
		match self{
			true => {
				// basically, 0xFF_FF_FF_FF
				// maybe i should use 0x1?
				!0.into()
			}
			false => {
				0.into()
			}
		}
	}
}

impl Word{
	#[inline(always)]
	pub const fn to_i32(self) -> i32{
		self.inner.get_bits::<{DATA_SIZE-1}>(0) .to_u32() as i32
		+ {
			if self.inner.get_bit(DATA_SIZE - 1) {
				-2_i32.pow(DATA_SIZE as u32 - 1)
			} else {
				0
			}
		}
	}
}


#[derive(LogicBlock)]
pub struct RegisterActivationWriter{
	pub value			: Signal<In , Bit>,
	pub register_index	: Signal<In , RegisterIndex>,
	pub registers		: [Signal<Out, Word>; REGISTER_COUNT],
}

impl Logic for  RegisterActivationWriter{
	fn update(&mut self) {
		self.registers [self.register_index.val().index()].next =  self.value.val().into();
	}
}
#[derive(LogicBlock)]
pub struct AluCore {
	config					: AluConfigMem,
	clock					: Signal<In, Clock>,
	// working					: DFF<Bit>,

	memory_io				: MemoryIo,

	data_input_0			: RegisterDataReader,
	data_input_1			: RegisterDataReader,
	activation_input		: RegisterActivationReader,

	main_data_output		: RegisterDataWriter,
	aux_data_output			: RegisterDataWriter,
	activation_output		: RegisterActivationWriter,
}

impl Logic for AluCore {
	fn update(&mut self) {
		// reset everything
		self.main_data_output.enable.next = false;
		self.aux_data_output.enable.next = false;
		self.activation_output.value.next = false;
		self.memory_io.command.next = MemoryIoCommand::Nop;

		if self.config.is_ready.val() {
			self.activation_input.register_index.next
				= self.config.output.val().execution_signal_input;

			self.activation_output.register_index.next
				= self.config.output.val().execution_signal_output;

			self.data_input_0.register_index.next
				= self.config.output.val().data_input_0;

			self.data_input_1.register_index.next
				= self.config.output.val().data_input_1;

			self.main_data_output.register_index.next
				= self.config.output.val().main_data_output;

			self.aux_data_output.register_index.next
				= self.config.output.val().aux_data_output;
		}

		if self.config.is_ready.val() && self.clock.val().clk {
			if self.config.output.val().operation != AluOperation::NoOp{
				self.main_data_output.enable.next = true;
				match self.config.output.val().operation {
					AluOperation::Eq 		=> {
						self.main_data_output.value.next =
							if self.data_input_0.value.val() == self.data_input_1.value.val(){
								1.into()
							} else {
								0.into()
							};
					}
					AluOperation::Mov 		=> {
						self.main_data_output.value.next = self.data_input_0.value.val();
					}
					AluOperation::Not 		=> {
						self.main_data_output.value.next = !self.data_input_0.value.val();
					}
					AluOperation::And 		=> {
						self.main_data_output.value.next =
							self.data_input_0.value.val() & self.data_input_1.value.val();
					}
					AluOperation::Or 		=> {
						self.main_data_output.value.next =
							self.data_input_0.value.val().bitor(  self.data_input_1.value.val() )
						;
					}
					AluOperation::Xor 			=> {
						self.main_data_output.value.next =
							self.data_input_0.value.val() ^ self.data_input_1.value.val();
					}
					AluOperation::ShiftLeft 	=> {
						self.main_data_output.value.next =
							self.data_input_0.value.val() << self.data_input_1.value.val();
					}
					AluOperation::ShiftRight	=> {
						self.main_data_output.value.next =
							self.data_input_0.value.val() >> self.data_input_1.value.val();
					}
					AluOperation::SelectPart => {
						todo!();
						// self.main_data_output.value.next =
						// 	self.data_input_0.out.val().get_bits::<5>();
						self.activation_output.value.next = true;
					}
					AluOperation::Add 		=> {
						self.main_data_output.value.next =
							self.data_input_0.value.val() + self.data_input_1.value.val();
					}
					AluOperation::Sub 		=> {
						self.main_data_output.value.next =
							self.data_input_0.value.val() - self.data_input_1.value.val();
					}
					AluOperation::Mul 		=> {
						self.main_data_output.value.next =
							( self.data_input_0.value.val().to_i32()
							* self.data_input_1.value.val().to_i32()
							)
							.to_bits();

						self.aux_data_output.value.next =
							(
								(
									( self.data_input_0.value.val().to_i32() as i64
									* self.data_input_1.value.val().to_i32() as i64
									)
									>> 32
								) as i32
							).into() //not pretty at all, but it is necessary due to the hdl_gen macro.
							;
					}
					AluOperation::Div 		=> {
						self.aux_data_output.enable.next
							= self.config.output.val().aux_output_activated;

						if self.data_input_1.value.val() == 0 {
							self.aux_data_output.value.next = 1.into();
						} else {
							self.aux_data_output.value.next = 0.into();
							self.main_data_output.value.next =
								( 	self.data_input_0.value.val().to_i32()
								/ 	self.data_input_1.value.val().to_i32()
								)
								.into();
						}
					}
					AluOperation::Rem 		=> {
						self.aux_data_output.enable.next
							= self.config.output.val().aux_output_activated;

						if self.data_input_1.value.val() == 0 {
							self.aux_data_output.value.next = 1.into();
						} else {
							self.aux_data_output.value.next = 0.into();
							self.main_data_output.value.next =
								( 	self.data_input_0.value.val().to_i32()
								% 	self.data_input_1.value.val().to_i32()
								)
								.to_bits();
						}
					}
					AluOperation::Neg 		=> {
						self.main_data_output.value.next =
							 - self.data_input_0.value.val()
					}
					AluOperation::ReadFromMem => {
						self.memory_io.address.next = self.data_input_0.value.val();
						self.memory_io.command.next = MemoryIoCommand::Read;
						self.main_data_output.value.next = self.memory_io.value.val();
					}
					AluOperation::WriteToMem => {
						self.memory_io.address.next = self.data_input_0.value.val();
						self.memory_io.value.next = self.data_input_1.value.val();
						self.memory_io.command.next = MemoryIoCommand::Write;
					}
					AluOperation::NoOp => { unreachable!() }
				}
				self.main_data_output.enable.next = true;
			}
		}
	}
}
pub const MEMORY_SIZE: usize = 1024;

// very basic
#[derive(LogicBlock)]
pub struct Memory{
	clock		: Signal<In, Clock>,
	registers	: Box<[DFF<Word>; MEMORY_SIZE]>
}
impl Logic for Memory{
	fn update(&mut self) {
		for  i in 0..MEMORY_SIZE {
			let reg = &mut self.registers[i];
			reg.clock.next = self.clock.val();
			reg.d.next = reg.q.val();
		}
	}
}

impl Memory{
	pub fn get_io(&mut self) -> MemoryIo{
		for i in 0..MEMORY_SIZE {
		}
		todo!()
	}
}



#[derive(LogicState)]
pub enum MemoryIoCommand {
	Nop,
	Write,
	Read,
}
#[derive(LogicBlock)]
pub struct MemoryIo {
	command 			: Signal<In		, MemoryIoCommand>,
	value				: Signal<InOut	, Word>,
	address				: Signal<In		, Word>,
	buses				: Box<[TristateBuffer<Word>; MEMORY_SIZE]>,
}

impl Logic for MemoryIo {
	fn update(&mut self) {
		for i in 0..MEMORY_SIZE {
			self.buses
		}
		match self.command.val(){
			MemoryIoCommand::Nop   => {}
			MemoryIoCommand::Write => {
				self.buses[self.address.val().to_i32() as usize].next = self.value.val();
			}
			MemoryIoCommand::Read  => {
				self.value.next = self.buses[self.address.val().to_i32() as usize].val();
			}
		}
	}
}









// maybe ill care about this later. This is a proof of concept after all

//
// #[derive(Clone, Copy, LogicBlock)]
// pub struct MemoryCommander {
//
// }
//
// pub struct Memory{
// 	cmd_queue	: SDRAMFIFO<>
// 	inner		: SDRAMCommand
// 	// i have no idea what to write here lmao
// }