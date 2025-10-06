//! SDL's 2D rendering API wrapper.
//!
//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryRender)):
//! - [ ] SDL_AddVulkanRenderSemaphores
//! - [ ] SDL_ConvertEventToRenderCoordinates
//! - [ ] SDL_CreateGPURenderer
//! - [ ] SDL_CreateGPURenderState
//! - [x] SDL_CreateRenderer
//! - [x] SDL_CreateRendererWithProperties
//! - [x] SDL_CreateSoftwareRenderer
//! - [ ] SDL_DestroyGPURenderState
//! - [x] SDL_DestroyRenderer
//! - [x] SDL_FlushRenderer
//! - [x] SDL_GetCurrentRenderOutputSize
//! - [x] SDL_GetNumRenderDrivers
//! - [ ] SDL_GetRenderClipRect
//! - [ ] SDL_GetRenderColorScale
//! - [x] SDL_GetRenderDrawBlendMode
//! - [x] SDL_GetRenderDrawColor
//! - [x] SDL_GetRenderDrawColorFloat
//! - [ ] SDL_GetRenderDriver
//! - [x] SDL_GetRendererName
//! - [ ] SDL_GetRendererProperties
//! - [ ] SDL_GetRenderLogicalPresentation
//! - [ ] SDL_GetRenderLogicalPresentationRect
//! - [ ] SDL_GetRenderMetalCommandEncoder
//! - [ ] SDL_GetRenderMetalLayer
//! - [x] SDL_GetRenderOutputSize
//! - [ ] SDL_GetRenderSafeArea
//! - [ ] SDL_GetRenderScale
//! - [x] SDL_GetRenderTarget
//! - [ ] SDL_GetRenderTextureAddressMode
//! - [ ] SDL_GetRenderViewport
//! - [x] SDL_GetRenderVSync
//! - [x] SDL_GetRenderWindow
//! - [x] SDL_RenderClear
//! - [ ] SDL_RenderClipEnabled
//! - [ ] SDL_RenderCoordinatesFromWindow
//! - [ ] SDL_RenderCoordinatesToWindow
//! - [ ] SDL_RenderDebugText
//! - [ ] SDL_RenderDebugTextFormat
//! - [ ] SDL_RenderFillRect
//! - [ ] SDL_RenderFillRects
//! - [ ] SDL_RenderGeometry
//! - [ ] SDL_RenderGeometryRaw
//! - [x] SDL_RenderLine
//! - [x] SDL_RenderLines
//! - [x] SDL_RenderPoint
//! - [x] SDL_RenderPoints
//! - [x] SDL_RenderPresent
//! - [x] SDL_RenderReadPixels
//! - [x] SDL_RenderRect
//! - [x] SDL_RenderRects
//! - [x] SDL_RenderTexture
//! - [x] SDL_RenderTexture9Grid
//! - [x] SDL_RenderTexture9GridTiled
//! - [x] SDL_RenderTextureAffine
//! - [x] SDL_RenderTextureRotated
//! - [x] SDL_RenderTextureTiled
//! - [ ] SDL_RenderViewportSet
//! - [ ] SDL_SetGPURenderStateFragmentUniforms
//! - [ ] SDL_SetRenderClipRect
//! - [ ] SDL_SetRenderColorScale
//! - [x] SDL_SetRenderDrawBlendMode
//! - [x] SDL_SetRenderDrawColor
//! - [x] SDL_SetRenderDrawColorFloat
//! - [ ] SDL_SetRenderGPUState
//! - [ ] SDL_SetRenderLogicalPresentation
//! - [ ] SDL_SetRenderScale
//! - [x] SDL_SetRenderTarget
//! - [ ] SDL_SetRenderTextureAddressMode
//! - [ ] SDL_SetRenderViewport
//! - [x] SDL_SetRenderVSync

use std::{
    ffi::{CStr, c_void},
    mem::MaybeUninit,
};

use sdl3_sys::{blendmode::SDL_BlendMode, pixels::SDL_Colorspace, render::*};

