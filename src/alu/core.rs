use std::ops::Deref;
use bevy::render::render_resource::IndexFormat;
use super::{AluConfigBank, AluConfigReader, AluOperation};
use crate::{
    cpu_registers::{
        CpuActivationReader, CpuActivationWriter, CpuRegisterBankReader, CpuRegisterBankWriter,
    },
};
use crate::cpu_registers::CpuRegisterBank;
use crate::main_memory::{MainMemory, MainMemoryIo};


pub struct AluCore {
    config              : AluConfigReader,
    
    main_memory         : MainMemoryIo,

    data_input_0        : CpuRegisterBankReader,
    data_input_1        : CpuRegisterBankReader,
    activation_input    : CpuActivationReader,

    main_data_output    : CpuRegisterBankWriter,
    aux_data_output     : CpuRegisterBankWriter,
    activation_output   : CpuActivationWriter,
}

impl AluCore{
    pub fn new(
        alu_addr        : usize, 
        configs         : &mut AluConfigBank, 
        main_memory     : &mut MainMemory, 
        cpu_registers   : &mut CpuRegisterBank
    ) -> Self{
        AluCore{
           config           : configs.get_specific_reader(alu_addr),
           main_memory      : main_memory.get_io(),
           data_input_0     : cpu_registers.get_reader(),
           data_input_1     : cpu_registers.get_reader(),
           activation_input : CpuActivationReader ::new(cpu_registers.get_reader()),
           main_data_output : cpu_registers.get_writer(),
           aux_data_output  : cpu_registers.get_writer(),
           activation_output: CpuActivationWriter::new(cpu_registers.get_writer()),
       }  
    }
    pub fn step(&mut self, ){
        let alu_config = self.config.read();
        match &*alu_config{
            AluOperation::NoOp => {}
            AluOperation::Eq { .. } => {}
            AluOperation::Mov { .. } => {}
            AluOperation::Latch { .. } => {}
            AluOperation::Not { .. } => {}
            AluOperation::And { .. } => {}
            AluOperation::Or { .. } => {}
            AluOperation::Xor { .. } => {}
            AluOperation::ShiftLeft { .. } => {}
            AluOperation::ShiftRight { .. } => {}
            AluOperation::SelectPart { .. } => {}
            AluOperation::Add { .. } => {}
            AluOperation::Sub { .. } => {}
            AluOperation::Mul { .. } => {}
            AluOperation::Div { .. } => {}
            AluOperation::Rem { .. } => {}
            AluOperation::Neg { .. } => {}
            AluOperation::ReadFromMem { .. } => {}
            AluOperation::WriteToMem { .. } => {}
        }
    }
}