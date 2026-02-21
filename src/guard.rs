use sdl3_sys::blendmode::SDL_BlendMode;

use crate::{
    color::{RgbF32, RgbU8, RgbaF32, RgbaU8},
    renderer::RendererRef,
    texture::TextureRef,
    traits::{BlendMode, ColorModF32, ColorModU8},
};

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

pub struct DrawColorGuard<'a> {
    rnd: RendererRef<'a>,
    old: RgbaF32,
}

impl<'a> DrawColorGuard<'a> {
    pub fn new(rnd: RendererRef<'a>, color: RgbaF32) -> Self {
        let old = rnd.draw_color_f32();

        rnd.set_draw_color_f32(color);

        Self { rnd, old }
    }

    pub fn set(&self, color: RgbaF32) {
        self.rnd.set_draw_color_f32(color);
    }
}

impl Drop for DrawColorGuard<'_> {
    fn drop(&mut self) {
        self.rnd.set_draw_color_f32(self.old);
    }
}

pub struct RenderTargetGuard<'a> {
    rnd: RendererRef<'a>,
    old: Option<TextureRef<'a>>,
}

impl<'a> RenderTargetGuard<'a> {
    pub fn new(rnd: RendererRef<'a>, target: TextureRef) -> Self {
        let old = unsafe { rnd.target() };

        let _ = rnd.set_target(target);

        Self { rnd, old }
    }

    pub fn set(&self, target: TextureRef) {
        let _ = self.rnd.set_target(target);
    }
}

impl Drop for RenderTargetGuard<'_> {
    fn drop(&mut self) {
        let _ = match self.old {
            Some(tgt) => self.rnd.set_target(tgt),
            None => self.rnd.reset_target(),
        };
    }
}

pub struct AlphaModF32Guard<T: ColorModF32> {
    obj: T,
    old: f32,
}

impl<T: ColorModF32> AlphaModF32Guard<T> {
    pub fn new(obj: T, am: f32) -> Self {
        let old = obj.alpha_mod_f32();

        let _ = obj.set_alpha_mod_f32(am);

        Self { obj, old }
    }

    pub fn set(&self, am: f32) {
        let _ = self.obj.set_alpha_mod_f32(am);
    }
}

impl<T: ColorModF32> Drop for AlphaModF32Guard<T> {
    fn drop(&mut self) {
        self.obj.set_alpha_mod_f32(self.old);
    }
}

pub struct RgbModF32Guard<T: ColorModF32> {
    obj: T,
    old: RgbF32,
}

impl<T: ColorModF32> RgbModF32Guard<T> {
    pub fn new(obj: T, am: RgbF32) -> Self {
        let old = obj.rgb_mod_f32();

        let _ = obj.set_rgb_mod_f32(am);

        Self { obj, old }
    }

    pub fn set(&self, am: RgbF32) {
        let _ = self.obj.set_rgb_mod_f32(am);
    }
}

impl<T: ColorModF32> Drop for RgbModF32Guard<T> {
    fn drop(&mut self) {
        self.obj.set_rgb_mod_f32(self.old);
    }
}

pub struct ColorModF32Guard<T: ColorModF32> {
    obj: T,
    old: RgbaF32,
}

impl<T: ColorModF32> ColorModF32Guard<T> {
    pub fn new(obj: T, am: RgbaF32) -> Self {
        let old = obj.color_mod_f32();

        let _ = obj.set_color_mod_f32(am);

        Self { obj, old }
    }

    pub fn set(&self, am: RgbaF32) {
        let _ = self.obj.set_color_mod_f32(am);
    }
}

impl<T: ColorModF32> Drop for ColorModF32Guard<T> {
    fn drop(&mut self) {
        self.obj.set_color_mod_f32(self.old);
    }
}

pub struct AlphaModU8Guard<T: ColorModU8> {
    obj: T,
    old: u8,
}

impl<T: ColorModU8> AlphaModU8Guard<T> {
    pub fn new(obj: T, am: u8) -> Self {
        let old = obj.alpha_mod_u8();

        let _ = obj.set_alpha_mod_u8(am);

        Self { obj, old }
    }

    pub fn set(&self, am: u8) {
        let _ = self.obj.set_alpha_mod_u8(am);
    }
}

impl<T: ColorModU8> Drop for AlphaModU8Guard<T> {
    fn drop(&mut self) {
        self.obj.set_alpha_mod_u8(self.old);
    }
}

pub struct RgbModU8Guard<T: ColorModU8> {
    obj: T,
    old: RgbU8,
}

impl<T: ColorModU8> RgbModU8Guard<T> {
    pub fn new(obj: T, am: RgbU8) -> Self {
        let old = obj.rgb_mod_u8();

        let _ = obj.set_rgb_mod_u8(am);

        Self { obj, old }
    }

    pub fn set(&self, am: RgbU8) {
        let _ = self.obj.set_rgb_mod_u8(am);
    }
}

impl<T: ColorModU8> Drop for RgbModU8Guard<T> {
    fn drop(&mut self) {
        self.obj.set_rgb_mod_u8(self.old);
    }
}

pub struct ColorModU8Guard<T: ColorModU8> {
    obj: T,
    old: RgbaU8,
}

impl<T: ColorModU8> ColorModU8Guard<T> {
    pub fn new(obj: T, am: RgbaU8) -> Self {
        let old = obj.color_mod_u8();

        let _ = obj.set_color_mod_u8(am);

        Self { obj, old }
    }

    pub fn set(&self, am: RgbaU8) {
        let _ = self.obj.set_color_mod_u8(am);
    }
}

impl<T: ColorModU8> Drop for ColorModU8Guard<T> {
    fn drop(&mut self) {
        self.obj.set_color_mod_u8(self.old);
    }
}
