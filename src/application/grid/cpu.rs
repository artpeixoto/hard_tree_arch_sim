use crate::application::draw::alu::AluBankGridDefns;
use crate::application::draw::cpu_register::CpuRegisterBankGridDefns;
use crate::application::draw::instruction_memory::InstructionMemoryGridDefns;
use crate::application::grid::blocked_point::BlockedPoints;
use crate::application::simulation::alu::AluBank;
use crate::application::simulation::cpu_registers::CpuRegisterBank;

pub struct CpuGridDefns{
    pub alu_bank            : AluBankGridDefns,
    pub register_bank       : CpuRegisterBankGridDefns,
    pub instruction_memory  : InstructionMemoryGridDefns,
    pub blocked_points  : BlockedPoints,
}

pub struct CpuComponentsGridDefns {

}



