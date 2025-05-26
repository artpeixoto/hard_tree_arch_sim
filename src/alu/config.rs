use crate::instruction_reader::InstructionMemory;
use crate::memory_primitives::register_bank::RegisterBankWriter;
use crate::{
    cpu_registers::CpuRegisterAddress,
    memory_primitives::{
        register::{RegisterReader, RegisterWriter},
        register_bank::RegisterBank,
    },
};

pub const ALU_COUNT: usize = 32;
#[derive(Clone, PartialEq, Eq, Debug, Copy)]
pub enum MovInput {
    Source(CpuRegisterAddress),
    SourceAddr(CpuRegisterAddress),
}

pub type AluConfigBank = RegisterBank<AluOperation, ALU_COUNT>;
pub type AluConfigBankWriter = RegisterBankWriter<AluOperation, ALU_COUNT>;
pub type AluConfigReader = RegisterReader<AluOperation>;

#[derive(Clone, PartialEq, Eq, Debug, Copy)]
pub enum AluOperation {
    NoOp,
    Eq {
        execution_signal_input: CpuRegisterAddress,
        execution_signal_output: Option<CpuRegisterAddress>,
        data_input_0: CpuRegisterAddress,
        data_input_1: CpuRegisterAddress,
        data_output: CpuRegisterAddress,
    },
    Mov {
        execution_signal_input: CpuRegisterAddress,
        address_input: MovInput,
        data_output: CpuRegisterAddress,
        execution_signal_output: Option<CpuRegisterAddress>,
    },
    Latch {
        execution_signal_input  : CpuRegisterAddress,
        data_input              : CpuRegisterAddress,
        main_data_output        : CpuRegisterAddress,
        execution_signal_output : Option<CpuRegisterAddress>,
    },
    Not {
        execution_signal_input  : CpuRegisterAddress,
        data_input              : CpuRegisterAddress,
        main_data_output        : CpuRegisterAddress,
        execution_signal_output : Option<CpuRegisterAddress>,
    },
    And {
        execution_signal_input  : CpuRegisterAddress,
        data_input_0            : CpuRegisterAddress,
        data_input_1            : CpuRegisterAddress,
        main_data_output        : CpuRegisterAddress,
        execution_signal_output : Option<CpuRegisterAddress>,
    },
    Or {
        execution_signal_input  : CpuRegisterAddress,
        data_input_0            : CpuRegisterAddress,
        data_input_1            : CpuRegisterAddress,
        main_data_output        : CpuRegisterAddress,
        execution_signal_output : Option<CpuRegisterAddress>,
    },
    Xor {
        execution_signal_input  : CpuRegisterAddress,
        data_input_1            : CpuRegisterAddress,
        data_input_0            : CpuRegisterAddress,
        main_data_output        : CpuRegisterAddress,
        execution_signal_output : Option<CpuRegisterAddress>,
    },
    ShiftLeft {
        execution_signal_input  : CpuRegisterAddress,
        value                   : CpuRegisterAddress,
        shift_count             : CpuRegisterAddress,
        main_data_output        : CpuRegisterAddress,
        execution_signal_output : Option<CpuRegisterAddress>,
    },
    ShiftRight {
        execution_signal_input: CpuRegisterAddress,
        value: CpuRegisterAddress,
        shift_count: CpuRegisterAddress,
        main_data_output: CpuRegisterAddress,
        execution_signal_output: Option<CpuRegisterAddress>,
    },
    SelectPart {
        execution_signal_input: CpuRegisterAddress,
        data_input: CpuRegisterAddress,
        selection_input: CpuRegisterAddress,
        main_data_output: CpuRegisterAddress,
        execution_signal_output: Option<CpuRegisterAddress>,
    },
    Add {
        execution_signal_input  : CpuRegisterAddress,
        data_input_1            : CpuRegisterAddress,
        data_input_0            : CpuRegisterAddress,
        main_data_output        : CpuRegisterAddress,
        flags_output            : Option<CpuRegisterAddress>,
        execution_signal_output : Option<CpuRegisterAddress>,
    },
    Sub {
        execution_signal_input: CpuRegisterAddress,
        data_input_1            : CpuRegisterAddress,
        data_input_0            : CpuRegisterAddress,
        main_data_output        : CpuRegisterAddress,
        flags_output            : Option<CpuRegisterAddress>,
        execution_signal_output : Option<CpuRegisterAddress>,
    },
    Mul {
        execution_signal_input: CpuRegisterAddress,
        data_input_1: CpuRegisterAddress,
        data_input_0: CpuRegisterAddress,
        first_word_output: CpuRegisterAddress,
        second_word_output: Option<CpuRegisterAddress>,
        execution_signal_output: Option<CpuRegisterAddress>,
    },
    Div {
        execution_signal_input: CpuRegisterAddress,
        dividend: CpuRegisterAddress,
        divisor: CpuRegisterAddress,
        main_data_output: CpuRegisterAddress,
        div_by_zero_flag_output: Option<CpuRegisterAddress>,
        execution_signal_output: Option<CpuRegisterAddress>,
    },
    Rem {
        execution_signal_input  : CpuRegisterAddress,
        dividend: CpuRegisterAddress,
        divisor: CpuRegisterAddress,
        main_data_output        : CpuRegisterAddress,
        div_by_zero_flag_output : Option<CpuRegisterAddress>,
        execution_signal_output : Option<CpuRegisterAddress>,
    },

