use std::ops::Index;
use crate::application::direction::Direction;
use crate::application::draw::port::{PortGridData, PortSignalDirection};
use crate::application::grid::blocked_point::BlockedPoints;
use crate::application::grid::pos::{GridPos, GridDist, grid_pos, GridSize, grid_size};
use crate::application::simulation::cpu_registers::CpuRegisterPortName;

pub struct CpuRegisterGridInfo{
    grid_pos        : GridPos,
}

impl CpuRegisterGridInfo {
    pub fn grid_pos(&self) -> &GridPos{
        &self.grid_pos
    }
    
    pub fn new(pos: GridPos) -> Self {
        Self {
            grid_pos : pos,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Eq, )]
pub struct CpuRegisterPortsGridInfo{
    pub input : PortGridData,
    pub output: PortGridData,
}

impl Index<CpuRegisterPortName> for CpuRegisterPortsGridInfo {
    type Output = PortGridData; 

    fn index(&self, index: CpuRegisterPortName) -> &Self::Output {
        match index{
            CpuRegisterPortName::Input => {&self.input}
            CpuRegisterPortName::Output => {&self.output}
        }
    }
}