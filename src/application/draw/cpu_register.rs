use itertools::Itertools;
use macroquad::color::{BLACK, BLUE, DARKBROWN, RED, WHITE};
use macroquad::math::Vec2;
use macroquad::prelude::draw_rectangle;
use wgpu::hal::DynCommandEncoder;
use crate::application::direction::Direction;
use crate::application::draw::component_bank::{ComponentBankDrawingData, ComponentBankPortDataContainer, ComponentBankPortName};
use crate::application::draw::cursor::RectCursor;
use crate::application::draw::grid_to_screen::GridToScreenMapper;
use crate::application::draw::port::{draw_port, PortDrawingData, PortGridData};
use crate::application::draw::text::{draw_text_line_normal, draw_text_line_tiny, normal_font, tiny_font};
use crate::application::draw::text::title::draw_title;
use crate::application::grid::cpu_register::{CpuRegisterGridInfo, CpuRegisterPortsGridInfo};
use crate::application::grid::pos::{grid_dist, grid_pos, grid_size, GridSize};
use crate::application::draw::pos::{dist, pos, Pos, Size};
use crate::application::grid::blocked_point::BlockedPoints;
use crate::application::grid::rect::{grid_rect, GridRect};
use crate::application::simulation::cpu_registers::{CpuRegister, CpuRegisterBank, CpuRegisterPortName, CPU_REGISTER_COUNT};

pub struct CpuRegisterDrawingInfo{
    pub size: Size,
}
pub fn calculate_grid_info(
    register            : &CpuRegister,
    reg_grid_info       : &CpuRegisterGridInfo,
    reg_drawing_info    : &CpuRegisterDrawingInfo,

    port_drawing_info   : &PortDrawingData,
    grid_to_screen_info : &GridToScreenMapper,
) -> ( GridRect, BlockedPoints, CpuRegisterPortsGridInfo,) {
   let reg_grid_rect = {
       let reg_grid_pos = *reg_grid_info.grid_pos();
       let reg_grid_size = grid_to_screen_info.screen_to_grid_size(reg_drawing_info.size);
       grid_rect(reg_grid_pos, reg_grid_size)
   };

    let reg_ports_grid_info  = {
        let y       = reg_grid_info.grid_pos().y - 1;
        let x_right = reg_grid_info.grid_pos().x + reg_grid_rect.top_left.x - 1;
        CpuRegisterPortsGridInfo{
            input : PortGridData {
                position    : grid_pos(x_right, y),
                direction   : Direction::Up,
            },
            output: PortGridData {
                position : grid_pos(x_right - 2, y),
                direction: Direction::Up,
            },
        }
    };
    let blocked_points = BlockedPoints::new_from_blocked_inner_rect(reg_grid_rect.clone());
    (
        reg_grid_rect,
        blocked_points,
        reg_ports_grid_info,
    )

}
pub fn draw_cpu_register(
    register            : &CpuRegister,
    reg_grid_info       : &CpuRegisterGridInfo,
    reg_drawing_info    : &CpuRegisterDrawingInfo,

    port_drawing_info   : &PortDrawingData,
    grid_to_screen_info : &GridToScreenMapper,
){
    let (
        reg_grid_rect,
        blocked_points,
        reg_ports_grid_info,
    ) = calculate_grid_info(
        register,
        reg_grid_info,
        reg_drawing_info,
        port_drawing_info,
        grid_to_screen_info,
    );

    let cursor =
        grid_to_screen_info
        .get_cursor_for_region(
            *reg_grid_info.grid_pos() ,
            reg_grid_rect.size
        );


    let full_port_height = port_drawing_info.full_len();

    { // draw ports

        let reg_ports_info = register.ports_info();

        for port_name in CpuRegisterPortName::iter_ports(){
            let port_info  = &reg_ports_info[port_name];
            let port_grid_info = &reg_ports_grid_info[port_name];
            draw_port(
                port_info,
                port_grid_info,
                port_drawing_info,
                grid_to_screen_info,
            )
        }
    }

    { // draw inner square
        let mut cursor = cursor.after_going( dist(2, full_port_height));

        // draw base rectangle
        draw_rectangle(
            cursor.top_left().x as f32,
            cursor.top_left().y as f32,
            cursor.remaining_size().x as f32,
            cursor.remaining_size().y as f32,
            DARKBROWN
        );

        // draw INDEX
        const INDEX_FONT_SIZE: i32 = tiny_font::DIMS.full_height() as i32;

        draw_rectangle(
            cursor.top_left().x as f32,
            (cursor.top_left().y - INDEX_FONT_SIZE - 2) as f32,
            (cursor.remaining_size().x as f32 * 0.3), // ????
            (INDEX_FONT_SIZE + 2) as f32,
            BLACK.with_alpha(0.3)
        );

        draw_text_line_tiny(
            &format!("{:X}", register.address),
            pos(
                cursor.top_left().x  + 2,
                cursor.top_left().y  - INDEX_FONT_SIZE
            ),
            1,
            BLACK
        );

        cursor.pad(2, 2);

        { // DRAW VALUE

            let cursor = cursor.after_going(cursor.remaining_size().with_x(0)/2);

            draw_text_line_normal(
                &format!("{:X}", register.value),
                (cursor.top_left() - dist(0, normal_font::DIMS.full_height() as i32 / 2) ),
                1,
                WHITE
            );
        }
    }



}
pub type CpuRegisterBankDrawingData = ComponentBankDrawingData<CpuRegisterDrawingInfo>;
pub type CpuRegisterBankPortName = ComponentBankPortName<CpuRegisterPortName,  CPU_REGISTER_COUNT>;
