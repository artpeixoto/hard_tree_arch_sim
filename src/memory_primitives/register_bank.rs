use bevy::prelude::*;
use std::ops::{Deref};
use itertools::Itertools;
use crate::{ClockTransition, Step};
use crate::cpu_registers::CpuRegisterAddress;
use super::register::{Register, RegisterReader, RegisterWriter};

#[derive(Component)]
pub struct RegisterBank<Data, const COUNT: usize> {
    registers       : Box<[Register<Data>; COUNT]>,
}

impl<Data, const COUNT: usize> ClockTransition for RegisterBank<Data, COUNT>{
    fn step(&mut self, _step: &Step) {
        self.registers.iter().for_each(|reg| {reg.transition()})
    }
}
impl<Data, const COUNT: usize> RegisterBank<Data, { COUNT }> {
    pub fn new(initial_vals: impl Iterator<Item=Data>) -> Self {
        let registers  =
            Box::new(
                initial_vals
                .map(|data| Register::new(data))
                .collect_array()
                .unwrap()
            );

        Self { registers }
    }
}

#[derive(Component)]
pub struct RegisterBankReader<Data, const COUNT: usize> {
    readers : Box<[RegisterReader<Data>; COUNT]>,
}

impl<Data,const COUNT: usize> RegisterBankReader<Data,COUNT>{
   pub fn read<'a>(&'a self, addr: CpuRegisterAddress) -> impl Deref<Target=Data> + 'a{
        self.readers[addr].read()
   }
}

pub struct RegisterBankWriter<Data, const COUNT: usize>{
    writers : Box<[RegisterWriter<Data>; COUNT]>,
}

impl<Data: Clone, const COUNT: usize> RegisterBankWriter<Data, COUNT> {
    pub fn write(&mut self, value: Data, address: usize) {
        self.writers[address].write(value);
    }
    // pub fn write_many(&mut self, values: Vec<Data>, start_address: usize ) {
    //     values
    //     .into_iter()
    //     .enumerate()
    //     .for_each(|(ix, val)| {
    //         self.write(val, start_address + ix);
    //     });
    // }
}

impl <Data,  const COUNT: usize>
    RegisterBank<Data, COUNT>
where
{
    pub fn get_reader(&self) -> RegisterBankReader<Data, COUNT> {
        let a = self.registers.iter().map(|reg| reg.get_reader()).collect_array().unwrap();
        RegisterBankReader{
            readers: Box::new(a)
        }
    }
    pub fn get_writer(&self) -> RegisterBankWriter<Data, COUNT> {
        let a = self.registers.iter().map(|reg| reg.get_writer()).collect_array().unwrap();
        RegisterBankWriter{
            writers: Box::new(a)
        }
    }

    pub fn get_specific_reader(&mut self, addr: usize) -> RegisterReader<Data>{
        self.registers[addr].get_reader() 
    }
    pub fn get_specific_writer(&mut self, addr: usize) -> RegisterWriter<Data>{
        self.registers[addr].get_writer()
    }
}
