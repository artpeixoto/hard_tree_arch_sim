use std::collections::HashMap;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
use strucc::application::draw::alu::AluBankDrawingDefns;
use strucc::application::draw::cpu::CpuDrawingDefns;
use strucc::application::draw::cpu_register::CpuRegisterBankDrawingDefns;
use strucc::application::draw::grid_to_screen::{draw_path_grid, GridToScreenMapper};
use strucc::application::draw::instruction_memory;
use strucc::application::draw::instruction_memory::{InstructionMemoryCurrentPosition, InstructionMemoryDrawingDefns};
use strucc::application::draw::path::draw_path;
use strucc::application::draw::port::{PortColorIndex, PortDrawingDefns, PortSignalDirection, SignalType};
use strucc::application::draw::pos::{dist, pos, size, Size};
use strucc::application::draw::shapes::draw_rectangle_pos;
use strucc::application::grid::blocked_point::BlockedPoints;
use strucc::application::grid::component::{ComponentGridData, DrawableComponent};
use strucc::application::grid::cpu::{CpuComponentsGridDefns, CpuGridDefns};
use strucc::application::grid::grid_limits::GridLimits;
use strucc::application::grid::path::Path;
use strucc::application::grid::pos::grid_pos;
use strucc::application::simulation::alu::{AluBank, AluOperation, ALU_COUNT};
use strucc::application::simulation::controller::Controller;
use strucc::application::simulation::cpu_registers::{CpuRegisterBank, REGISTER_COUNT};
use strucc::application::simulation::simulation::Cpu;
use strucc::application::simulation::instruction::Instruction;
use strucc::application::simulation::instruction_reader::InstructionMemory;
use strucc::application::simulation::main_memory::MainMemory;
use strucc::word::Word;

// arch name: STruCC
//  Spatially distributed and Structured Computation and Control

fn main(){
    macroquad::Window::new("STruCC Cpu Simulator", amain());
}
async fn amain() {
    let program = vec![
        Instruction::SetAluConfig {
            alu_addr    : 2,
            alu_config  : AluOperation::Add {
                activation_input: 2 ,
                data_input_1: 0,
                data_input_0: 1,
                data_output_0: 3,
                flags_output: None,
                activation_output: None,
            }
        };
        128
    ];

    let data = vec![];

    let screen_size = size(1600, 900);

    let grid_lims = GridLimits::new(u16vec2(screen_size.x as u16 / 2 , screen_size.y as u16 / 2));

    let grid_to_screen_mapper = GridToScreenMapper::new(
        &grid_lims,
        Rect::new(0_f32, 0_f32, screen_size.x as f32, screen_size.y as f32),
    );

    let cpu = build_full_cpu(program, data, screen_size, &grid_to_screen_mapper);


    // let instruction_memory_calculate_defns=
    //     cpu.instruction_memory.calculate_defns (
    //
    //
    //     );

    loop{
        clear_background(WHITE);
        next_frame().await;
    }
}
pub struct FullCpu{
    pub sim             : Cpu,
    pub grid_defns      : CpuGridDefns,
    pub drawing_defns   : CpuDrawingDefns
}
pub struct Application{
    pub cpu: FullCpu,
    pub paths                   : Vec<Path>,
    pub screen_size             : Size,
    pub grid_to_screen_mapper   : GridToScreenMapper,
    pub grid_limits             : GridLimits,

}
fn draw(app: &Application){
    draw_full_cpu(
        &app.cpu,
        &app.grid_to_screen_mapper
    )
}
fn draw_fps(){
    draw_rectangle_pos(pos(0,0), dist(200, 30), BLACK);
    macroquad::prelude::draw_fps();
}
fn draw_paths(
    // cpu: &FullCpu,
    paths: &Vec<Path>,
    // grid_limits: &GridLimits,
    grid_to_screen_mapper: &GridToScreenMapper,
) {
    pub const PATH_COLORS: [Color;7] = [BLUE, RED, GREEN, BROWN, PURPLE, GOLD, MAGENTA];
    //
    // draw_path_grid(
    //     grid_to_screen_mapper,
    //     grid_limits,
    //     &cpu.grid_defns.blocked_points
    // );

    for (ix, path) in paths.iter().enumerate(){
        draw_path(
            path,
            &PATH_COLORS[ix%PATH_COLORS.len()],
            grid_to_screen_mapper,
        )
    }
}

