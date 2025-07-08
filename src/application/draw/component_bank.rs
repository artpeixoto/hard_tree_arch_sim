use std::marker::PhantomData;
use itertools::Itertools;
use wgpu::naga::FastHashMap;
use crate::application::grid::component::{ComponentGridData, DrawableComponent, PortDataContainer, PortName};
use crate::application::draw::alu::AluDrawingData;
use crate::application::draw::grid_to_screen::GridToScreenMapper;
use crate::application::draw::port::{PortData, PortDrawingData, PortGridData};
use crate::application::draw::pos::Size;
use crate::application::grid::blocked_point::BlockedPoints;
use crate::application::grid::pos::{grid_pos, grid_size, GridPos};
use crate::application::grid::rect::grid_rect;
use crate::application::simulation::alu::{Alus, ALU_COUNT};
use crate::application::simulation::component_bank::ComponentBank;

#[derive(Clone, PartialEq, Eq)]
pub struct ComponentBankDrawingData<CompDrawingData> {
    pub size: Size,
    pub row_count: usize,
    pub drawing_data: CompDrawingData,
}

impl<CompDrawingData: Default> Default for ComponentBankDrawingData<CompDrawingData> {
    fn default() -> Self {
        Self {
            size: Size::new(1920 / 2, 1080 / 2),
            row_count: 4,
            drawing_data: CompDrawingData::default(),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug, Hash)]
pub struct ComponentBankPortName<CompPortName, const COMP_COUNT: usize>{
    pub comp_addr: usize ,
    pub port_name: CompPortName
}

impl<CompPortName: PortName, const COMP_COUNT: usize>
    PortName
    for ComponentBankPortName<CompPortName, COMP_COUNT>
{
    fn all_port_names() -> Vec<Self> {
        (0..COMP_COUNT)
        .into_iter()
        .map(|address| {
            CompPortName
            ::all_port_names()
            .into_iter()
            .map(move |port_name|{ComponentBankPortName {
                comp_addr: address,
                port_name: port_name.clone()}
            })
        })
        .flatten()
        .collect_vec()
    }

    fn small_name(&self) -> &str {
        self.port_name.small_name()
    }
}

pub struct ComponentBankPortDataContainer<CompPortName: PortName, Data, const COMP_COUNT: usize>{
    pub elements: FastHashMap<ComponentBankPortName<CompPortName, COMP_COUNT>, Data>
}
impl<CompPortName, Data, const  COMP_COUNT: usize>
    PortDataContainer<ComponentBankPortName<CompPortName, COMP_COUNT>, Data>
    for ComponentBankPortDataContainer<CompPortName, Data, COMP_COUNT>
where
    CompPortName: PortName
{
    fn get_for_port(&self, port_name: &ComponentBankPortName<CompPortName, COMP_COUNT>) -> &Data {
        self.elements.get(port_name).unwrap()
    }
}

impl<
    Comp,
    const COMP_COUNT: usize
>
    DrawableComponent
    for ComponentBank<Comp, COMP_COUNT>
where
    Comp: DrawableComponent,
    Comp::PortName: PortName,
{
    type DrawingData = ComponentBankDrawingData<Comp::DrawingData>;
    type PortName = ComponentBankPortName<Comp::PortName, COMP_COUNT>;
    type PortDataContainer = ComponentBankPortDataContainer<Comp::PortName, PortData, COMP_COUNT>;
    type PortGridDataContainer = ComponentBankPortDataContainer<Comp::PortName, PortGridData, COMP_COUNT>;

    fn calculate_grid_data(
        &self,
        grid_top_left: GridPos,
        drawing_data: &Self::DrawingData,
        port_drawing_data: &PortDrawingData,
        grid_to_screen_mapper: &GridToScreenMapper
    ) -> ComponentGridData<Self::PortName, Self::PortDataContainer, Self::PortGridDataContainer> {
        let col_count = ALU_COUNT / drawing_data.row_count;
        let full_grid_size = grid_to_screen_mapper.screen_to_grid_size(drawing_data.size);

        let alu_grid_size =
            self
                .components[0]
                .calculate_grid_data(
                    grid_top_left,
                    &drawing_data.drawing_data,
                    port_drawing_data,
                    grid_to_screen_mapper
                )
                .grid_rect
                .size;

        let remaining_grid_size = full_grid_size - grid_size(
            (alu_grid_size.x * col_count as i16),
            (alu_grid_size.y * drawing_data.row_count as i16),
        );

        let grid_spacing = grid_size(
            remaining_grid_size.x / (col_count as i16 + 1),
            remaining_grid_size.y / (drawing_data.row_count as i16 + 1),
        );
        let mut port_data = FastHashMap::default();
        let mut port_grid_data = FastHashMap::default();
        let mut blocked_points = BlockedPoints::new();

        for ix in 0..col_count {
            let grid_x = grid_top_left.x + grid_spacing.x + (ix as i16 * (grid_spacing.x +
                alu_grid_size.x));
            for iy in 0..drawing_data.row_count {
                let grid_y = grid_top_left.y + grid_spacing.y + (iy as i16 * (grid_spacing.y +
                    alu_grid_size.y));
                let alu_grid_pos = grid_pos(grid_x, grid_y);

                let alu_addr = iy * drawing_data.row_count + ix;

                let alu = &self.components[alu_addr];

                let alu_grid_data = alu.calculate_grid_data(
                    alu_grid_pos,
                    &drawing_data.drawing_data,
                    &port_drawing_data,
                    &grid_to_screen_mapper
                );

                blocked_points.add_from(alu_grid_data.blocked_points);

                for inner_port_name in Comp::PortName::all_port_names() {
                    let cur_port_name: Self::PortName  = ComponentBankPortName{
                        comp_addr: alu_addr,
                        port_name: inner_port_name.clone(),
                    };
                    port_data.insert(
                        cur_port_name.clone(),
                        alu_grid_data.ports_data.get_for_port(&inner_port_name).clone()
                    );
                    port_grid_data.insert(
                        cur_port_name.clone(),
                        alu_grid_data.ports_grid_data.get_for_port(&inner_port_name).clone()
                    );
                }
            }
        }

        let port_data = ComponentBankPortDataContainer{
            elements: port_data
        };
        let port_grid_data = ComponentBankPortDataContainer{
            elements: port_grid_data
        };


        ComponentGridData{
            grid_rect: grid_rect(grid_top_left, full_grid_size),
            blocked_points,
            ports_data: port_data,
            ports_grid_data: port_grid_data,
            _phantom: PhantomData,
        }
    }

    fn draw(&self, grid_data: &ComponentGridData<Self::PortName, Self::PortDataContainer, Self::PortGridDataContainer>, drawing_data: &Self::DrawingData, port_drawing_data: &PortDrawingData, grid_to_screen_mapper: &GridToScreenMapper) {
        let grid_top_left = grid_data.grid_rect.top_left;
        let col_count = ALU_COUNT / drawing_data.row_count;
        let full_grid_size = grid_to_screen_mapper.screen_to_grid_size(drawing_data.size);

        let alu_grid_size =
            self
                .components[0]
                .calculate_grid_data(
                    grid_top_left,
                    &drawing_data.drawing_data,
                    port_drawing_data,
                    grid_to_screen_mapper
                )
                .grid_rect
                .size;

        let remaining_grid_size = full_grid_size - grid_size(
            (alu_grid_size.x * col_count as i16),
            (alu_grid_size.y * drawing_data.row_count as i16),
        );

        let grid_spacing = grid_size(
            remaining_grid_size.x / (col_count as i16 + 1),
            remaining_grid_size.y / (drawing_data.row_count as i16 + 1),
        );

        for ix in 0..col_count {
            let grid_x = grid_top_left.x + grid_spacing.x + (ix as i16 * (grid_spacing.x +
                alu_grid_size.x));
            for iy in 0..drawing_data.row_count {
                let grid_y = grid_top_left.y + grid_spacing.y + (iy as i16 * (grid_spacing.y +
                    alu_grid_size.y));
                let alu_grid_pos = grid_pos(grid_x, grid_y);

                let alu_addr = iy * drawing_data.row_count + ix;

                let alu = &self.components[alu_addr];

                let alu_grid_data = alu.calculate_grid_data(
                    alu_grid_pos,
                    &drawing_data.drawing_data,
                    &port_drawing_data,
                    &grid_to_screen_mapper
                );

                alu.draw(
                    &alu_grid_data,
                    &drawing_data.drawing_data,
                    &port_drawing_data,
                    &grid_to_screen_mapper
                );
            }
        }
    }
}