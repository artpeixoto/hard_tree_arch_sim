use std::{array, iter};
use crate::alu::{AluConfigBank, AluCore, AluOperation, Alus, ALU_COUNT};
use crate::controller::Controller;
use crate::cpu_registers::{CpuRegisterBank, CPU_REGISTERS_COUNT};
use crate::instruction::Instruction;
use crate::instruction_reader::{InstructionMemory, InstructionReader};
use crate::main_memory::MainMemory;

pub struct Cpu{
    pub alus        : Alus,
    pub alu_configs : AluConfigBank,
    pub registers   : CpuRegisterBank,
    pub controller  : Controller,
    pub main_memory : MainMemory,
}

impl Cpu {
    pub fn new(program: Vec<Instruction>) -> Self {
        let mut main_memory = MainMemory::new();

        let mut registers = CpuRegisterBank::new(iter::from_fn(|| Some(0)).take(CPU_REGISTERS_COUNT));

        let mut alu_configs = AluConfigBank::new(iter::from_fn(|| Some(AluOperation::NoOp)).take(ALU_COUNT));

        let instruction_memory = InstructionMemory::new(program.clone());

        let alus            = Box::new(array::from_fn(|i|
            AluCore::new(
                i,
                &mut alu_configs,
                &mut main_memory,
                &mut registers,
            )
        ));

        let controller = Controller::new(
            &mut main_memory,
            &instruction_memory,
            &mut alu_configs,
            &mut registers,
        );

        Cpu{
            alu_configs,
            main_memory,
            alus,
            registers ,
            controller
        }
    }
}

