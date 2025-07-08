use std::sync::LazyLock;
use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::miniquad::FilterMode;
use macroquad::prelude::{draw_text_ex, load_ttf_font_from_bytes, Font, TextDimensions, TextParams};
use crate::application::draw::text::font_dims::FontDimensions;
use crate::application::draw::pos::Pos;
use crate::define_font;

define_font!(normal_font = {
    source: "ProggySmall.ttf",
    dims: FontDimensions{
        top_height: 8,
        bottom_height: 2,
        drawing_size: 16,
    }
});

pub fn draw_text_line_normal(
    text        : &str,
    pos         : Pos,
    size_scale  : u8,
    color       : Color,
) -> TextDimensions {
    draw_text_ex(
        text,
        pos.x as f32,
        (pos.y + normal_font::DIMS.top_height as i32) as f32,
        TextParams {
            font: Some(&normal_font::FONT),
            font_size: 16 * (size_scale as u16),
            font_scale: 1.0,
            color,
            ..Default::default()
        },
    )
}
