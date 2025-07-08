use crate::tools::convert::ConvertInto;
use itertools::Itertools;
use palette::{IntoColor, WithAlpha};
use crate::tools::used_in::UsedIn;

pub type MacroquadColor = macroquad::color::Color;
pub type PaletteLinColor = palette::LinSrgb;
pub type PaletteLinAColor = palette::LinSrgba;
pub type PaletteAColor = palette::Srgba;
pub type PaletteColor = palette::Srgb;

pub trait ToMacroquadColorExt{
    fn to_macroquad_color(&self) -> MacroquadColor;
}


impl ToMacroquadColorExt for PaletteAColor {
    #[inline(always)]
    fn to_macroquad_color(&self) -> MacroquadColor {
        MacroquadColor::new(self.red, self.green, self.blue, self.alpha)
    }
}
impl ToMacroquadColorExt for PaletteColor {

    #[inline(always)]
    fn to_macroquad_color(&self) -> MacroquadColor {
        self.with_alpha(1.0).to_macroquad_color()
    }
}

impl ToMacroquadColorExt for PaletteLinColor {

    #[inline(always)]
    fn to_macroquad_color(&self) -> MacroquadColor {
        let normal_color: PaletteColor = self.clone().into();
        normal_color.to_macroquad_color()
    }
}

impl ToMacroquadColorExt for PaletteLinAColor {

    #[inline(always)]
    fn to_macroquad_color(&self) -> MacroquadColor {
        let normal_color: PaletteAColor = self.clone().into();
        normal_color.to_macroquad_color()
    }
}

pub trait ToPaletteColorExt {
    fn to_palette_color(&self) -> PaletteAColor;
}

impl ToPaletteColorExt for MacroquadColor {
    fn to_palette_color(&self) -> PaletteAColor {
        PaletteAColor::new(
            self.r,
            self.g,
            self.b,
            self.a
        )
    }
}