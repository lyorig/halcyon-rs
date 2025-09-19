use std::{
    ffi::{CStr, c_void},
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use sdl3_sys::{
    pixels::SDL_Colorspace,
    rect::{SDL_FPoint, SDL_FRect},
    render::*,
};

use crate::{
    defs::SdlResult,
    error,
    properties::Properties,
    surface::SurfaceRef,
    texture::TextureRef,
    util::{self, to_result},
    window::WindowRef,
};

pub struct RendererBuilder {
    inner: Properties,
}

impl RendererBuilder {
    pub fn new<'w>(wnd: impl Into<WindowRef>) -> Self {
        let mut ret = Self {
            inner: Properties::new(),
        };

        ret.window(wnd);
        ret
    }

    pub fn name(&mut self, value: &CStr) -> &mut Self {
        let _ = self
            .inner
            .set_string(SDL_PROP_RENDERER_CREATE_NAME_STRING, value);

        self
    }

    /// Private and only used in `RendererBuilder::new()`.
    fn window(&mut self, value: impl Into<WindowRef>) -> &mut Self {
        let _ = self.inner.set_pointer(
            SDL_PROP_RENDERER_CREATE_WINDOW_POINTER,
            value.into().handle.as_ptr() as *mut c_void,
        );

        self
    }

    pub fn surface<'surf>(&mut self, value: impl Into<SurfaceRef>) -> &mut Self {
        let _ = self.inner.set_pointer(
            SDL_PROP_RENDERER_CREATE_SURFACE_POINTER,
            value.into().handle.as_ptr() as *mut c_void,
        );

        self
    }

    pub fn colorspace(&mut self, value: SDL_Colorspace) -> &mut Self {
        let _ = self.inner.set_number(
            SDL_PROP_RENDERER_CREATE_OUTPUT_COLORSPACE_NUMBER,
            value.0.into(),
        );
        self
    }

    pub fn vsync(&mut self, value: i64) -> &mut Self {
        let _ = self
            .inner
            .set_number(SDL_PROP_RENDERER_CREATE_PRESENT_VSYNC_NUMBER, value);

        self
    }

    /// Build the renderer.
    ///
    /// This doesn't require a `Video` subsystem parameter, as the `Window`
    /// you're creating this with needs one, proving the subsystem has been
    /// initialized.
    #[doc(alias = "SDL_CreateRendererWithProperties")]
    pub fn build(&self) -> SdlResult<Renderer> {
        Renderer::from_ptr(unsafe { SDL_CreateRendererWithProperties(self.inner.id()) })
    }
}

#[derive(Clone, Copy)]
pub struct RendererRef {
    pub(crate) handle: NonNull<SDL_Renderer>,
}

impl RendererRef {
    #[doc(alias = "SDL_RenderClear")]
    pub fn clear(&self) -> SdlResult {
        to_result(unsafe { SDL_RenderClear(self.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_RenderPresent")]
    pub fn present(&self) -> SdlResult {
        to_result(unsafe { SDL_RenderPresent(self.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_RenderTexture")]
    pub fn draw(
        &self,
        tex: impl Into<TextureRef>,
        src: Option<&SDL_FRect>,
        dst: Option<&SDL_FRect>,
    ) -> SdlResult {
        to_result(unsafe {
            SDL_RenderTexture(
                self.handle.as_ptr(),
                tex.into().handle.as_ptr(),
                util::opt2ptr(src),
                util::opt2ptr(dst),
            )
        })
    }

    #[doc(alias = "SDL_RenderTextureAffine")]
    pub fn draw_affine(
        &self,
        tex: impl Into<TextureRef>,
        src: Option<&SDL_FRect>,
        origin: Option<&SDL_FPoint>,
        right: Option<&SDL_FPoint>,
        down: Option<&SDL_FPoint>,
    ) -> SdlResult {
        to_result(unsafe {
            SDL_RenderTextureAffine(
                self.handle.as_ptr(),
                tex.into().handle.as_ptr(),
                util::opt2ptr(src),
                util::opt2ptr(origin),
                util::opt2ptr(right),
                util::opt2ptr(down),
            )
        })
    }

    #[doc(alias = "SDL_RenderTextureTiled")]
    pub fn draw_tiled(
        &self,
        tex: impl Into<TextureRef>,
        src: Option<&SDL_FRect>,
        scale: f32,
        dst: Option<&SDL_FRect>,
    ) -> SdlResult {
        to_result(unsafe {
            SDL_RenderTextureTiled(
                self.handle.as_ptr(),
                tex.into().handle.as_ptr(),
                util::opt2ptr(src),
                scale,
                util::opt2ptr(dst),
            )
        })
    }

    #[doc(alias = "SDL_RenderTexture9Grid")]
    pub fn draw_9grid(
        &self,
        tex: impl Into<TextureRef>,
        src: Option<&SDL_FRect>,
        width_left: f32,
        width_right: f32,
        width_top: f32,
        width_bottom: f32,
        scale: f32,
        dst: Option<&SDL_FRect>,
    ) -> SdlResult {
        to_result(unsafe {
            SDL_RenderTexture9Grid(
                self.handle.as_ptr(),
                tex.into().handle.as_ptr(),
                util::opt2ptr(src),
                width_left,
                width_right,
                width_top,
                width_bottom,
                scale,
                util::opt2ptr(dst),
            )
        })
    }

    #[doc(alias = "SDL_SetRenderTarget")]
    pub fn set_target(&self, tgt: impl Into<TextureRef>) -> SdlResult {
        to_result(unsafe { SDL_SetRenderTarget(self.handle.as_ptr(), tgt.into().handle.as_ptr()) })
    }

    #[doc(alias = "SDL_SetRenderTarget")]
    pub fn reset_target(&self) -> SdlResult {
        to_result(unsafe { SDL_SetRenderTarget(self.handle.as_ptr(), std::ptr::null_mut()) })
    }
}

pub struct Renderer {
    pub(crate) inner: RendererRef,
}

impl Renderer {
    fn from_ptr(ptr: *mut SDL_Renderer) -> SdlResult<Renderer> {
        match NonNull::new(ptr) {
            None => Err(error::get()),
            Some(handle) => Ok(Renderer {
                inner: RendererRef { handle },
            }),
        }
    }

    #[doc(alias = "SDL_CreateRenderer")]
    pub fn new(wnd: impl Into<WindowRef>) -> SdlResult<Renderer> {
        Self::from_ptr(unsafe { SDL_CreateRenderer(wnd.into().handle.as_ptr(), std::ptr::null()) })
    }
}

impl Deref for Renderer {
    type Target = RendererRef;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Renderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl From<&Renderer> for RendererRef {
    fn from(value: &Renderer) -> Self {
        value.inner
    }
}

impl Drop for Renderer {
    #[doc(alias = "SDL_DestroyRenderer")]
    fn drop(&mut self) {
        unsafe {
            SDL_DestroyRenderer(self.inner.handle.as_ptr());
        }
    }
}