    Neg {
        execution_signal_input: CpuRegisterAddress,
        input: CpuRegisterAddress,
        main_data_output: CpuRegisterAddress,
        execution_signal_output: Option<CpuRegisterAddress>,
    },

    ReadFromMem {
        execution_signal_input: CpuRegisterAddress,
        data_input_0: CpuRegisterAddress,
        main_data_output: CpuRegisterAddress,
        execution_signal_output: Option<CpuRegisterAddress>,
    },

    WriteToMem {
        execution_signal_input  : CpuRegisterAddress,
        address_input           : CpuRegisterAddress,
        data_input              : CpuRegisterAddress,
        execution_signal_output : Option<CpuRegisterAddress>,
    },
}
pub struct AluPortsConfig {
    pub input_0: Option<CpuRegisterAddress>,
    pub input_1: Option<CpuRegisterAddress>,
    pub activation_input: Option<CpuRegisterAddress>,
    pub main_output: Option<CpuRegisterAddress>,
    pub aux_output: Option<CpuRegisterAddress>,
    pub activation_output: Option<CpuRegisterAddress>,
}
impl AluOperation {
    pub fn get_ports_config(&self) -> AluPortsConfig {
        match self.clone() {
            AluOperation::NoOp => AluPortsConfig {
                input_0: None,
                input_1: None,
                activation_input: None,
                main_output: None,
                aux_output: None,
                activation_output: None,
            },
            AluOperation::Eq {
                execution_signal_input,
                execution_signal_output,
                data_input_0,
                data_input_1,
                data_output: main_data_output,
            } => AluPortsConfig {
                input_0: Some(data_input_0),
                input_1: Some(data_input_1),
                activation_input: Some(execution_signal_input),
                main_output: Some(main_data_output),
                aux_output: None,
                activation_output: execution_signal_output,
            },

            AluOperation::Mov {
                execution_signal_input,
                execution_signal_output,
                address_input,
                data_output: output,
            } => AluPortsConfig {
                input_0: Some(match address_input{
                    MovInput::Source(src) => src,
                    MovInput::SourceAddr(addr_src) => addr_src
                }),
                input_1: None,
                activation_input: Some(execution_signal_input),
                main_output: Some(output),
                aux_output: None,
                activation_output: execution_signal_output,
            },

            AluOperation::Latch {
                execution_signal_input,
                data_input: data_input_0,
                main_data_output,
                execution_signal_output,
            } => AluPortsConfig {
                input_0: Some(data_input_0),
                input_1: None,
                activation_input: Some(execution_signal_input),
                main_output: Some(main_data_output),
                aux_output: None,
                activation_output: execution_signal_output,
            },
            AluOperation::Not {
                execution_signal_input,
                data_input: data_input_0,
                main_data_output,
                execution_signal_output,
            } => AluPortsConfig{
                input_0: Some(data_input_0),
                input_1: None,
                activation_input: Some(execution_signal_input),
                main_output: Some(main_data_output),
                aux_output: None,
                activation_output:execution_signal_output,
            },
            AluOperation::And {
                execution_signal_input,
                data_input_1,
                data_input_0,
                main_data_output,
                execution_signal_output,
            } => AluPortsConfig {
                input_0: Some(data_input_0),
                input_1: Some(data_input_1),
                activation_input: Some(execution_signal_input),
                main_output: Some(main_data_output),
                aux_output: None,
                activation_output: execution_signal_output,
            },
            AluOperation::Or {
                execution_signal_input,
                data_input_1,
                data_input_0,
                main_data_output,
                execution_signal_output,
            } => AluPortsConfig {
                input_0: Some(data_input_0),
                input_1: Some(data_input_1),
                activation_input: Some(execution_signal_input),
                main_output: Some(main_data_output),
                aux_output: None,
                activation_output: execution_signal_output,
            },
            AluOperation::Xor {
                execution_signal_input,
                data_input_1,
                data_input_0,
                main_data_output,
                execution_signal_output,
            } => AluPortsConfig {
                input_0: Some(data_input_0),
                input_1: Some(data_input_1),
                activation_input: Some(execution_signal_input),
                main_output: Some(main_data_output),
                aux_output: None,
                activation_output: execution_signal_output,
            },
            AluOperation::ShiftLeft {
                execution_signal_input,
                value,
                shift_count,
                main_data_output,
                execution_signal_output,
            } => AluPortsConfig {
                input_0: Some(value),
                input_1: Some(shift_count),
                activation_input: Some(execution_signal_input),
                main_output: Some(main_data_output),
                aux_output: None,
                activation_output: execution_signal_output,
            },
            AluOperation::ShiftRight {
                execution_signal_input,
                value,
                shift_count,
                main_data_output,
                execution_signal_output,
            } => AluPortsConfig {
                input_0: Some(value),
                input_1: Some(shift_count),
                activation_input: Some(execution_signal_input),
                main_output: Some(main_data_output),
                aux_output: None,
                activation_output: execution_signal_output,
            },
            AluOperation::SelectPart {
                execution_signal_input,
                data_input: data_input_1,
                selection_input: data_input_0,
                main_data_output,
                execution_signal_output,
            } => AluPortsConfig {
                input_0: Some(data_input_0),
                input_1: Some(data_input_1),
                activation_input: Some(execution_signal_input),
                main_output: Some(main_data_output),
                aux_output: None,
                activation_output: execution_signal_output,
            },
            AluOperation::Add {
                execution_signal_input,
                data_input_1,
                data_input_0,
                main_data_output,
                flags_output,
                execution_signal_output,
            } => AluPortsConfig {
                input_0: Some(data_input_0),
                input_1: Some(data_input_1),
                activation_input: Some(execution_signal_input),
                main_output: Some(main_data_output),
                aux_output: flags_output,
                activation_output: execution_signal_output,
            },
            AluOperation::Sub {
                execution_signal_input,
                data_input_1,
                data_input_0,
                main_data_output,
                flags_output,
                execution_signal_output,
            } => AluPortsConfig {
                input_0: Some(data_input_0),
                input_1: Some(data_input_1),
                activation_input: Some(execution_signal_input),
                main_output: Some(main_data_output),
                aux_output : flags_output,
                activation_output: execution_signal_output,
            },
            AluOperation::Mul {
                execution_signal_input,
                data_input_1,
                data_input_0,
                first_word_output: main_data_output,
                second_word_output,
                execution_signal_output,
            } => AluPortsConfig {
                input_0: Some(data_input_0),
                input_1: Some(data_input_1),
                activation_input: Some(execution_signal_input),
                main_output: Some(main_data_output),
                aux_output: second_word_output,
                activation_output: execution_signal_output,
            },
            AluOperation::Div {
                execution_signal_input,
                divisor: data_input_1,
                dividend: data_input_0,
                main_data_output,
                div_by_zero_flag_output: div_by_zero_output,
                execution_signal_output,
            } => AluPortsConfig {
                input_0: Some(data_input_0),
                input_1: Some(data_input_1),
                activation_input: Some(execution_signal_input),
                main_output: Some(main_data_output),
                aux_output: div_by_zero_output,
                activation_output: execution_signal_output,
            },
            AluOperation::Rem {
                execution_signal_input,
                divisor: data_input_1,
                dividend: data_input_0,
                main_data_output,
                div_by_zero_flag_output: div_by_zero_output,
                execution_signal_output,
            } => AluPortsConfig {
                input_0: Some(data_input_0),
                input_1: Some(data_input_1),
                activation_input: Some(execution_signal_input),
                main_output: Some(main_data_output),
                aux_output: div_by_zero_output,
                activation_output: execution_signal_output,
            },
            AluOperation::Neg {
                execution_signal_input,
                input: data_input_0,
                main_data_output,
                execution_signal_output,
            } => AluPortsConfig {
                input_0: Some(data_input_0),
                input_1: None,
                activation_input: Some(execution_signal_input),
                main_output: Some(main_data_output),
                aux_output: None,
                activation_output: execution_signal_output,
            },
            AluOperation::ReadFromMem {
                execution_signal_input,
                data_input_0: addr_input,
                main_data_output,
                execution_signal_output,
            } => AluPortsConfig {
                input_0: Some(addr_input),
                input_1: None,
                activation_input: Some(execution_signal_input),
                main_output: Some(main_data_output),
                aux_output: None,
                activation_output: execution_signal_output,
            },
            AluOperation::WriteToMem {
                execution_signal_input,
                data_input,
                address_input,
                execution_signal_output,
            } => AluPortsConfig {
                input_0: Some(data_input),
                input_1: Some(address_input),
                activation_input: Some(execution_signal_input),
                main_output: None,
                aux_output: None,
                activation_output: execution_signal_output,
            },
        }
    }
}
