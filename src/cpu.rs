use rust_hdl::prelude::*;
use crate::alu::AluBank;
use crate::controller::Controller;
use crate::cpu_registers::CpuRegisterBank;
use crate::main_memory::MainMemory;

pub struct Cpu{
    alus        : AluBank,
    registers   : CpuRegisterBank,
    controller  : Controller,
    clock       : Signal<In, Clock>,
    main_memory : MainMemory,
}
impl Cpu{
    pub fn new(clock: Signal<In, Clock>) -> Self{
         
    }
}
