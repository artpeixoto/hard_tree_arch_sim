use crate::application::grid::pos::{grid_pos, grid_size, GridPos, GridSize};

#[derive(Clone, PartialEq, Eq, Debug,)]
pub struct GridRect{
    pub top_left    : GridPos,
    pub size        : GridSize,
}
pub const fn grid_rect(top_left: GridPos, size : GridSize) -> GridRect {
    GridRect::new(top_left, size)
}

impl GridRect{
    pub const fn new(top_left: GridPos, size: GridSize) -> Self{
        // TODO: adicionar logica para lidar com as possibilidades melhor
        Self{top_left, size}
    }
    pub fn new_from_points(GridPos{x: x0, y: y0} : GridPos, GridPos{x: x1, y: y1}: GridPos) -> Self{
        let (left, right) = if x0 <= x1 { (x0, x1) } else { (x1, x0) };
        let (top, bottom) = if y0 <= y1 { (y0, y1) } else { (y1, y0) };
        Self{
            top_left: grid_pos( left, top ),
            size    : grid_size( right - left, bottom - top ),
        }
    }
}