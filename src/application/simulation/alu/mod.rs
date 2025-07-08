pub mod core;
pub mod config;

use std::array;
pub use core::*;
pub use config::*;
use crate::{Step };
use crate::application::simulation::component_bank::ComponentBank;
use crate::application::simulation::cpu_registers::CpuRegisterBank;
use crate::application::simulation::main_memory::MainMemory;

pub type Alus = ComponentBank<AluCore, ALU_COUNT>;

impl Alus {
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




