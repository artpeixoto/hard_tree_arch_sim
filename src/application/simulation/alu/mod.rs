pub mod core;
pub mod op;

use std::array;
pub use core::*;
pub use op::*;
use crate::{Step };
use crate::application::simulation::component_bank::ComponentBank;
use crate::application::simulation::cpu_registers::CpuRegisterBank;
use crate::application::simulation::main_memory::MainMemory;

pub type AluBank = ComponentBank<AluCore, ALU_COUNT>;
pub type AluAddress = usize;
pub const ALU_COUNT: usize = 32;

impl AluBank {
    pub fn new(
        main_memory: &mut MainMemory,
    ) -> Self{

        Self{
            components: Box::new(array::from_fn(|i|
                AluCore::new(
                    i,
                    main_memory,
                )
            ))
        }
    }
}




