use std::ops::Index;
use itertools::Itertools;
use crate::application::simulation::alu::{AluPortName, AluPortsDefns};
use crate::application::direction::Direction;
use crate::application::draw::port::{PortDefns, PortGridDefns, PortSignalDirection};
use crate::application::grid::alu::AluPortName::{DataIn0, DataIn1};
use crate::application::grid::blocked_point::BlockedPoints;
use crate::application::grid::component::{PortDataContainer, SimpleComponentGridDefns};
use crate::application::grid::pos::{grid_pos, GridPos, GridDist};
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AluPortsGridDefns {
    pub data_in_0       : PortGridDefns,
    pub data_in_1       : PortGridDefns,
    pub activation_in   : PortGridDefns,

    pub data_out_0      : PortGridDefns,
    pub data_out_1      : PortGridDefns,
    pub activation_out  : PortGridDefns,

    // pub setup_in     : PortGridInfo,
}

pub type AluGridDefns =
    SimpleComponentGridDefns<
        AluPortName,
        AluPortsDefns,
        AluPortsGridDefns
    >;

impl PortDataContainer<AluPortName, PortGridDefns> for AluPortsGridDefns {
    fn get_for_port(&self, port_name: &AluPortName) -> &PortGridDefns {
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