use bevy::prelude::Component;
use bevy::render::extract_component::ExtractComponent;
use crate::{memory_primitives::{register::{RegisterReader, RegisterWriter}, register_bank::{RegisterBank, RegisterBankReader, RegisterBankWriter}}, word::Word};

pub const CPU_REGISTERS_COUNT             : usize = 64;

pub type CpuRegisterAddress = usize;
pub type CpuRegisterBank       = RegisterBank<Word, CPU_REGISTERS_COUNT>;
pub type CpuRegisterReader 	   = RegisterReader<Word>;
pub type CpuRegisterBankReader = RegisterBankReader<Word, CPU_REGISTERS_COUNT>;
pub type CpuRegisterWriter 	   = RegisterWriter<Word>;
pub type CpuRegisterBankWriter = RegisterBankWriter<Word, CPU_REGISTERS_COUNT>;

#[derive(Component)]
pub struct CpuActivationReader {
    pub inner       : CpuRegisterBankReader
}

impl CpuActivationReader {
    pub fn new(reader: CpuRegisterBankReader) -> Self {
        Self {
            inner   : reader
        }
    }
    pub fn read<'a>(&self, addr: usize) -> bool{
        self.inner.read(addr) != 0
    }
}

pub struct CpuActivationWriter {
    inner       : CpuRegisterBankWriter,
}

impl CpuActivationWriter {
    pub fn new(writer: CpuRegisterBankWriter) -> Self{
        Self {
            inner   : writer
        }
    }
    pub fn write(&mut self,  value: bool, address: CpuRegisterAddress,) {
        self.inner.write(
            match value{
                true    => {0xFF_FF_FF_FF}
                false   => {0}
            },
            address
        )
    }
}