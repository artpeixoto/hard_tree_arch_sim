use std::collections::hash_map::Keys;
use std::ops::Sub;
use std::sync::LazyLock;
use rust_hdl::prelude::*;

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
pub type AluIndexPort<Dir>			= Signal<Dir, Bits<ALU_INDEX_SIGNAL_SIZE>>;
pub type RegisterIndex  			= Bits<REGISTER_INDEX_SIGNAL_SIZE>;
pub type RegisterIndexSignal<Dir>  	= Signal<Dir, Bits<REGISTER_INDEX_SIGNAL_SIZE>>;

pub static PROGRAM_COUNTER_REGISTER			: LazyLock<RegisterIndex> = LazyLock::new(||Bits::from(63));
pub static STACK_POINTER_REGISTER  			: LazyLock<RegisterIndex> = LazyLock::new(||Bits::from(62));
pub static RETURN_POINTER_REGISTER  		: LazyLock<RegisterIndex> = LazyLock::new(||Bits::from(61));


#[derive(Clone, PartialEq, Eq, Debug, Copy, Default)]
pub struct AluConfig{
	execution_signal_input	: RegisterIndex,
	execution_signal_output	: RegisterIndex,
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

pub type AluConfigSignal<Dir> = Signal<Dir, Bits<ALU_CONFIG_SIGNAL_SIZE>>;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, LogicStruct)]
pub struct AddOperation{
	first_operand_input		: RegisterIndex,
	second_operand_input	: RegisterIndex,
	output					: RegisterIndex,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IntOperation{
	Add{
		first_operand_input		: RegisterIndex,
		second_operand_input	: RegisterIndex,
		output					: RegisterIndex,
		flags_output			: RegisterIndex,
	},
	Sub{
		first_operand_input		: RegisterIndex,
		second_operand_input	: RegisterIndex,
		output					: RegisterIndex,
		flags_output			: RegisterIndex,
	}
	// Mul{
	// 	first_operand_input		: RegisterIndex,
	// 	second_operand_input	: RegisterIndex,
	// 	output					: RegisterIndex,
	// },
	// Div{
	// 	first_operand_input		: RegisterIndex,
	// 	second_operand_input	: RegisterIndex,
	// 	output					: RegisterIndex,
	// },
	// Rem{
	// 	first_operand_input		: RegisterIndex,
	// 	second_operand_input	: RegisterIndex,
	// 	output					: RegisterIndex,
	// },
	// Neg{
	// 	operand_input			: RegisterIndex,
	// 	output					: RegisterIndex,
	// }
}
#[derive(Clone, PartialEq, Eq, Debug, Copy)]
pub enum LogicOperation {
	Not{
		input					: RegisterIndex,
		output					: RegisterIndex,
	},
	And{
		first_operand_input		: RegisterIndex,
		second_operand_input	: RegisterIndex,
		output					: RegisterIndex,
	},
	Or{
		first_operand_input		: RegisterIndex,
		second_operand_input	: RegisterIndex,
		output					: RegisterIndex,
	},
	Shift{
		count_input				: RegisterIndex,
		output					: RegisterIndex,
	},
	Any{
		operand_input			: RegisterIndex,
		output					: RegisterIndex,
	},
	SelectBit{
		operand_input			: RegisterIndex,
		bit_index				: Bits<{clog2(DATA_SIZE)}>, //32 bits
		output					: RegisterIndex,
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MemIoOperation{
	ReadFromMem {
		memory_location_input   : RegisterIndex,
		data_output				: RegisterIndex,
	},
	WriteToMem {
		memory_location_input	: RegisterIndex,
		data_input				: RegisterIndex,
	},
}

#[derive(Clone, PartialEq, Eq, Debug, Copy, Default)]
pub enum AluOperation{
	#[default]
	NoOp,
	MemIo(MemIoOperation),
	Logic(LogicOperation),
	Int(IntOperation),
	Eq{
		first_operand_input		: RegisterIndex,
		second_operand_input	: RegisterIndex,
		output					: RegisterIndex,
	}
	,
}

pub struct Controller{

}

pub struct Alu{
	set_config_input_signal	: Signal<In, Bit>,
	config_input_signal		: AluConfigSignal<In>,
	config					: DFF<AluConfig>,

	clock					: Signal<In, Clock>,

	data_input_0			: Signal<In, DataWord>,
	data_input_1			: Signal<In, DataWord>,

	activation_input		: Signal<In, Bit>,

	data_output				: Signal<In, DataWord>,
	flags_output			: Signal<In, Bits<12>>,
	activation_output		: Signal<Out, Bit>,
}

impl
