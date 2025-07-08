use macroquad::color::Color;
use macroquad::math::{vec2, Vec2};
use macroquad::miniquad::window::dpi_scale;
use macroquad::prelude::{draw_text, TextParams};
use macroquad::text::{draw_text_ex, load_ttf_font, TextDimensions};
use crate::application::draw::text::draw_text_line_normal;
use crate::application::draw::pos::Pos;
use crate::define_font;

define_font!(
    title_font = {
        source: "pixeled-wide.ttf",
        dims:
            FontDimensions{
                top_height: 6,
                bottom_height: 0,
                drawing_size: 5
            }
    }
);

pub fn draw_title( title: &str, pos: Pos, font_scale: u8, color: Color) -> TextDimensions {
    let dims = draw_text_ex(
        title,
        pos.x as f32 ,
        (pos.y + title_font::DIMS.top_height * font_scale as i32) as f32,
        TextParams{
            font: Some(&title_font::FONT),
            font_size: (5 * (font_scale as u16)),
            font_scale: 1.0,
            color: color.clone(),
            ..Default::default()
        },
    );
    dims
}