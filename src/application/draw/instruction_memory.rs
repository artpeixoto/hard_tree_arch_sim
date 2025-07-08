use macroquad::color::{BLACK, DARKGRAY, GRAY, WHITE};
use macroquad::math::{ivec2, Vec2};
use macroquad::prelude::{draw_line,  draw_text, measure_text};
use crate::application::direction::HorOrVer;
use crate::application::draw::cursor::RectCursor;
use crate::application::draw::shapes::{draw_line_pos, draw_rectangle_pos};
use crate::application::draw::text::{draw_multiline_text_pos, draw_text_pos, draw_title, measure_multiline_text, normal_font, TextStyle};
use crate::application::draw::pos::{ Dist, Pos, ScreenUnit, Size, *};
use crate::application::simulation::instruction_reader::InstructionMemory;

pub fn draw_instruction_memory(
    memory      : &InstructionMemory,
    top_left    : &Pos,
    size        : &Size,
    current_ix  : usize
) {
    let mut cursor = RectCursor::new(*top_left, *size);

    let title_dims = draw_title("Instruction Memory", *top_left, 2, BLACK,);

    cursor.go(Dist::new(0, title_dims.height as ScreenUnit * 2  + 2));
    let initial_cursor = cursor.clone();
    {
        let mut current_cell_ix = current_ix;
        loop {
            let Some(current_instruction) = memory.0.get(current_cell_ix).cloned() else {break};
            let instruction_value_text = format!("{:#?}", current_instruction);
            let font_dims = normal_font::DIMS;

            // draw top line
            draw_line_pos(
                cursor.top_left(),
                pos(cursor.right(), cursor.top()),
                1,
                BLACK
            ); 

            { // draw address
                let mut cursor = 
                    cursor
                    .split(
                        font_dims.full_height() + 4, 
                        HorOrVer::Vertical
                    );
                
                draw_rectangle_pos(
                    cursor.top_left(),
                    cursor.remaining_size(),
                    DARKGRAY
                );

                cursor.pad(0,2);

                draw_text_pos(
                    &format!("ADDR {:}", current_cell_ix),
                    cursor.top_left() + dist(2,2),
                    TextStyle::Normal,
                    1,
                    WHITE
                );
            }

            // prepare to draw instruction
            let text_dims = measure_multiline_text(&instruction_value_text, TextStyle::Normal, 1 );

            // verify we have enough space to do so
            if text_dims.height as i32 + cursor.top() > cursor.bottom(){
                break;
            }
 
            { // draw instruction
                draw_multiline_text_pos(
                    &instruction_value_text,
                    cursor.top_left() + dist(2,2),
                    TextStyle::Normal,
                    1,
                    BLACK
                );
            }
            cursor.go(dist(0, text_dims.height as i32 + 4));
            current_cell_ix += 1;
        }
    }
    
    draw_line_pos(
        initial_cursor.top_left(),
        initial_cursor.bottom_left(),
        1,
        BLACK
    ); 

    draw_line_pos(
        initial_cursor.top_right(),
        initial_cursor.bottom_right(),
        1,
        BLACK
    ); 
}