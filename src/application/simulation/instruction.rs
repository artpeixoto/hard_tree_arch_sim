use crate::application::simulation::cpu_registers::CpuRegisterAddress;
use crate::application::simulation::alu::AluOperation;
use crate::word::Word;

pub const CONTROLLER_INSTRUCTION_SIZE   		: usize = 64;


#[derive( PartialEq, Copy, Clone, Debug,Eq, Default)]
pub enum Instruction {
    SetAluConfig{
        alu_config	: AluOperation, 
        alu_addr	: usize,
    },
    
    SetLiteral{
        literal			: Word,
        register: CpuRegisterAddress,
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
        // relative        : bool,
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




