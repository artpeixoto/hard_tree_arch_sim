use std::sync::LazyLock;
use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::prelude::{Font, TextDimensions, TextParams};
use macroquad::text::{draw_text, draw_text_ex, load_ttf_font_from_bytes};
use crate::application::draw::pos::Pos;
use crate::define_font;

define_font!(tiny_font = {
    source: "TinyUnicode.ttf",
    dims: FontDimensions{
        top_height: 7,
        bottom_height: 2,
        drawing_size: 16,
    }
});
///
/// font height is 7. 5 going up, 2 going down. Generally. let a distance of 8 between one and another and it should be fine
pub fn draw_text_line_tiny(text: &str, pos: Pos, size_scale: u8, color: Color,) -> TextDimensions {
    draw_text_ex(
        text,
        pos.x as f32 ,
        (pos.y + tiny_font::DIMS.top_height as i32) as f32,
        TextParams {
            font_size: 16 * (size_scale as u16),
            font_scale: 1.0,
            color,
            font: Some(&tiny_font::FONT),
            ..Default::default()
        },
    )
}