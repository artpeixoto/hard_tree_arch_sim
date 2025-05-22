use std::{array, marker::PhantomData};

use rust_hdl::prelude::*;

use super::register::{Register, RegisterReader, RegisterRwCommand, RegisterWriter};

#[derive(LogicBlock)]
pub struct RegisterBank<Data: Synth, const ADDR_SIZE: usize, const COUNT: usize> {
    clock		    : Signal<In, Clock>,
    registers       : Box<[Register<Data>; COUNT]>,
    _phantom        : PhantomData<[(); ADDR_SIZE]>,
}

impl<Data: Synth, const ADDR_SIZE: usize, const COUNT: usize> RegisterBank<Data, ADDR_SIZE, COUNT> {
    pub fn new(clock: Signal<In, Clock>) -> Self{
        let registers    = 
            Box::new(array::from_fn(|_| Register::new(clock.clone())));
        Self{
            clock,
            registers,
            _phantom: Default::default(),
        }
    }
}

impl<Data: Synth, const ADDR_LEN: usize, const COUNT: usize>
    Logic for RegisterBank<Data, ADDR_LEN, COUNT>
{
    // #[hdl_gen]
    fn update(&mut self) {
        for  i in 0..COUNT {
            self.registers[i].clock.next = self.clock.val();
        }
    }
}

#[derive(LogicBlock)]
pub struct RegisterBankReader<Data: Synth, const ADDR_LEN: usize, const COUNT: usize> {
    pub address         : Signal<In, Bits<ADDR_LEN>>,
    pub value			: Signal<Out, Data>,
    inner_readers		: Box<[RegisterReader<Data>; COUNT]>,
}

impl<Data, const ADDR_LEN: usize, const COUNT: usize>
    Logic for RegisterBankReader<Data, ADDR_LEN, COUNT>
where
    Data    : Synth,
{
    #[hdl_gen]
    fn update(&mut self) {
        self.value.next = self.inner_readers[self.address.val().index()].value.val();
    }
}

impl <Data, const ADDR_LEN: usize, const COUNT: usize>
    RegisterBank<Data, ADDR_LEN, COUNT>
where
    Data: Synth,
{
    pub fn get_reader(&mut self) -> RegisterBankReader<Data, ADDR_LEN, COUNT> {
        let inner_readers
            : Box<[_;COUNT]>  //
            = Box::new(array::from_fn(|i| self.registers[i].get_reader())); //this may go wrong


        RegisterBankReader {
            inner_readers,
            address: Default::default(),
            value  : Default::default(),
        }
    }
    pub fn get_writer(&mut self) -> RegisterBankWriter<Data, ADDR_LEN, COUNT> {
        let inner_writers
            : Box<[_;COUNT]>  //
            = Box::new(array::from_fn(|i| self.registers[i].get_writer())); //this may go wrong

        RegisterBankWriter{
            address: Default::default(),
            value: Default::default(),
            write_enable: Default::default(),
            inners: inner_writers
        }
    }

    pub fn get_specific_reader(&mut self, addr: usize) -> RegisterReader<Data>{
        self.registers[addr].get_reader() 
    }
    pub fn get_specific_writer(&mut self, addr: usize) -> RegisterWriter<Data>{
        self.registers[addr].get_writer()
    }
}


#[derive(LogicBlock)]
pub struct RegisterBankWriter<Data: Synth, const ADDR_LEN: usize, const COUNT: usize> {
    pub address         : Signal<In, Bits<ADDR_LEN>>,
    pub write_enable    : Signal<In, Bit>,
    pub value			: Signal<In, Data>,
    inners              : Box<[RegisterWriter<Data>; COUNT]>
}

impl <Data, const COUNT: usize, const ADDR_LEN: usize>
Logic for RegisterBankWriter<Data, ADDR_LEN, COUNT>
where
    Data: Synth,
{
    fn update(&mut self) {
        for i in  0..COUNT {
            self.inners[i].write_enable.next = false;
        }

        if self.write_enable.val(){
            self.inners[self.address.val().index()].write_value.next  = self.value.val();
            self.inners[self.address.val().index()].write_enable.next = true;
        }
    }
}
impl<Data: Synth, const ADDR_LEN: usize, const COUNT: usize> 
    RegisterBank<Data, ADDR_LEN, COUNT> 
{
    
}



#[derive(LogicBlock)]
pub struct RegisterBankRw<Data: Synth, const ADDR_SIZE: usize, const SIZE: usize> {
    pub command 		: Signal<In		, RegisterRwCommand>,
    pub value		    : Signal<InOut	, Data>,
    pub address			: Signal<In		, Bits<ADDR_SIZE>>,
    inner_writer        : RegisterBankWriter<Data, ADDR_SIZE, SIZE>,
    inner_reader        : RegisterBankReader<Data, ADDR_SIZE, SIZE>,
}

impl<Data: Synth, const ADDR_SIZE: usize, const SIZE: usize> 
Logic for RegisterBankRw<Data, ADDR_SIZE, SIZE> 
{
    #[hdl_gen]
    fn update(&mut self) {
        self.inner_writer.write_enable.next = false;
        self.inner_reader.address.next = self.address.val();
        self.inner_writer.address.next = self.address.val();

        match self.command.val(){
            RegisterRwCommand::Write => {
                self.inner_writer.write_enable.next = true;
                self.inner_writer.value.next = self.value.val();
            }
            RegisterRwCommand::Read => {
                self.value.next = self.inner_reader.value.val();
            }
        }
    }
}

impl<Data: Synth, const ADDR_SIZE: usize, const SIZE: usize> RegisterBank<Data, ADDR_SIZE, SIZE>
{
	pub fn get_reader_writer(&mut self) -> RegisterBankRw<Data, ADDR_SIZE, SIZE>{
        let reader = self.get_reader();
        let writer = self.get_writer();
        RegisterBankRw{
            command : Default::default(),
            value   : Default::default(),
            address : Default::default(),
            inner_reader: reader,
            inner_writer: writer,
        }
    }
}