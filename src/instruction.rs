use std::{ iter::repeat_n};
use crate::{alu:: AluOperation, cpu_registers::CpuRegisterAddress, };
use crate::word::Word;

pub const CONTROLLER_INSTRUCTION_SIZE   		: usize = 64;

#[derive( PartialEq, Copy, Clone, Debug,Eq, Default)]
pub enum Instruction {
    SetAluConfig{
        alu_addr	: usize,
        alu_config	: AluOperation
    },

    SetLiteral{
        register_index  : CpuRegisterAddress,
        literal			: Word,
    },

    // PopStack{
    //     register_index	: CpuRegistersAddress,
    // },
    //
    // PushToStack{
    //     register_index	: CpuRegistersAddress,
    // },

    WaitForActivationSignal{
        register_index  : CpuRegisterAddress
    },

    Jump{
        relative        : bool,
        addr            : Word
    },
    
    ResetAll,

    #[default]
    NoOp,
}






#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HorizontalDir{
    Left,
    Right
}



