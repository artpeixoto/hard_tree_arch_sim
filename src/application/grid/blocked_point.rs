use std::cmp::{max, min};
use wgpu::naga::FastHashSet;
use crate::application::grid::cpu_register::CpuRegisterGridInfo;
use crate::application::grid::pos::{grid_dist, grid_pos, grid_size, GridPos, GridSize};
use crate::application::grid::rect::{grid_rect, GridRect};

pub struct BlockedPoints (pub FastHashSet<GridPos>);
impl BlockedPoints{
    pub fn new() -> Self{
        BlockedPoints(FastHashSet::default())
    }
    pub fn new_from_blocked_inner_rect(rect: GridRect) -> Self{
        let mut res = BlockedPoints::new();
        res.block_rect(grid_rect(
            rect.top_left + grid_dist(1,1),
            rect.size - grid_size(2,2)
        ));
        res
    }
    pub fn block_rect(&mut self, GridRect{top_left, size}: GridRect) {

        for x in (0..size.x).into_iter().map(|x| x + top_left.x){
            for y in (0..size.y).into_iter().map(|y| y + top_left.y){
                self.0.insert(grid_pos(x,y));
            }
        }
    }
    pub fn add_from(&mut self, other: BlockedPoints){
        self.0.extend(other.0.iter()); 
    }
    
    pub fn add_point(&mut self, point: GridPos) {
        self.0.insert(point);
    }
    
    pub fn point_is_available(&self, point: &GridPos) -> bool{
        !self.0.contains(point)
    }
}
