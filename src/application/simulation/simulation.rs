use std::{array, iter};
use std::ops::Not;
use itertools::Itertools;
use crate::application::simulation::alu::{AluCore, AluOperation, AluBank, ALU_COUNT};
use crate::application::simulation::controller::{AluConfigWriter, Controller};
use crate::application::simulation::cpu_registers::{CpuRegisterBank, REGISTER_COUNT};
use crate::application::simulation::instruction::Instruction;
use crate::application::simulation::instruction_reader::{InstructionMemory, InstructionReader};
use crate::application::simulation::main_memory::MainMemory;
use crate::{Step};
use crate::word::Word;

pub struct Cpu {
    pub alu_bank: AluBank,
    pub register_bank: CpuRegisterBank,
    pub controller          : Controller,
    pub instruction_memory  : InstructionMemory,
    pub main_memory         : MainMemory,
}
impl Cpu {

    #[must_use]
    pub fn execute(&mut self) -> bool {
        if let Some(mut controller_read_req) =
            self.controller.cpu_registers_reader.get_read_request() {
            controller_read_req.satisfy( &self.register_bank)
        }


        if let Some(mut controller_pc_read_req) =
            self.controller
            .instruction_reader
            .program_counter_reader
            .get_read_request()
        {
            controller_pc_read_req.satisfy(&self.register_bank);
        }

        self.controller
            .alu_config_writer
            .configure_alus(&mut self.alu_bank);


        // give alus the requested data
        for alu in self.alu_bank.components.iter_mut(){
            let mut reqs = alu.collect_read_requests();
            for (port, req) in &mut reqs{
                req.satisfy(&self.register_bank);
            }
            // alu_reads.push(reqs);
        }

        if self.controller.execute().not(){
            return false;
        };


        for alu in self.alu_bank.components.iter_mut(){
            alu.execute();
        }

        for alu in self.alu_bank.components.iter_mut(){
            let mut reqs = alu.collect_write_requests();
            for (port, req) in &mut reqs{
                req.satisfy(&mut self.register_bank);
            }
            // alu_reads.push(reqs);
        }

        if let Some(write_req) = self.controller.cpu_registers_writer.get_write_request(){
            write_req.satisfy(&mut self.register_bank);
        }

        if let Some(write_pc_req) = self.controller.instruction_reader.program_counter_writer.get_write_request(){
            write_pc_req.satisfy(&mut self.register_bank);
        }

        true
    }
}