fn draw_full_cpu(
    cpu                     : &FullCpu,
    // grid_limits             : &GridLimits,
    grid_to_screen_mapper   : &GridToScreenMapper,
) {

    // draw_path_grid(
    //     &grid_to_screen_mapper,
    //     &grid_limits,
    //     &BlockedPoints::new()
    // );

    cpu.sim.instruction_memory.draw(
        &InstructionMemoryCurrentPosition(0),
        &cpu.grid_defns.instruction_memory,
        &cpu.drawing_defns.instruction_memory,
        &cpu.drawing_defns.port,
        &grid_to_screen_mapper
    );

    let registers_drawing_state = Box::new([();REGISTER_COUNT]);

    cpu.sim.register_bank.draw(
        &registers_drawing_state,
        &cpu.grid_defns.register_bank,
        &cpu.drawing_defns.register_bank,
        &cpu.drawing_defns.port,
        &grid_to_screen_mapper
    );
    let alu_bank_drawing_state = Box::new([(); ALU_COUNT]);
    cpu.sim.alu_bank.draw(
        &alu_bank_drawing_state,
        &cpu.grid_defns.alu_bank,
        &cpu.drawing_defns.alu_bank,
        &cpu.drawing_defns.port,
        &grid_to_screen_mapper
    );
}

fn build_full_cpu(
    program                 : Vec<Instruction>,
    data                    : Vec<Word>,
    screen_size             : Size,
    grid_to_screen_mapper   : &GridToScreenMapper,
) -> FullCpu {

    let mut main_memory = MainMemory::new(data);

    let registers = CpuRegisterBank::new();

    let instruction_memory = InstructionMemory::new(program.clone());

    let alus = AluBank::new(&mut  main_memory);

    let controller = Controller::new(
        &instruction_memory,
    );

    let cpu = {
        Cpu {
            main_memory,
            alu_bank: alus,
            register_bank: registers,
            controller,
            instruction_memory,
        }
    };


    let port_drawing_data = PortDrawingDefns {
        base        : 6,
        line_len    : 4,
        arrow_height: 8,
        line_width  : 1,
        color_defn: Box::new(|color| -> Color{
            match color{
                PortColorIndex::Deactivated => {DARKGRAY},
                PortColorIndex::Active(SignalType::Data, PortSignalDirection::Input) => {BLUE}
                PortColorIndex::Active(SignalType::Data, PortSignalDirection::Output) => {RED}
                PortColorIndex::Active(SignalType::Activation, PortSignalDirection::Input) => {GREEN}
                PortColorIndex::Active(SignalType::Activation, PortSignalDirection::Output) => {YELLOW}
                PortColorIndex::Active(SignalType::Setup, PortSignalDirection::Input) => {VIOLET}
                PortColorIndex::Active(SignalType::Setup, PortSignalDirection::Output) => {BROWN}
            }
        }),
    };

    let register_bank_drawing_data = CpuRegisterBankDrawingDefns {
        size: size(screen_size.x / 2, screen_size.y / 2),
        row_count: 8,
        inner_drawing_defns: Default::default(),
    };


    let alu_bank_drawing_data =
        AluBankDrawingDefns {
            size: screen_size / 2,
            row_count: 4,
            inner_drawing_defns: Default::default(),
        };

    let instruction_memory_top_left =
        grid_pos(1, 1)  ;

    let register_bank_top_left =
        grid_to_screen_mapper
            .screen_to_nearest_grid_pos( pos(screen_size.x / 3,  screen_size.y / 2)  );

    let alu_bank_top_left =
        grid_to_screen_mapper
            .screen_to_nearest_grid_pos( pos(screen_size.x / 3, 0)  );


    let alu_bank_grid_defns = cpu.alu_bank.calculate_defns(
        alu_bank_top_left,
        &alu_bank_drawing_data,
        &port_drawing_data,
        &grid_to_screen_mapper
    );
    let register_bank_calculated_defns = cpu.register_bank.calculate_defns(
        register_bank_top_left,
        &register_bank_drawing_data,
        &port_drawing_data,
        &grid_to_screen_mapper
    );

    let instruction_mem_drawing_defns = InstructionMemoryDrawingDefns {
        size: pos(screen_size.x/4, screen_size.y)
    };

    let instruction_mem_calculated_defns =
        cpu
        .instruction_memory
        .calculate_defns(
            instruction_memory_top_left,
            &instruction_mem_drawing_defns,
            &port_drawing_data,
            &grid_to_screen_mapper
        );

    let cpu_drawing_defns = CpuDrawingDefns{
        port: port_drawing_data,
        register_bank: register_bank_drawing_data,
        alu_bank: alu_bank_drawing_data,
        instruction_memory: instruction_mem_drawing_defns,
    };
    let all_blocked_points = {
        let mut blocked = alu_bank_grid_defns.blocked_points.clone();
        blocked.add_from(register_bank_calculated_defns.blocked_points());
        blocked.add_from(instruction_mem_calculated_defns.blocked_points());
        blocked
    };

    let cpu_grid_defns = CpuGridDefns{
        alu_bank            : alu_bank_grid_defns,
        register_bank       : register_bank_calculated_defns,
        instruction_memory  : instruction_mem_calculated_defns,
        blocked_points: all_blocked_points
    };

    FullCpu{
        sim: cpu,
        grid_defns: cpu_grid_defns,
        drawing_defns: cpu_drawing_defns,
    }
}

pub fn calculate_paths(
    cpu             : &Cpu,
    cpu_grid_defns  : &CpuGridDefns,
    grid_limits     : &GridLimits
) -> Vec<Path>{
         
}