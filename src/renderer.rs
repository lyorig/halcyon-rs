//! SDL's 2D rendering API wrapper.
//!
//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryRender)):
//! - [ ] SDL_AddVulkanRenderSemaphores
//! - [ ] SDL_ConvertEventToRenderCoordinates
//! - [ ] SDL_CreateGPURenderer
//! - [ ] SDL_CreateGPURenderState
//! - [x] SDL_CreateRenderer
//! - [x] SDL_CreateRendererWithProperties
//! - [ ] SDL_CreateSoftwareRenderer
//! - [ ] SDL_CreateWindowAndRenderer
//! - [ ] SDL_DestroyGPURenderState
//! - [x] SDL_DestroyRenderer
//! - [ ] SDL_FlushRenderer
//! - [ ] SDL_GetCurrentRenderOutputSize
//! - [ ] SDL_GetNumRenderDrivers
//! - [ ] SDL_GetRenderClipRect
//! - [ ] SDL_GetRenderColorScale
//! - [ ] SDL_GetRenderDrawBlendMode
//! - [ ] SDL_GetRenderDrawColor
//! - [ ] SDL_GetRenderDrawColorFloat
//! - [ ] SDL_GetRenderDriver
//! - [ ] SDL_GetRenderer
//! - [ ] SDL_GetRendererFromTexture
//! - [ ] SDL_GetRendererName
//! - [ ] SDL_GetRendererProperties
//! - [ ] SDL_GetRenderLogicalPresentation
//! - [ ] SDL_GetRenderLogicalPresentationRect
//! - [ ] SDL_GetRenderMetalCommandEncoder
//! - [ ] SDL_GetRenderMetalLayer
//! - [ ] SDL_GetRenderOutputSize
//! - [ ] SDL_GetRenderSafeArea
//! - [ ] SDL_GetRenderScale
//! - [ ] SDL_GetRenderTarget
//! - [ ] SDL_GetRenderTextureAddressMode
//! - [ ] SDL_GetRenderViewport
//! - [ ] SDL_GetRenderVSync
//! - [ ] SDL_GetRenderWindow
//! - [ ] SDL_RenderClear
//! - [ ] SDL_RenderClipEnabled
//! - [ ] SDL_RenderCoordinatesFromWindow
//! - [ ] SDL_RenderCoordinatesToWindow
//! - [ ] SDL_RenderDebugText
//! - [ ] SDL_RenderDebugTextFormat
//! - [ ] SDL_RenderFillRect
//! - [ ] SDL_RenderFillRects
//! - [ ] SDL_RenderGeometry
//! - [ ] SDL_RenderGeometryRaw
//! - [ ] SDL_RenderLine
//! - [ ] SDL_RenderLines
//! - [ ] SDL_RenderPoint
//! - [ ] SDL_RenderPoints
//! - [ ] SDL_RenderPresent
//! - [ ] SDL_RenderReadPixels
//! - [ ] SDL_RenderRect
//! - [ ] SDL_RenderRects
//! - [ ] SDL_RenderTexture
//! - [ ] SDL_RenderTexture9Grid
//! - [ ] SDL_RenderTexture9GridTiled
//! - [ ] SDL_RenderTextureAffine
//! - [ ] SDL_RenderTextureRotated
//! - [ ] SDL_RenderTextureTiled
//! - [ ] SDL_RenderViewportSet
//! - [ ] SDL_SetGPURenderStateFragmentUniforms
//! - [ ] SDL_SetRenderClipRect
//! - [ ] SDL_SetRenderColorScale
//! - [ ] SDL_SetRenderDrawBlendMode
//! - [ ] SDL_SetRenderDrawColor
//! - [ ] SDL_SetRenderDrawColorFloat
//! - [ ] SDL_SetRenderGPUState
//! - [ ] SDL_SetRenderLogicalPresentation
//! - [ ] SDL_SetRenderScale
//! - [ ] SDL_SetRenderTarget
//! - [ ] SDL_SetRenderTextureAddressMode
//! - [ ] SDL_SetRenderViewport
//! - [ ] SDL_SetRenderVSync

use std::ffi::{CStr, c_void};

use sdl3_sys::{
    pixels::SDL_Colorspace,
    rect::{SDL_FPoint, SDL_FRect},
    render::*,
};

use crate::{
    defs::SdlResult,
    properties::Properties,
    resource,
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

resource!(Renderer, RendererRef, SDL_Renderer, SDL_DestroyRenderer);

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

impl Renderer {
    #[doc(alias = "SDL_CreateRenderer")]
    pub fn new(wnd: impl Into<WindowRef>) -> SdlResult<Renderer> {
        Self::from_ptr(unsafe { SDL_CreateRenderer(wnd.into().handle.as_ptr(), std::ptr::null()) })
    }
}
