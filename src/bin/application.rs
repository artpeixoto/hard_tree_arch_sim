use macroquad::prelude::{ * };
use strucc::application::simulation::alu::{AluOperation, Alus};
use strucc::application::draw::instruction_memory::draw_instruction_memory;
use strucc::application::draw::pos::pos;
use strucc::application::simulation::cpu::Cpu;
use strucc::application::simulation::cpu_registers::{CpuRegisterBank, CPU_REGISTER_COUNT};
use strucc::application::simulation::instruction::Instruction;
use strucc::application::simulation::instruction_reader::InstructionMemory;

// arch name: STruCC
//  Spatially distributed sTRUctured Computation and Control

fn main(){
    macroquad::Window::new("STruCC Cpu Simulator", amain());
}
async fn amain() {
    let mut cpu = Cpu::new(
        vec![
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
        ]);
    // for i in 0..CPU_REGISTER_COUNT {
    //     cpu.registers.write(i32::MAX);
    // }
    // 
    // for i in 0..CPU_REGISTER_COUNT {
    //     cpu.registers.step(&1);
    //     // step_cpu_registers(cpu.registers)
    // }
    // 

    loop{
        clear_background(WHITE);
        clear_background(BEIGE.with_alpha(0.2));
        
        let screen_width = screen_width() as i32;
        let screen_height = screen_height() as i32;

        draw_instruction_memory(
            &cpu.instruction_memory,
            &pos(4, 4),
            &pos(400, screen_height - 40),
            0
        );

        // draw_cpu_registers(
        //     &cpu.registers,
        //     pos(500,4),
        //     pos(1200,300)
        // );

        next_frame().await;
    }
}