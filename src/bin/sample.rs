use macroquad::color::{BLUE, WHITE};
use macroquad::math::{u16vec2, vec2, Rect};
use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;
use palette::blend::Blend;
use palette::{Mix, WithAlpha};
use palette::named::{GREY, TEAL};
use palette::rgb::Rgba;
use strucc::application::draw::instruction_memory::draw_instruction_memory;
use strucc::application::simulation::alu::{ AluOperation};
use strucc::application::direction::Direction;
use strucc::application::draw::alu::{AluDrawingData, AlusDrawingData};
use strucc::application::draw::color::{ToMacroquadColorExt, ToPaletteColorExt};
use strucc::application::draw::cpu_register::{draw_cpu_register, CpuRegisterDrawingInfo};
use strucc::application::draw::path::{draw_path, PATH_COLORS};
use strucc::application::draw::grid_to_screen::{draw_path_grid, GridToScreenMapper};
use strucc::application::draw::port::{draw_port, PortSignalDirection, PortDrawingData, PortGridData, PortColorIndex, SignalType, PortData};
use strucc::application::grid::{* };
use strucc::application::draw::text::{draw_text_line_normal, draw_title};
use strucc::application::draw::text::draw_text_line_tiny;
use strucc::application::grid::blocked_point::BlockedPoints;
use strucc::application::grid::cpu_register::CpuRegisterGridInfo;
use strucc::application::grid::grid_limits::GridLimits;
use strucc::application::grid::path::{find_path_a_star, Paths};
use strucc::application::grid::pos::{grid_pos, grid_size};
use strucc::application::draw::pos::{pos, size};
use strucc::application::grid::component::DrawableComponent;
use strucc::application::grid::rect::grid_rect;
use strucc::application::simulation::cpu::Cpu;
use strucc::application::simulation::cpu_registers::CpuRegister;
use strucc::application::simulation::instruction::Instruction;
use strucc::memory_primitives::register::Register;
fn main(){
    macroquad::Window::new("sample", amain());
}
async fn amain() {
    let path_grid_size = GridLimits::new(u16vec2(180, 100));
    let background_color = {
        let white = WHITE.to_palette_color().into_linear();
        let black = BLACK.to_palette_color().into_linear();
        white.mix(black, 0.1).to_macroquad_color()
    };

    let mut screen_size = screen_size();
    let mut grid_to_screen_mapper = GridToScreenMapper::new(&path_grid_size, Rect::new(20.0, 20.0, screen_size.0 - 40.0, screen_size.1 - 40.0));

    let register_info = CpuRegister{address: 69, value: 420, };
    let register_grid_info = CpuRegisterGridInfo::new(
        grid_pos(90, 50),
    );

    let register_drawing_info = CpuRegisterDrawingInfo{
        size: size(60, 30)
    };

    let cpu = Cpu::new(
        vec![
            Instruction::SetAluConfig {
                alu_addr: 2,
                alu_config: AluOperation::Add {
                    activation_input: 2 ,
                    data_input_1: 0,
                    data_input_0: 1,
                    data_output_0: 3,
                    flags_output: None,
                    activation_output: None,
                }
            };
            128
        ]);

    let alu_pos =
            grid_pos(90, 60);

    let alu_drawing_info =
        AluDrawingData {
            full_size:size(50, 60),
            header_height: 12
        };

    let mut blocked = BlockedPoints(Default::default());


    blocked.block_rect(grid_rect(grid_pos(0, 10), grid_size(170, 20)));
    blocked.block_rect(grid_rect(grid_pos(10, 70), grid_size(170, 20)));

    let mut paths = Paths::new();
    let new_path =
        find_path_a_star(
            &grid_pos(0, 3),
            &grid_pos(179, 92),
            &paths,
            &blocked,
            &path_grid_size,
        )
        .unwrap();
    
    paths.push(
        new_path
    );


    let new_path =
        find_path_a_star(
            &grid_pos(1, 3),
            &grid_pos(178, 92),
            &paths,
            &blocked,
            &path_grid_size,
        )
        .unwrap();

    paths.push(
        new_path
    );

    let new_path =
        find_path_a_star(
            &grid_pos(2, 3),
            &grid_pos(177, 92),
            &paths,
            &blocked,
            &path_grid_size,
        )
        .unwrap();

    paths.push(
        new_path
    );

    loop{
        screen_size = miniquad::window::screen_size();

        grid_to_screen_mapper.update_transform(
            &path_grid_size,
            Rect::new(10.0,10.0, screen_size.0 - 20.0, screen_size.1 - 20.0)
        );


        clear_background(background_color);

        let port_grid_info = PortGridData {
            direction   : Direction::Right,
            position    : grid_pos(179, 92),
        };

        let port_drawing_data = PortDrawingData {
            base        : 6,
            line_len    : 4,
            arrow_height: 8,
            line_width  : 1,
            color_defn: Box::new(|color| -> Color{
                match color{
                    PortColorIndex::Deactivated => {macroquad::color::GRAY},
                    PortColorIndex::Active(SignalType::Data, PortSignalDirection::Input) => {BLUE}
                    PortColorIndex::Active(SignalType::Data, PortSignalDirection::Output) => {RED}
                    PortColorIndex::Active(SignalType::Activation, PortSignalDirection::Input) => {GREEN}
                    PortColorIndex::Active(SignalType::Activation, PortSignalDirection::Output) => {YELLOW}
                    PortColorIndex::Active(SignalType::Setup, PortSignalDirection::Input) => {VIOLET}
                    PortColorIndex::Active(SignalType::Setup, PortSignalDirection::Output) => {BROWN}
                }
            }),
        };

        draw_instruction_memory(
            &cpu.instruction_memory,
            &pos(4, 4),
            &pos(400, screen_size.1 as i32 - 40),
            0
        );

        let port_info = {
            PortData {
                active: true,
                signal_dir: PortSignalDirection::Input,
                signal_type: SignalType::Data,
            }
        };


        draw_port(
            &port_info,
            &port_grid_info,
            &port_drawing_data,
            &grid_to_screen_mapper,
        );

        draw_path_grid(
            &&grid_to_screen_mapper,
            &path_grid_size,
            &blocked,
            // &paths
        );

        for (path, path_color) in paths.iter().zip(PATH_COLORS.iter()){
            draw_path(
                path, 
                path_color, 
                &&grid_to_screen_mapper
            )
        }


        draw_text_line_tiny(
            "Hello, bitch. Go kys",
            pos(
                5,
                14
            ),
            1,
            BLACK
        );

        draw_text_line_tiny(
            "Hello, bitch. Go kys",
            pos(5, 22),
            1,
            RED
        );


        draw_text_line_normal("Hello, bitch. Go kys", pos(5, 32), 1, BLACK);
        draw_text_line_normal("Hello, bitch. Go kys", pos(5, 42), 1, BLACK);
        draw_title("Sup, Bitch", pos(5, 60),1,  BLACK );

        draw_cpu_register(&register_info, &register_grid_info, &register_drawing_info, &port_drawing_data, &&grid_to_screen_mapper);

        let alu = &cpu.alus.components[0];

        let alu_grid_data = alu.calculate_grid_data(
            alu_pos,
            &alu_drawing_info,
            &port_drawing_data,
            &grid_to_screen_mapper,
        );

        cpu.alus.components[0].draw(
            &alu_grid_data,
            &alu_drawing_info,
            &port_drawing_data,
            &grid_to_screen_mapper
        );

        let alus_drawing_data = AlusDrawingData::default();

        let alus_grid_data = cpu.alus.calculate_grid_data(
            grid_pos(10,30),
            &alus_drawing_data,
            &port_drawing_data,
            &grid_to_screen_mapper
        );

        cpu.alus.draw(
            &alus_grid_data,
            &alus_drawing_data,
            &port_drawing_data,
            &grid_to_screen_mapper
        );

        next_frame().await
    }
}
