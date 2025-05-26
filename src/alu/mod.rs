pub mod core;
pub mod config;
pub use core::*;
pub use config::*;

pub type Alus = Box<[AluCore; ALU_COUNT]>;


