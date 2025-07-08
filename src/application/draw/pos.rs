
pub type FPos = macroquad::prelude::Vec2;
pub type FSize= macroquad::prelude::Vec2;
pub type FDist= macroquad::prelude::Vec2;

// pub use macroquad::prelude::IVec2 as Pos;
// pub use macroquad::prelude::IVec2 as Dist;
// pub use macroquad::prelude::ivec2 as dist;

pub type Pos = macroquad::prelude::IVec2;
pub type Dist = macroquad::prelude::IVec2;
pub type Size = macroquad::prelude::IVec2;
pub type ScreenUnit = i32;

pub use macroquad::prelude::ivec2 as pos;
pub use macroquad::prelude::ivec2 as dist;
pub use macroquad::prelude::ivec2 as size;

pub use macroquad::math::vec2 as fpos;
pub use macroquad::prelude::vec2 as fdist;
pub use macroquad::prelude::vec2 as fsize;

// just to aid with reading, and if i want to change it
// later, i can
pub trait ToFPosExt {
    fn as_fpos(&self ) -> FPos;
}

impl ToFPosExt for Pos{
    fn as_fpos(&self) -> FPos {
        fpos(
            self.x as f32 + 0.5,
            self.y as f32 + 0.5
        )
    }
}
pub trait ToFDistExt {
    fn as_fdist(&self ) -> FDist;
}

impl ToFDistExt for Dist{
    fn as_fdist(&self) -> FDist {
        fdist(
            self.x as f32,
            self.y as f32
        )
    }
}
pub trait AsPosExt {
    fn as_pos(&self ) -> Pos;
}

impl AsPosExt for FPos{
    fn as_pos(&self) -> Pos {
        pos(
            (self.x - 0.5)  as i32,
            (self.y - 0.5) as i32,
        )
    }
}



