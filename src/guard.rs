use sdl3_sys::blendmode::SDL_BlendMode;

use crate::{
    color::{RgbF32, RgbaF32},
    renderer::RendererRef,
    texture::TextureRef,
    traits::{BlendMode, ColorMod},
};

pub struct DrawColorGuard {
    rnd: RendererRef,
    old: RgbaF32,
}

impl DrawColorGuard {
    pub fn new(rnd: impl Into<RendererRef>, color: RgbaF32) -> Self {
        let rnd: RendererRef = rnd.into();
        let old = rnd.draw_color_f32();

        rnd.set_draw_color_f32(color);

        Self { rnd, old }
    }

    pub fn set(&self, color: RgbaF32) {
        self.rnd.set_draw_color_f32(color);
    }
}

impl Drop for DrawColorGuard {
    fn drop(&mut self) {
        self.rnd.set_draw_color_f32(self.old);
    }
}

pub struct RenderTargetGuard {
    rnd: RendererRef,
    old: Option<TextureRef>,
}

impl RenderTargetGuard {
    pub fn new(rnd: impl Into<RendererRef>, target: impl Into<TextureRef>) -> Self {
        let rnd: RendererRef = rnd.into();
        let old = rnd.target();

        let _ = rnd.set_target(target);

        Self { rnd, old }
    }

    pub fn set(&self, target: impl Into<TextureRef>) {
        let _ = self.rnd.set_target(target);
    }
}

impl Drop for RenderTargetGuard {
    fn drop(&mut self) {
        let _ = match self.old {
            Some(tgt) => self.rnd.set_target(tgt),
            None => self.rnd.reset_target(),
        };
    }
}

pub struct AlphaModGuard<T: ColorMod> {
    obj: T,
    old: f32,
}

impl<T: ColorMod> AlphaModGuard<T> {
    pub fn new(obj: T, am: f32) -> Self {
        let old = obj.alpha_mod_f32();

        let _ = obj.set_alpha_mod_f32(am);

        Self { obj, old }
    }

    pub fn set(&self, am: f32) {
        let _ = self.obj.set_alpha_mod_f32(am);
    }
}

impl<T: ColorMod> Drop for AlphaModGuard<T> {
    fn drop(&mut self) {
        self.obj.set_alpha_mod_f32(self.old);
    }
}

pub struct RgbModGuard<T: ColorMod> {
    obj: T,
    old: RgbF32,
}

impl<T: ColorMod> RgbModGuard<T> {
    pub fn new(obj: T, am: RgbF32) -> Self {
        let old = obj.rgb_mod_f32();

        let _ = obj.set_rgb_mod_f32(am);

        Self { obj, old }
    }

    pub fn set(&self, am: RgbF32) {
        let _ = self.obj.set_rgb_mod_f32(am);
    }
}

impl<T: ColorMod> Drop for RgbModGuard<T> {
    fn drop(&mut self) {
        self.obj.set_rgb_mod_f32(self.old);
    }
}

pub struct ColorModGuard<T: ColorMod> {
    obj: T,
    old: RgbaF32,
}

impl<T: ColorMod> ColorModGuard<T> {
    pub fn new(obj: T, am: RgbaF32) -> Self {
        let old = obj.color_mod_f32();

        let _ = obj.set_color_mod_f32(am);

        Self { obj, old }
    }

    pub fn set(&self, am: RgbaF32) {
        let _ = self.obj.set_color_mod_f32(am);
    }
}

impl<T: ColorMod> Drop for ColorModGuard<T> {
    fn drop(&mut self) {
        self.obj.set_color_mod_f32(self.old);
    }
}

pub struct BlendModeGuard<T: BlendMode> {
    rnd: T,
    old: SDL_BlendMode,
}

impl<T: BlendMode> BlendModeGuard<T> {
    pub fn new(rnd: T, bm: SDL_BlendMode) -> Self {
        let old = rnd.blend_mode();

        let _ = rnd.set_blend_mode(bm);

        Self { rnd, old }
    }

    pub fn set(&self, bm: SDL_BlendMode) {
        let _ = self.rnd.set_blend_mode(bm);
    }
}

impl<T: BlendMode> Drop for BlendModeGuard<T> {
    fn drop(&mut self) {
        self.rnd.set_blend_mode(self.old);
    }
}
