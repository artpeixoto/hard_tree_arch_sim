use super::{AluConfigBank, AluConfigReader};
use crate::cpu_registers::CpuRegisterBank;
use crate::main_memory::MainMemory;
use crate::{
    cpu_registers::{
        CpuActivationReader, CpuActivationWriter, CpuRegisterBankReader, CpuRegisterBankWriter,
    },
    main_memory::MainMemoryRw,
    memory_primitives::register::{RegisterReader, RegisterRwCommand},
    word::ToWord,
};
use rust_hdl::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug, Copy, LogicState)]
pub enum AluOperation {
    NoOp,
    Eq,
    Mov,

    Not,
    And,
    Or,
    Xor,
    ShiftLeft,
    ShiftRight,
    SelectPart,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Neg,

    ReadFromMem,
    WriteToMem,
}
impl Into<AluOperation> for Bits<{ AluOperation::BITS }> {
    fn into(self) -> AluOperation {
        self.to_u8().into()
    }
}
impl Into<AluOperation> for u8 {
    fn into(self) -> AluOperation {
        use AluOperation::*;
        match self {
            0 => NoOp,
            1 => Eq,
            2 => Mov,
            3 => Not,
            4 => And,
            5 => Or,
            6 => Xor,
            7 => ShiftLeft,
            8 => ShiftRight,
            9 => Add,
            10 => Sub,
            11 => Mul,
            12 => Div,
            13 => Rem,
            14 => Neg,
            15 => ReadFromMem,
            16 => WriteToMem,
            _ => panic!(),
        }
    }
}
pub const ALU_OP_SIZE: usize = AluOperation::BITS;

#[derive(LogicBlock)]
pub struct AluCore {
    config_reader: AluConfigReader,
    clock: Signal<In, Clock>,

    main_memory: MainMemoryRw,

    data_input_0: CpuRegisterBankReader,
    data_input_1: CpuRegisterBankReader,
    activation_input: CpuActivationReader,

    main_data_output: CpuRegisterBankWriter,
    aux_data_output: CpuRegisterBankWriter,
    activation_output: CpuActivationWriter,
}

