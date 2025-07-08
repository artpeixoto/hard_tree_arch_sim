use std::hash::Hash;
use std::marker::PhantomData;
use crate::application::draw::grid_to_screen::GridToScreenMapper;
use crate::application::draw::port::{PortDrawingData, PortGridData, PortData};
use crate::application::grid::blocked_point::BlockedPoints;
use crate::application::grid::pos::GridPos;
use crate::application::grid::rect::GridRect;

pub trait DrawableComponent {
    type DrawingData;
    type PortName: PortName;
    type PortDataContainer
        : PortDataContainer<Self::PortName, PortData>;
    type PortGridDataContainer
        : PortDataContainer<Self::PortName, PortGridData>;

    fn calculate_grid_data(
        &self,
        grid_pos         : GridPos,
        drawing_info     : &Self::DrawingData,
        port_drawing_info: &PortDrawingData,
        grid_to_screen   : &GridToScreenMapper,
    ) -> ComponentGridData<
        Self::PortName,
        Self::PortDataContainer,
        Self::PortGridDataContainer
    >;

    fn draw(
        &self,
        grid_data: &ComponentGridData<
            Self::PortName,
            Self::PortDataContainer,
            Self::PortGridDataContainer
        >,
        drawing_data     : &Self::DrawingData,
        port_drawing_info: &PortDrawingData,
        grid_to_screen   : &GridToScreenMapper,
    );
}

pub trait PortName: Sized + Hash + Eq + Clone {
    fn all_port_names() -> Vec<Self>;

    /// Name must be 5 characters or fewer
    fn small_name(&self) -> &str;
}

pub struct ComponentGridData<N, P, G,>
where
    N: PortName,
    P: PortDataContainer<N, PortData>,
    G: PortDataContainer<N, PortGridData>,
{
    pub grid_rect       : GridRect,
    pub blocked_points  : BlockedPoints,
    pub ports_data      : P,
    pub ports_grid_data : G,
    pub _phantom        : PhantomData<N>,
}

pub trait PortDataContainer< N: PortName, P, >{
    fn get_for_port(&self, port_name: &N) -> &P;
}
