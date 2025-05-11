use std::array;
use std::collections::hash_map::Keys;
use std::ops::{Not, Sub};
use std::sync::LazyLock;
use rust_hdl::prelude::*;
use rust_hdl::widgets::edge_ff::EdgeDFF;

pub const ALU_INDEX_SIGNAL_SIZE: usize = 5;
pub const ALU_COUNT			 : usize = 2_usize.pow(ALU_INDEX_SIGNAL_SIZE as u32);

pub const REGISTER_INDEX_SIGNAL_SIZE: usize = 6;
pub const REGISTER_COUNT	 : usize = 2_usize.pow(REGISTER_INDEX_SIGNAL_SIZE as u32);
pub const INSTRUCTION_SIZE   : usize = 64;

pub const DATA_SIZE: usize = 32;

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
		VerilogLiteral::from(0_u64)
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
		literal			: DataWord,
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

impl ToBits for ControllerCmd{
	fn to_bits<const N: usize>(self) -> Bits<N> {
		todo!()
	}
}

pub type DataWord 					= Bits<DATA_SIZE>;
pub type AluIndex					= Bits<ALU_INDEX_SIGNAL_SIZE>;
pub type AluIndexSignal<Dir>		= Signal<Dir, Bits<ALU_INDEX_SIGNAL_SIZE>>;
pub type RegisterIndex  			= Bits<REGISTER_INDEX_SIGNAL_SIZE>;
pub type RegisterIndexSignal<Dir>  	= Signal<Dir, Bits<REGISTER_INDEX_SIGNAL_SIZE>>;

pub static PROGRAM_COUNTER_REGISTER	: LazyLock<RegisterIndex> = LazyLock::new(||Bits::from(63));
pub static STACK_POINTER_REGISTER  	: LazyLock<RegisterIndex> = LazyLock::new(||Bits::from(62));
pub static RETURN_POINTER_REGISTER  : LazyLock<RegisterIndex> = LazyLock::new(||Bits::from(61));


#[derive(Clone, PartialEq, Eq, Debug, Copy, Default)]
pub struct AluConfig{
	execution_signal_input	: RegisterIndex,
	execution_signal_output	: RegisterIndex,

	data_input_0			: RegisterIndex,
	data_input_1			: RegisterIndex,

	main_data_output		: RegisterIndex,
	aux_data_output			: RegisterIndex,
	operation				: AluOperation
}

impl Synth for AluConfig{
	const BITS: usize = 48;

	fn descriptor() -> TypeDescriptor {
		todo!()
	}

	fn vcd(self) -> VCDValue {
		todo!()
	}

	fn verilog(self) -> VerilogLiteral {
		todo!()
	}
}

pub const ALU_CONFIG_SIGNAL_SIZE: usize = AluConfig::BITS;

pub type AluConfigSignal<Dir> = Signal<Dir, AluConfig>;

#[derive(Clone, PartialEq, Eq, Debug, Copy, Default)]
pub enum AluOperation{
	#[default]
	NoOp,

	ReadFromMem,
	WriteToMem,

	Not,
	And,
	Or,
	BitShift,
	Any,
	SelectBit,

	Add,
	Sub,
	Mul,
	Div,
	Rem,
	Neg,

	Eq,
}

pub struct Controller{

}

#[derive(LogicBlock)]
pub struct Registers {
	clock		: Signal<In, Clock>,
	registers	: [DFF<DataWord>; REGISTER_COUNT]
}
impl Logic for Registers{
	fn update(&mut self) {
		for  i in 0..REGISTER_COUNT {
			let reg = &mut self.registers[i];
			reg.clock.next = self.clock.val();
			reg.d.next = reg.q.val();
		}
	}
}

impl Registers{
	pub fn get_reader(&mut self) -> RegisterDataReader {
		let mut register_inputs =
			array::from_fn::< Signal<In, DataWord>, REGISTER_COUNT,_>(|_| Default::default());
		for i in 0..REGISTER_COUNT {
			self.registers[i].q.join(&mut register_inputs[i])
		}

		RegisterDataReader {
			registers: register_inputs,
			enable: Default::default(),
			register_index: Default::default(),
			out: Default::default(),
		}
	}
}

#[derive(LogicBlock)]
pub struct RegisterDataReader {
	register_index 	: Signal<In, RegisterIndex>,
	enable			: Signal<In, bool>,
	registers		: [Signal<In, DataWord>; REGISTER_COUNT],
	out				: Signal<Out, DataWord>,
}

impl Logic for RegisterDataReader {
	#[hdl_gen]
	fn update(&mut self) {
		if self.enable.val(){
			self.out.next = self.registers[self.register_index.val().index()].val();
		}
	}
}
#[derive(LogicBlock)]
pub struct RegisterActivationReader {
	register_index 	: Signal<In, RegisterIndex>,
	enable			: Signal<In, bool>,
	registers		: [Signal<In, DataWord>; REGISTER_COUNT],
	out				: Signal<Out, Bit>,
}
impl Logic for RegisterActivationReader {
	fn update(&mut self) {
		todo!()
	}
}

#[derive(LogicBlock)]
pub struct RegisterDataWriter{
	enable			: Signal<In, bool>,
	register_index	: Signal<In, RegisterIndex>,
	registers		: [Signal<Out, DataWord>; REGISTER_COUNT],
	input			: Signal<In, DataWord>,
}
impl Logic for RegisterDataWriter{
	#[hdl_gen]
	fn update(&mut self) {
	}
}
#[derive(LogicBlock)]
pub struct AluConfigMem{
	clock					: Signal<In, Clock>,
	alu_change_config_input	: Signal<In, bool>,
	alu_config_input		: Signal<In, AluConfig>,
	alu_config				: DFF<AluConfig>,

	output					: Signal<Out, AluConfig>,
	is_ready				: Signal<Out, bool>,
}
impl Logic for AluConfigMem{
	fn update(&mut self) {
		dff_setup!(self, clock, alu_config );

		if self.clock.pos_edge() {
			self.alu_config.d.next = self.alu_change_config_input.val();
		}
	}
	
}
#[derive(LogicBlock)]
pub struct AluCore {
	clock					: Signal<In, Clock>,

	data_input_0			: RegisterDataReader,
	data_input_1			: RegisterDataReader,
	activation_input		: RegisterActivationReader,

	main_data_output		: RegisterDataWriter,
	aux_data_output			: RegisterDataWriter,
	
	activation_output		: Signal<Out, Bit>,
}
#[derive(LogicBlock)]
pub struct RegisterActivationWriter{
}
impl Logic for  RegisterActivationWriter{
	fn update(&mut self) {
		todo!()
	}
}

impl Logic for AluCore {
	#[hdl_gen]
	fn update(&mut self) {

		// reset everything
		self.data_input_0.enable.next = false;
		self.data_input_1.enable.next = false;

		if self.alu_change_config_input.val() {
			self.alu_config.clock.next 	= self.clock.val();
			self.alu_config.d.next 		= self.alu_config_input.val();
		} else if self.activation_input.val() {

		} else {
			//in this case, nothing happens, but we should ensure everything is down.

		}
	}
}

