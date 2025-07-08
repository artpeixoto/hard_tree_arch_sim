use std::{array, iter};
use itertools::Itertools;
use crate::application::simulation::alu::{AluCore, AluOperation, Alus, ALU_COUNT};
use crate::application::simulation::controller::{AluConfigWriter, Controller};
use crate::application::simulation::cpu_registers::{CpuRegisterBank, CPU_REGISTER_COUNT};
use crate::application::simulation::instruction::Instruction;
use crate::application::simulation::instruction_reader::{InstructionMemory, InstructionReader};
use crate::application::simulation::main_memory::MainMemory;
use crate::{Step};

pub struct Cpu{
    pub alus                : Alus,
    pub registers           : CpuRegisterBank,
    pub controller          : Controller,
    pub instruction_memory  : InstructionMemory,
    pub main_memory         : MainMemory,
}


impl Cpu {
    pub fn new(program: Vec<Instruction>) -> Self {
        let mut main_memory = MainMemory::new();

        let registers = CpuRegisterBank::new();

        let instruction_memory = InstructionMemory::new(program.clone());

        let alus = Alus::new(&mut  main_memory);

        let controller = Controller::new(
            &instruction_memory,
        );
        
        Cpu{
            main_memory,
            alus,
            registers ,
            controller,
            instruction_memory, 
        }
    }
     
    pub fn update(&mut self, ){
        if let Some(mut controller_read_req) = self.controller.cpu_registers_reader
            .get_read_request() {
            controller_read_req.satisfy( &self.registers)
        }


        if let Some(mut controller_pc_read_req) =
            self.controller
            .instruction_reader
            .program_counter_reader
            .get_read_request()
        {
            controller_pc_read_req.satisfy(&self.registers);
        }

        // let mut alu_reads = Vec::new();

        match &self.controller.alu_config_writer{
            AluConfigWriter::Deactivated => {}
            AluConfigWriter::WritingToSingle { target, op } => {
                self.alus.components[*target].set_new_operation(op.clone());
            }
            AluConfigWriter::WritingToAll { op } => {
                for alu in self.alus.components.iter_mut() {
                    alu.set_new_operation(op.clone());
                }
            }
        }

        // give alus the requested data
        for alu in self.alus.components.iter_mut(){
            let mut reqs = alu.collect_read_requests();
            for (port, req) in &mut reqs{
                req.satisfy(&self.registers);
            }

            // alu_reads.push(reqs);
        }

        self.controller.execute();

        for alu in self.alus.components.iter_mut(){
            alu.execute();
        }

        for alu in self.alus.components.iter_mut(){
            let mut reqs = alu.collect_write_requests();
            for (port, req) in &mut reqs{
                req.satisfy(&mut self.registers);
            }
            // alu_reads.push(reqs);
        }

        if let Some(write_req) = self.controller.cpu_registers_writer.get_write_request(){
            write_req.satisfy(&mut self.registers);
        }

        if let Some(write_pc_req) = self.controller.instruction_reader.program_counter_writer.get_write_request(){
            write_pc_req.satisfy(&mut self.registers);
        }
    }
}
