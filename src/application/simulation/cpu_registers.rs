use crate::memory_primitives::register_bank::ConnectionTarget;
use std::ops::Index;
use itertools::Itertools;
use crate::{memory_primitives::{register::{}, register_bank::{RegisterBank }}, word::Word};
use crate::application::draw::component_bank::ComponentBankDrawingData;
use crate::application::simulation::alu::ALU_COUNT;
use crate::application::draw::port::{PortSignalDirection, PortData, SignalType};
use crate::application::simulation::component_bank::ComponentBank;
use crate::application::simulation::cpu_registers::CpuRegisterDataReader::{Connected, Deactivated};
use crate::memory_primitives::register::Register;
use crate::tools::used_in::UsedIn;
use crate::word::{ToBool, ToWord};

pub type CpuRegisterAddress = usize;
pub const CPU_REGISTER_COUNT: CpuRegisterAddress = 64;
pub type CpuRegisterBank = ComponentBank<CpuRegister, CPU_REGISTER_COUNT>;

impl CpuRegisterBank {
    pub fn new() -> Self{
        let registers = (0..CPU_REGISTER_COUNT).into_iter().map(|address|CpuRegister::new(address))
            .collect_array().unwrap().used_in(Box::new);
        CpuRegisterBank {
           components: registers 
        }
    }
}
pub struct CpuRegister{
    pub address : CpuRegisterAddress,
    pub value   : Word,
}

pub struct CpuRegisterPortsInfo{
    pub input: PortData,
    pub output: PortData,
}

impl CpuRegister {
    pub fn new(address: CpuRegisterAddress) -> Self{
        CpuRegister{
            address,
            value   : 0
        }
    }
    pub fn ports_info(&self) -> CpuRegisterPortsInfo{
        CpuRegisterPortsInfo{
            input: PortData {
                active: true,
                signal_dir: PortSignalDirection::Input,
                signal_type: SignalType::Data,
            },
            output: PortData {
                active: true,
                signal_dir: PortSignalDirection::Output,
                signal_type: SignalType::Data,
            },
        }
    }
    pub fn write(&mut self, new_value: Word) {
        self.value = new_value;
    }
    pub fn read(&self) -> Word {
        self.value
    }
}

impl Index<CpuRegisterPortName> for CpuRegisterPortsInfo {
    type Output = PortData;

    fn index(&self, index: CpuRegisterPortName) -> &Self::Output {
        match index{
            CpuRegisterPortName::Input  => {&self.input}
            CpuRegisterPortName::Output => {&self.output}
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum CpuRegisterPortName{
    Input,
    Output,
}

impl CpuRegisterPortName{
    pub fn iter_ports()  -> impl Iterator<Item=CpuRegisterPortName>{
        [
            CpuRegisterPortName::Input,
            CpuRegisterPortName::Output,
        ]
        .into_iter()
    }
}



pub enum CpuRegisterDataReader {
    Deactivated,
    Connected {
        source: CpuRegisterAddress,
        value : Option<Word>,
    }
}

impl CpuRegisterDataReader {
    pub fn new() -> Self{
        Deactivated
    }
    pub fn deactivate(&mut self) {
        *self = Deactivated;
    }
    pub fn is_active(&self) -> bool{
        matches!(self, Connected {..})
    }
    pub fn set_connection(&mut self, target: Option<CpuRegisterAddress>){
        if let Some(target) = target {
            *self = Connected {
                source: target,
                value: None,
            };
        } else {
            *self = Deactivated;
        }
    }
    pub fn read(&self) -> Option<Word> {
        if let Connected{ source:_, value} = self
        && let Some(val) = value
        {
            Some(*val)
        }  else {
            None
        }
    }
    pub fn get_read_request(&mut self) -> Option<CpuRegisterReadRequest> {
        if let Connected{ source:source, value} = self{
            Some(CpuRegisterReadRequest{
                source: *source,
                target: value,
            })
        } else {
            None
        }
    }
}

pub enum CpuRegisterDataWriter{
    Deactivated,
    Connected{
        target: CpuRegisterAddress,
        value: Option<Word>
    }
}

impl CpuRegisterDataWriter{
    pub fn new() -> Self{
        Self::Deactivated 
    }
    pub fn deactivate(&mut self, ){
        *self = CpuRegisterDataWriter::Deactivated;
    }
    pub fn is_active(&self) -> bool{
        matches!(self,  CpuRegisterDataWriter::Connected{..})
    }
    pub fn  set_connection(&mut self, target: Option<CpuRegisterAddress>){
        if let Some(target) = target {
            *self = Self::Connected {
                target,
                value: None,
            };
        } else {
            *self = Self::Deactivated;
        }

    }
    pub fn write(&mut self, value: Word) {
        if let CpuRegisterDataWriter::Connected{
            target,
            value: inner_value
        } = self{
            *inner_value = Some(value)
        } else {
        }
    }
    pub fn get_write_request(&self) -> Option<CpuRegisterWriteRequest>{
        if let  CpuRegisterDataWriter::Connected {
                target,
                value: inner_value,
            } = self
        && let Some(val) = inner_value
        {
            Some(CpuRegisterWriteRequest{
                target: *target,
                value : *val,
            })
        } else {
            None
        }
    }
}
pub struct CpuRegisterWriteRequest{
    target       : CpuRegisterAddress,
    value        : Word,
}
impl CpuRegisterWriteRequest {
    pub fn satisfy(&self, register_bank: &mut CpuRegisterBank) {
         register_bank.components[self.target].write(self.value);
    }
}
pub struct CpuRegisterReadRequest<'a>{
    source : CpuRegisterAddress,
    target : &'a mut Option<Word>
}
impl CpuRegisterReadRequest<'_>{
    pub fn satisfy(&mut self, register_bank: &CpuRegisterBank){
        *self.target = Some( register_bank.components[self.source].value)
    }
}

pub struct CpuRegisterActReader{
    inner     : CpuRegisterDataReader,
}

impl CpuRegisterActReader{

    pub fn new() -> Self{
        Self{inner: CpuRegisterDataReader::new()} 
    }
    pub fn deactivate(&mut self, ){
        self.inner.deactivate();
    }
    pub fn is_active(&self) -> bool{
        self.inner.is_active()
    }
    pub fn  set_connection(&mut self, target: Option<CpuRegisterAddress>){
        self.inner.set_connection(target);
    }

    pub fn get_read_request<'a>(&'a mut self) ->  Option<CpuRegisterReadRequest<'a>>{
       self.inner.get_read_request() 
    }
    pub fn read(&self) -> Option<bool>{
        self.inner.read().map(|val| val.to_bool())
    }
}



pub struct CpuRegisterActWriter {
    inner       : CpuRegisterDataWriter,
}

impl CpuRegisterActWriter {
    pub fn new() -> Self{
        Self {
            inner   : CpuRegisterDataWriter::Deactivated
        }
    }
    pub fn set_connection(&mut self, target: Option<CpuRegisterAddress>){
        self.inner.set_connection(target);
    }
    pub fn get_write_request(&self) -> Option<CpuRegisterWriteRequest>{
        self.inner.get_write_request()
    }
    pub fn deactivate(&mut self){
        self.inner.deactivate();
    }
    pub fn is_active(&self) -> bool{
        self.inner.is_active()
    }
    pub fn write(&mut self,  value: bool) {
        self.inner.write(value.to_word())
    }
}
