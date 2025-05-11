use std::collections::hash_map::Keys;
use std::sync::LazyLock;

use rust_hdl::prelude::{Bit, Bits, LiteralType, Signal, ToBits};

pub const ALU_INDEX_SIZE     : usize = 5;
pub const ALU_COUNT			 : usize = 2_usize.pow(ALU_INDEX_SIZE as u32);
pub const REGISTER_INDEX_SIZE: usize = 5;
pub const REGISTER_COUNT	 : usize = 2_usize.pow(REGISTER_INDEX_SIZE as u32);
pub const INSTRUCTION_SIZE   : usize = 64;
pub const ARCHITECTURE_SIZE  : usize = 32;
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
	ResetAll{

	},
}
impl ToBits for ControllerCmd{
	fn to_bits<const N: usize>(self) -> Bits<N> {
		todo!()
	}
}

pub type Word 				= Bits<ARCHITECTURE_SIZE>;
pub type AluIndex			= Bits<ALU_INDEX_SIZE>;
pub type AluIndexPort<Dir>	= Signal<Dir, Bits<ALU_INDEX_SIZE>>;
pub type RegisterIndex  		= Bits<REGISTER_INDEX_SIZE>;
pub type RegisterIndexSignal<Dir>  	= Signal<Dir, Bits<REGISTER_INDEX_SIZE>>;

pub static PROGRAM_COUNTER_REGISTER			: LazyLock<RegisterIndex> = LazyLock::new(||Bits::from(31));
pub static STACK_POINTER_REGISTER  			: LazyLock<RegisterIndex> = LazyLock::new(||Bits::from(30)); 
pub static RETURN_POINTER_REGISTER  		: LazyLock<RegisterIndex> = LazyLock::new(||Bits::from(29)); 


pub struct AluConfig{
	execution_signal_input	: RegisterIndex,
	execution_signal_output	: RegisterIndex,
	operation				: AluOperation
}

impl ToBits for AluConfig{
	fn to_bits<const N: usize>(self) -> Bits<N> {
		todo!()
	}
}

pub const ALU_CONFIG_SIGNAL_SIZE: usize = 48;

pub type AluConfigSignal<Dir> = Signal<Dir, Bits<ALU_CONFIG_SIGNAL_SIZE>>;
pub enum IntOperation{
	Add{
		first_operand_input		: RegisterIndex,
		second_operand_input	: RegisterIndex,
		output					: RegisterIndex,
	},
	Sub{
		first_operand_input		: RegisterIndex,
		second_operand_input	: RegisterIndex,
		output					: RegisterIndex,
	},
	Mul{
		first_operand_input		: RegisterIndex,
		second_operand_input	: RegisterIndex,
		output					: RegisterIndex,
	},
	Div{
		first_operand_input		: RegisterIndex,
		second_operand_input	: RegisterIndex,
		output					: RegisterIndex,
	},
	Rem{
		first_operand_input		: RegisterIndex,
		second_operand_input	: RegisterIndex,
		output					: RegisterIndex,
	},
	Neg{
		operand_input			: RegisterIndex,
		output					: RegisterIndex,
	}
}
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
	}
}

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
pub enum AluOperation{
	MemIo(MemIoOperation),
	Logic(LogicOperation),
	IntOperation(IntOperation),
	Eq{
		first_operand_input		: RegisterIndex,
		second_operand_input	: RegisterIndex,
		output					: RegisterIndex,
	},
}

pub struct Controller{

}

pub struct Alu{
}
pub struct AluConfigMem{
	config_signal	:
	current_config:
}