use macroquad::math::{I16Vec2, U16Vec2};
use crate::application::direction::HorOrVer;
use crate::application::direction::HorOrVer::{Horizontal, Vertical};
use crate::application::grid::pos::{grid_pos, GridPos};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct GridLine {
    pub (in super) index    : GridPos,
    pub hor_or_ver          : HorOrVer,
}

impl GridLine {
    pub fn points(&self) -> [GridPos;2] {
        let first = grid_pos(self.index.x, self.index.y);
        let second = match  self.hor_or_ver {
            Horizontal  => first + I16Vec2::new(1, 0),
            Vertical    => first + I16Vec2::new(0, 1),
        };
        [first, second]
    }
}