use crate::{
    color::{RgbaF32, RgbaU8},
    defs::SdlResult,
    properties::Properties,
    rect::{PointF32, PointI32, RectF32, RectI32},
    resource,
    surface::{Surface, SurfaceRef},
    texture::TextureRef,
    util::{opt2ptr, to_result},
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

resource!(Renderer);

impl RendererRef {
    #[doc(alias = "SDL_GetRendererName")]
    pub fn name(&self) -> &str {
        unsafe {
            // SAFETY: Renderer name strings are stored in a static array.
            std::str::from_utf8_unchecked(
                CStr::from_ptr(SDL_GetRendererName(self.handle.as_ptr())).to_bytes(),
            )
        }
    }

    /// This function doesn't return an `Option`, as all renderers should have
    /// an associated window. If that's somehow violated, the program will panic.
    #[doc(alias = "SDL_GetRenderWindow")]
    pub fn window(&self) -> WindowRef {
        WindowRef::from_ptr(unsafe { SDL_GetRenderWindow(self.handle.as_ptr()) })
            .expect("Renderer has no associated window")
    }

    #[doc(alias = "SDL_GetRenderTarget")]
    pub fn target(&self) -> Option<TextureRef> {
        TextureRef::from_ptr(unsafe { SDL_GetRenderTarget(self.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_GetRenderVSync")]
    pub fn vsync(&self) -> i32 {
        let mut ret = MaybeUninit::uninit();
        unsafe {
            SDL_GetRenderVSync(self.handle.as_ptr(), ret.as_mut_ptr());
            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_GetRenderOutputSize")]
    pub fn output_size(&self) -> PointI32 {
        let mut ret = MaybeUninit::<PointI32>::uninit();
        let ptr = ret.as_mut_ptr();

        unsafe {
            SDL_GetRenderOutputSize(self.handle.as_ptr(), &raw mut (*ptr).x, &raw mut (*ptr).y);
            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_GetCurrentRenderOutputSize")]
    pub fn target_output_size(&self) -> PointI32 {
        let mut ret = MaybeUninit::<PointI32>::uninit();
        let ptr = ret.as_mut_ptr();

        unsafe {
            SDL_GetCurrentRenderOutputSize(
                self.handle.as_ptr(),
                &raw mut (*ptr).x,
                &raw mut (*ptr).y,
            );
            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_GetRenderDrawColor")]
    pub fn draw_color_u8(&self) -> RgbaU8 {
        let mut ret = MaybeUninit::<RgbaU8>::uninit();
        let ptr = ret.as_mut_ptr();

        // SAFETY: This function only reads struct fields.
        unsafe {
            SDL_GetRenderDrawColor(
                self.handle.as_ptr(),
                &raw mut (*ptr).rgb.r,
                &raw mut (*ptr).rgb.g,
                &raw mut (*ptr).rgb.b,
                &raw mut (*ptr).a,
            );

            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_GetRenderDrawColorFloat")]
    pub fn draw_color_f32(&self) -> RgbaF32 {
        let mut ret = MaybeUninit::<RgbaF32>::uninit();
        let ptr = ret.as_mut_ptr();

        // SAFETY: This function only reads struct fields.
        unsafe {
            SDL_GetRenderDrawColorFloat(
                self.handle.as_ptr(),
                &raw mut (*ptr).rgb.r,
                &raw mut (*ptr).rgb.g,
                &raw mut (*ptr).rgb.b,
                &raw mut (*ptr).a,
            );

            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_GetRenderDrawBlendMode")]
    pub fn blend_mode(&self) -> SDL_BlendMode {
        let mut ret = MaybeUninit::uninit();
        unsafe {
            SDL_GetRenderDrawBlendMode(self.handle.as_ptr(), ret.as_mut_ptr());
            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_RenderReadPixels")]
    pub fn read_target(&self) -> SdlResult<Surface> {
        Surface::from_ptr(unsafe { SDL_RenderReadPixels(self.handle.as_ptr(), std::ptr::null()) })
    }

    #[doc(alias = "SDL_RenderReadPixels")]
    pub fn read_target_area(&self, area: RectI32) -> SdlResult<Surface> {
        Surface::from_ptr(unsafe {
            SDL_RenderReadPixels(self.handle.as_ptr(), (&raw const area).cast())
        })
    }

    #[doc(alias = "SDL_RenderClear")]
    pub fn clear(&self) -> SdlResult {
        to_result(unsafe { SDL_RenderClear(self.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_RenderPresent")]
    pub fn present(&self) -> SdlResult {
        to_result(unsafe { SDL_RenderPresent(self.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_FlushRenderer")]
    pub fn flush(&self) -> SdlResult {
        to_result(unsafe { SDL_FlushRenderer(self.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_RenderTexture")]
    pub fn draw(
        &self,
        tex: impl Into<TextureRef>,
        src: Option<&RectF32>,
        dst: Option<&RectF32>,
    ) -> SdlResult {
        to_result(unsafe {
            SDL_RenderTexture(
                self.handle.as_ptr(),
                tex.into().handle.as_ptr(),
                opt2ptr(src),
                opt2ptr(dst),
            )
        })
    }

    #[doc(alias = "SDL_RenderTextureAffine")]
    pub fn draw_affine(
        &self,
        tex: impl Into<TextureRef>,
        src: Option<&RectF32>,
        origin: Option<&PointF32>,
        right: Option<&PointF32>,
        down: Option<&PointF32>,
    ) -> SdlResult {
        to_result(unsafe {
            SDL_RenderTextureAffine(
                self.handle.as_ptr(),
                tex.into().handle.as_ptr(),
                opt2ptr(src),
                opt2ptr(origin),
                opt2ptr(right),
                opt2ptr(down),
            )
        })
    }

    #[doc(alias = "SDL_RenderTextureTiled")]
    pub fn draw_tiled(
        &self,
        tex: impl Into<TextureRef>,
        src: Option<&RectF32>,
        scale: f32,
        dst: Option<&RectF32>,
    ) -> SdlResult {
        to_result(unsafe {
            SDL_RenderTextureTiled(
                self.handle.as_ptr(),
                tex.into().handle.as_ptr(),
                opt2ptr(src),
                scale,
                opt2ptr(dst),
            )
        })
    }

    #[doc(alias = "SDL_RenderTexture9Grid")]
    pub fn draw_9grid(
        &self,
        tex: impl Into<TextureRef>,
        src: Option<&RectF32>,
        width_left: f32,
        width_right: f32,
        width_top: f32,
        width_bottom: f32,
        scale: f32,
        dst: Option<&RectF32>,
    ) -> SdlResult {
        to_result(unsafe {
            SDL_RenderTexture9Grid(
                self.handle.as_ptr(),
                tex.into().handle.as_ptr(),
                opt2ptr(src),
                width_left,
                width_right,
                width_top,
                width_bottom,
                scale,
                opt2ptr(dst),
            )
        })
    }

    #[doc(alias = "SDL_RenderLine")]
    pub fn draw_line(&self, start: PointF32, end: PointF32) -> SdlResult {
        to_result(unsafe { SDL_RenderLine(self.handle.as_ptr(), start.x, start.y, end.x, end.y) })
    }

    #[doc(alias = "SDL_RenderLines")]
    pub fn draw_lines(&self, lines: &[PointF32]) -> SdlResult {
        to_result(unsafe {
            SDL_RenderLines(
                self.handle.as_ptr(),
                lines.as_ptr().cast(),
                lines.len() as i32,
            )
        })
    }

    #[doc(alias = "SDL_RenderPoint")]
    pub fn draw_point(&self, pos: PointF32) -> SdlResult {
        to_result(unsafe { SDL_RenderPoint(self.handle.as_ptr(), pos.x, pos.y) })
    }

    #[doc(alias = "SDL_RenderPoints")]
    pub fn draw_points(&self, points: &[PointF32]) -> SdlResult {
        to_result(unsafe {
            SDL_RenderPoints(
                self.handle.as_ptr(),
                points.as_ptr().cast(),
                points.len() as i32,
            )
        })
    }

    #[doc(alias = "SDL_RenderRect")]
    pub fn draw_rect(&self, rect: RectF32) -> SdlResult {
        to_result(unsafe { SDL_RenderRect(self.handle.as_ptr(), (&raw const rect).cast()) })
    }

    #[doc(alias = "SDL_RenderRect")]
    pub fn draw_target_outline(&self) -> SdlResult {
        to_result(unsafe { SDL_RenderRect(self.handle.as_ptr(), std::ptr::null()) })
    }

    #[doc(alias = "SDL_RenderRects")]
    pub fn draw_rects(&self, rects: &[RectF32]) -> SdlResult {
        to_result(unsafe {
            SDL_RenderRects(
                self.handle.as_ptr(),
                rects.as_ptr().cast(),
                rects.len() as i32,
            )
        })
    }

    #[doc(alias = "SDL_RenderFillRect")]
    pub fn fill_rect(&self, rect: RectF32) -> SdlResult {
        to_result(unsafe { SDL_RenderFillRect(self.handle.as_ptr(), (&raw const rect).cast()) })
    }

    #[doc(alias = "SDL_RenderFillRects")]
    pub fn fill_rects(&self, rects: &[RectF32]) -> SdlResult {
        to_result(unsafe {
            SDL_RenderFillRects(
                self.handle.as_ptr(),
                rects.as_ptr().cast(),
                rects.len() as i32,
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

    /// Quoting documentation for `SDL_SetRenderVSync()`:
    /// Not every value is supported by every driver, so you should check
    /// the return value to see whether the requested setting is supported.
    ///
    /// Can be used with `Renderer::VSYNC_ADAPTIVE` and `Renderer::VSYNC_DISABLED`.
    #[doc(alias = "SDL_SetRenderVSync")]
    pub fn set_vsync(&self, val: i32) -> bool {
        unsafe { SDL_SetRenderVSync(self.handle.as_ptr(), val) }
    }

    #[doc(alias = "SDL_SetRenderDrawColor")]
    pub fn set_draw_color_u8(&self, rgba: RgbaU8) {
        unsafe {
            SDL_SetRenderDrawColor(
                self.handle.as_ptr(),
                rgba.rgb.r,
                rgba.rgb.g,
                rgba.rgb.b,
                rgba.a,
            );
        }
    }

    #[doc(alias = "SDL_SetRenderDrawColorFloat")]
    pub fn set_draw_color_f32(&self, rgba: RgbaF32) {
        unsafe {
            SDL_SetRenderDrawColorFloat(
                self.handle.as_ptr(),
                rgba.rgb.r,
                rgba.rgb.g,
                rgba.rgb.b,
                rgba.a,
            );
        }
    }

    #[doc(alias = "SDL_SetRenderDrawBlendMode")]
    pub fn set_blend_mode(&self, bm: SDL_BlendMode) {
        unsafe {
            SDL_SetRenderDrawBlendMode(self.handle.as_ptr(), bm);
        }
    }
}

impl Renderer {
    const VSYNC_DISABLED: i32 = SDL_RENDERER_VSYNC_DISABLED;
    const VSYNC_ADAPTIVE: i32 = SDL_RENDERER_VSYNC_ADAPTIVE;

    #[doc(alias = "SDL_CreateRenderer")]
    pub fn new(wnd: impl Into<WindowRef>, name: Option<&CStr>) -> SdlResult<Renderer> {
        Self::from_ptr(unsafe {
            SDL_CreateRenderer(
                wnd.into().handle.as_ptr(),
                name.map_or(std::ptr::null(), |n| n.as_ptr()),
            )
        })
    }

    #[doc(alias = "SDL_GetNumRenderDrivers")]
    pub fn num_drivers() -> i32 {
        unsafe { SDL_GetNumRenderDrivers() }
    }
}
