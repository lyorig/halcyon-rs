use sdl3_sys::blendmode::SDL_BlendMode;

use crate::{color::RgbaF32, renderer::RendererRef, texture::TextureRef};

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

pub struct BlendModeGuard {
    rnd: RendererRef,
    old: SDL_BlendMode,
}

impl BlendModeGuard {
    pub fn new(rnd: impl Into<RendererRef>, bm: SDL_BlendMode) -> Self {
        let rnd: RendererRef = rnd.into();
        let old = rnd.blend_mode();

        let _ = rnd.set_blend_mode(bm);

        Self { rnd, old }
    }

    pub fn set(&self, bm: SDL_BlendMode) {
        let _ = self.rnd.set_blend_mode(bm);
    }
}

impl Drop for BlendModeGuard {
    fn drop(&mut self) {
        self.rnd.set_blend_mode(self.old);
    }
}
