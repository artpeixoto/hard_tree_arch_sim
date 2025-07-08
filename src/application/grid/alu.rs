use std::ops::Index;
use itertools::Itertools;
use crate::application::simulation::alu::AluPortName;
use crate::application::direction::Direction;
use crate::application::draw::port::{PortData, PortGridData, PortSignalDirection};
use crate::application::grid::alu::AluPortName::{DataIn0, DataIn1};
use crate::application::grid::blocked_point::BlockedPoints;
use crate::application::grid::component::PortDataContainer;
use crate::application::grid::pos::{grid_pos, GridPos, GridDist};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AluPortsGridData {
    pub data_in_0       : PortGridData,
    pub data_in_1       : PortGridData,
    pub activation_in   : PortGridData,

    pub data_out_0      : PortGridData,
    pub data_out_1      : PortGridData,
    pub activation_out  : PortGridData,

    // pub setup_in     : PortGridInfo,
}
impl PortDataContainer<AluPortName, PortGridData> for AluPortsGridData {
    fn get_for_port(&self, port_name: &AluPortName) -> &PortGridData {
        use AluPortName::*;
        match port_name{
            DataIn0 => {&self.data_in_0}
            DataIn1 => {&self.data_in_1}
            ActivationIn => {&self.activation_in}
            DataOut0 => {&self.data_out_0}
            DataOut1 => {&self.data_out_1}
            ActivationOut => {&self.activation_out}
        }
    }
}