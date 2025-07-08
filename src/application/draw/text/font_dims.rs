use crate::application::draw::pos::ScreenUnit;



#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FontDimensions {
    pub top_height      : ScreenUnit,
    pub bottom_height   : ScreenUnit,
    pub drawing_size    : u16,
}

impl FontDimensions {
    pub const fn full_height(&self) -> ScreenUnit { self.top_height + self.bottom_height }
}