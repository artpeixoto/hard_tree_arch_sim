use rust_hdl::prelude::*;
use crate::{alu::{AluAddr, AluConfig}, cpu_registers::CpuRegistersAddress, word::Word};

pub struct Controller{
    
}
pub const CONTROLLER_INSTRUCTION_SIZE   		: usize = 64;

#[derive( PartialEq, Copy, Clone, Debug,Eq,Default)]
pub enum ControllerInstruction{
    SetAluConfig{
        alu			: AluAddr,
        alu_config	: AluConfig
    },

    SetLiteral{
        register_index  : CpuRegistersAddress,
        literal			: Word,
    },

    PopStack{
        register_index	: CpuRegistersAddress,
    },

    PushToStack{
        register_index	: CpuRegistersAddress,
    },

    WaitForActivationSignal{
        register_index  : CpuRegistersAddress
    },

    Jump{
        relative        : bool,
        addr            : Word
    },

    ResetAll,

    #[default]
    NoOp,
}


impl Into<ControllerInstruction> for Bits<64>{
	fn into(self) -> ControllerInstruction {
		todo!()
	}
}

impl Synth for ControllerInstruction{
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