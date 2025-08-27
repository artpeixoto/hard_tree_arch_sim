use macroquad::prelude::*;
use simulation::alu::AluBank;
use crate::application::draw::text::draw_title;
use draw::pos::Pos;
use simulation::simulation::Cpu;
use simulation::instruction::Instruction;
use crate::word::Word;

pub mod grid;
pub mod draw;
pub mod prelude;
pub mod direction;
pub mod simulation;

// pub async fn run_simulation(program: Vec<Instruction>, data: Vec<Word>){
//     let cpu = Cpu::new(program, data);
//     let step = 0;
//              
//     loop {
//     }
// }

// pub fn draw_alus(
//     alus        : &Alus,
//     mut top_left    : Pos,
//     mut size        : Pos,
// ) {
// 
//     // {   // draw bounding rectangle
//     //     draw_rectangle_lines(
//     //         top_left.x,
//     //         top_left.y,
//     //         top_left.x + size.x,
//     //         top_left.y + size.y,
//     //         1.0,
//     //         BLACK
//     //     );
//     //
//     //     top_left = top_left + vec2(, 2.0);
//     //     size     = size - vec2(4.0, 4.0); //consider padding
//     // }
// 
//     { // draw title
//         let title_dims = draw_title(
//             "Alus",
//             top_left,
//             1,
//             BLACK
//         );
//         top_left.y = top_left.y + title_dims.height as i32;
//         size.y -= title_dims.height as i32;
//     }
// 
//     { // draw alus
// 
//     }
// 
// }





