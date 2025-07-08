use std::borrow::Cow;
use macroquad::color::{Color, BLUE, BROWN, GOLD, GREEN, MAGENTA, PURPLE, RED};
use macroquad::prelude::draw_line;
use crate::application::draw::grid_to_screen::GridToScreenMapper;
use crate::application::draw::shapes::draw_line_pos;
use crate::application::grid::path::Path;

pub type PathDrawingInfo = Color;
// pub struct PathDrawingInfo<'a>{
//     pub color: Cow<'a, Color>
// }

pub const PATH_COLORS: [Color;7] = [BLUE, RED, GREEN, BROWN, PURPLE, GOLD, MAGENTA];

impl Path{
    pub fn draw(&self, drawing_info: &PathDrawingInfo, grid_transform: &GridToScreenMapper) {
        draw_path(&self, drawing_info, grid_transform);
    }
}

pub fn draw_path(path: &Path, drawing_info: &PathDrawingInfo, grid_transform: &GridToScreenMapper, ) {
    for movement in path.walk(){
        let (p0, p1) =
            grid_transform.get_line(movement.line);

        draw_line_pos(
            p0,
            p1,
            1,
            *drawing_info
        );
    }
}