use crate::application::draw::alu::AluBankDrawingDefns;
use crate::application::draw::cpu_register::{CpuRegisterBankDrawingDefns, CpuRegisterDrawingDefn};
use crate::application::draw::grid_to_screen::GridToScreenMapper;
use crate::application::draw::instruction_memory::InstructionMemoryDrawingDefns;
use crate::application::draw::port::PortDrawingDefns;

pub struct CpuDrawingDefns{
    pub port                    : PortDrawingDefns,
    pub register_bank           : CpuRegisterBankDrawingDefns,
    pub alu_bank                : AluBankDrawingDefns,
    pub instruction_memory      : InstructionMemoryDrawingDefns,
}