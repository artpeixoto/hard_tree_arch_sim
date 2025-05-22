use rust_hdl::prelude::*;
use crate::{memory_primitives::{register::{RegisterReader, RegisterWriter}, register_bank::{RegisterBank, RegisterBankReader, RegisterBankWriter}}, word::Word};
use crate::memory_primitives::register_bank::RegisterBankRw;

pub const CPU_REGISTERS_ADDR_SIZE		: usize = clog2(CPU_REGISTERS_COUNT);
pub const CPU_REGISTERS_COUNT             : usize = 64;

pub type CpuRegistersAddress   = Bits<CPU_REGISTERS_ADDR_SIZE>;
pub type CpuRegisterBank       = RegisterBank<Word        , CPU_REGISTERS_ADDR_SIZE, CPU_REGISTERS_COUNT>;
pub type CpuRegisterReader 	   = RegisterReader<Word>;
pub type CpuRegisterBankReader = RegisterBankReader<Word  , CPU_REGISTERS_ADDR_SIZE, CPU_REGISTERS_COUNT>;
pub type CpuRegisterWriter 	   = RegisterWriter<Word>;
pub type CpuRegisterBankWriter = RegisterBankWriter<Word  , CPU_REGISTERS_ADDR_SIZE, CPU_REGISTERS_COUNT>;
pub type CpuRegisterBankRw     = RegisterBankRw<Word  , CPU_REGISTERS_ADDR_SIZE, CPU_REGISTERS_COUNT>;

#[derive(LogicBlock)]
pub struct CpuActivationReader {
    pub register_index  : Signal<In, CpuRegistersAddress>,
    pub value           : Signal<Out, Bit>,
    inner               : CpuRegisterBankReader
}
impl CpuActivationReader {
    pub fn new(reader: CpuRegisterBankReader) -> Self {
        Self {
            register_index: Default::default(),
            value: Default::default(),
            inner: reader
        }
    }
}

impl Logic for CpuActivationReader {

	#[hdl_gen]
    fn update(&mut self) {
        self.inner.address.next = self.register_index.val();
        self.value.next = self.inner.value.val().into();
    }
}


#[derive(LogicBlock)]
pub struct CpuActivationWriter {
    pub value			: Signal<In , Bit>,
    pub register_index	: Signal<In , CpuRegistersAddress>,
    inner               : CpuRegisterBankWriter,
}
impl CpuActivationWriter{
    pub fn new(writer: CpuRegisterBankWriter) -> Self {
        Self {
            register_index: Default::default(),
            value: Default::default(),
            inner: writer
        }
    }
}
impl Logic for CpuActivationWriter {
    fn update(&mut self) {
        self.inner.value.next = self.value.val().into();
        self.inner.write_enable.next = self.value.val();
        self.inner.address.next = self.register_index.val();
    }
}