impl AluCore {
    pub fn new(
        index: usize,
        clock: Signal<In, Clock>,
        main_memory: &mut MainMemory,
        cpu_register_bank: &mut CpuRegisterBank,
        alu_config_bank: &mut AluConfigBank,
    ) -> AluCore {
        AluCore {
            config_reader: alu_config_bank.get_reader(index),
            clock,
            main_memory: main_memory.get_reader_writer(),
            data_input_0: cpu_register_bank.get_reader(),
            data_input_1: cpu_register_bank.get_reader(),
            activation_input: CpuActivationReader::new(cpu_register_bank.get_reader()),
            main_data_output: cpu_register_bank.get_writer(),
            aux_data_output: cpu_register_bank.get_writer(),
            activation_output: CpuActivationWriter::new(cpu_register_bank.get_writer()),
        }
    }
}
impl Logic for AluCore {
    // #[hdl_gen]
    fn update(&mut self) {
        // reset everything
        self.main_data_output.write_enable.next = false;
        self.aux_data_output.write_enable.next = false;
        self.activation_output.value.next = false;
        self.main_memory.command.next = RegisterRwCommand::Read;

        self.activation_input.register_index.next =
            self.config_reader.value.val().execution_signal_input;

        self.activation_output.register_index.next =
            self.config_reader.value.val().execution_signal_output;

        self.data_input_0.address.next = self.config_reader.value.val().data_input_0;

        self.data_input_1.address.next = self.config_reader.value.val().data_input_1;

        self.main_data_output.address.next = self.config_reader.value.val().main_data_output;

        self.aux_data_output.address.next = self.config_reader.value.val().aux_data_output;

        if (self.clock.val().clk && self.config_reader.value.val().operation != AluOperation::NoOp)
        {
            self.main_data_output.write_enable.next = true;
            match self.config_reader.value.val().operation {
                AluOperation::Eq => {
                    self.main_data_output.value.next =
                        (self.data_input_0.value.val() == self.data_input_1.value.val()).into()
                }
                AluOperation::Mov => {
                    self.main_data_output.value.next = self.data_input_0.value.val();
                }
                AluOperation::Not => {
                    self.main_data_output.value.next = !self.data_input_0.value.val();
                }
                AluOperation::And => {
                    self.main_data_output.value.next =
                        self.data_input_0.value.val() & self.data_input_1.value.val();
                }
                AluOperation::Or => {
                    self.main_data_output.value.next =
                        self.data_input_0.value.val() | self.data_input_1.value.val();
                }
                AluOperation::Xor => {
                    self.main_data_output.value.next =
                        self.data_input_0.value.val() ^ self.data_input_1.value.val();
                }
                AluOperation::ShiftLeft => {
                    self.main_data_output.value.next = (self.data_input_0.value.val().inner
                        << self.data_input_1.value.val().inner)
                        .into();
                }
                AluOperation::ShiftRight => {
                    self.main_data_output.value.next = (self.data_input_0.value.val().inner
                        >> self.data_input_1.value.val().inner)
                        .into();
                }
                AluOperation::SelectPart => {
                    todo!();
                    // self.main_data_output.value.next =
                    // 	self.data_input_0.out.val().get_bits::<5>();
                    self.activation_output.value.next = true;
                }
                AluOperation::Add => {
                    self.main_data_output.value.next =
                        self.data_input_0.value.val() + self.data_input_1.value.val();
                }
                AluOperation::Sub => {
                    self.main_data_output.value.next =
                        self.data_input_0.value.val() - self.data_input_1.value.val();
                }
                AluOperation::Mul => {
                    self.main_data_output.value.next = (self.data_input_0.value.val().to_i32()
                        * self.data_input_1.value.val().to_i32())
                    .into();

                    self.aux_data_output.value.next =
                        (
                            (
                                ( self.data_input_0.value.val().to_i32() as i64
                                    * self.data_input_1.value.val().to_i32() as i64
                                )
                                >> 32
                            ) as i32
                        )
						.to_word() //not pretty at all, but it is necessary due to the hdl_gen macro.
                    ;
                }
                AluOperation::Div => {
                    self.aux_data_output.write_enable.next =
                        self.config_reader.value.val().aux_output_activated;

                    if self.data_input_1.value.val().to_i32() == 0 {
                        self.aux_data_output.value.next = 1.to_word();
                    } else {
                        self.aux_data_output.value.next = 0.to_word();
                        self.main_data_output.value.next = (self.data_input_0.value.val().to_i32()
                            / self.data_input_1.value.val().to_i32())
                        .to_word();
                    }
                }
                AluOperation::Rem => {
                    self.aux_data_output.write_enable.next =
                        self.config_reader.value.val().aux_output_activated;

                    if self.data_input_1.value.val() == 0_i32.into() {
                        self.aux_data_output.value.next = 1_i32.into();
                    } else {
                        self.aux_data_output.value.next = 0_i32.into();
                        self.main_data_output.value.next = (self.data_input_0.value.val().to_i32()
                            % self.data_input_1.value.val().to_i32())
                        .into();
                    }
                }
                AluOperation::Neg => {
                    self.main_data_output.value.next = -self.data_input_0.value.val()
                }
                AluOperation::ReadFromMem => {
                    self.main_memory.address.next = self.data_input_0.value.val().into();
                    self.main_memory.command.next = RegisterRwCommand::Read;
                    self.main_data_output.value.next = self.main_memory.value.val();
                }
                AluOperation::WriteToMem => {
                    self.main_memory.address.next = self.data_input_0.value.val().into();
                    self.main_memory.value.next = self.data_input_1.value.val();
                    self.main_memory.command.next = RegisterRwCommand::Write;
                }
                AluOperation::NoOp => {
                    unreachable!()
                }
            }
            self.main_data_output.write_enable.next = true;
        }
    }
}
