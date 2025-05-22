pub mod core;
pub mod config;

pub use core::*;
pub use config::*;


use rust_hdl::prelude::*;
#[derive(LogicBlock)]
pub struct AluBank{
    alus       : Box<[AluCore; ALU_COUNT]>,
    alu_configs: AluConfigBank,
}

impl Logic for AluBank {
    fn update(&mut self) {
    }
}
impl AluBank {
   pub fn new(clock: Signal<In, Clock>) -> AluBank  {
         
   } 
}