use sdl3_sys::blendmode::SDL_BlendMode;

use crate::color::{RgbF32, RgbU8, RgbaF32, RgbaU8};

pub trait BlendMode {
    fn blend_mode(&self) -> SDL_BlendMode;
    fn set_blend_mode(&self, bm: SDL_BlendMode);
}

pub trait ColorModU8 {
    fn rgb_mod_u8(&self) -> RgbU8;
    fn alpha_mod_u8(&self) -> u8;

    fn color_mod_u8(&self) -> RgbaU8 {
        RgbaU8::new(self.rgb_mod_u8(), self.alpha_mod_u8())
    }

    fn set_rgb_mod_u8(&self, rm: RgbU8);
    fn set_alpha_mod_u8(&self, am: u8);

    fn set_color_mod_u8(&self, col: RgbaU8) {
        self.set_rgb_mod_u8(col.rgb);
        self.set_alpha_mod_u8(col.a);
    }
}

pub trait ColorModF32 {
    fn rgb_mod_f32(&self) -> RgbF32;
    fn alpha_mod_f32(&self) -> f32;

    fn color_mod_f32(&self) -> RgbaF32 {
        RgbaF32::new(self.rgb_mod_f32(), self.alpha_mod_f32())
    }

    fn set_rgb_mod_f32(&self, rm: RgbF32);
    fn set_alpha_mod_f32(&self, am: f32);

    fn set_color_mod_f32(&self, col: RgbaF32) {
        self.set_rgb_mod_f32(col.rgb);
        self.set_alpha_mod_f32(col.a);
    }
}
