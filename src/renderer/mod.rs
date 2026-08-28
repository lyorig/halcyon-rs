//! SDL's 2D rendering API wrapper.
//!
//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryRender)):
//! - [ ] SDL_AddVulkanRenderSemaphores
//! - [ ] SDL_ConvertEventToRenderCoordinates
//! - [x] SDL_CreateGPURenderer
//! - [x] SDL_CreateRenderer
//! - [x] SDL_CreateRendererWithProperties
//! - [ ] SDL_CreateSoftwareRenderer
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
//! - [x] SDL_GetRendererProperties
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
//! - [x] SDL_RenderFillRect
//! - [x] SDL_RenderFillRects
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
//! - [ ] SDL_RenderTexture9GridTiled
//! - [x] SDL_RenderTextureAffine
//! - [ ] SDL_RenderTextureRotated
//! - [x] SDL_RenderTextureTiled
//! - [ ] SDL_RenderViewportSet
//! - [ ] SDL_SetRenderClipRect
//! - [ ] SDL_SetRenderColorScale
//! - [x] SDL_SetRenderDrawBlendMode
//! - [x] SDL_SetRenderDrawColor
//! - [x] SDL_SetRenderDrawColorFloat
//! - [x] SDL_SetRenderGPUState
//! - [ ] SDL_SetRenderLogicalPresentation
//! - [ ] SDL_SetRenderScale
//! - [x] SDL_SetRenderTarget
//! - [ ] SDL_SetRenderTextureAddressMode
//! - [ ] SDL_SetRenderViewport
//! - [x] SDL_SetRenderVSync

use std::{ffi::CStr, mem::MaybeUninit};

use sdl3_sys::render::*;

use crate::{
    Result,
    color::{RgbaF32, RgbaU8},
    gpu::{Device, RenderState},
    mod_reexport,
    pixels::BlendMode,
    properties::{Properties, PropertiesHandle},
    rect::{PointF32, PointI32, RectF32, RectI32},
    resource::Ref,
    resource_new,
    surface::Surface,
    texture::{Texture, TextureHandle},
    traits,
    util::{opt2ptr, to_result},
    window::{Window, WindowHandle},
};

mod_reexport!(builder);
mod_reexport!(properties);

resource_new!(SDL_Renderer, Renderer, SDL_DestroyRenderer);

impl RendererHandle {
    #[doc(alias = "SDL_GetRendererName")]
    pub fn name(&self) -> &str {
        unsafe {
            // SAFETY: Renderer name strings are all UTF-8.
            str::from_utf8_unchecked(
                CStr::from_ptr(SDL_GetRendererName(self.handle.as_ptr())).to_bytes(),
            )
        }
    }

    /// Read-only properties of this renderer, as documented by
    /// [`SDL_GetRendererProperties`](https://wiki.libsdl.org/SDL3/SDL_GetRendererProperties).
    ///
    /// Covers the generic properties plus the D3D9, D3D11, D3D12, Vulkan and
    /// GPU-renderer backends. Not covered: the Metal backend
    /// (`SDL_PROP_RENDERER_METAL_*`), which sdl3-sys does not expose.
    #[doc(alias = "SDL_GetRendererProperties")]
    pub fn properties(&self) -> RendererProperties<'_> {
        let id = unsafe { SDL_GetRendererProperties(self.handle.as_ptr()) };
        let handle =
            PropertiesHandle::from_id(id).expect("A valid renderer should always have a handle");

        let r = unsafe { Ref::from_handle(handle) };
        RendererProperties::new(r)
    }

    #[doc(alias = "SDL_GetRenderWindow")]
    pub fn window(&self) -> Ref<'_, Window> {
        let handle = WindowHandle::from_ptr(unsafe { SDL_GetRenderWindow(self.handle.as_ptr()) })
            .expect("Renderer has no associated window");
        unsafe { Ref::from_handle(handle) }
    }

    #[doc(alias = "SDL_GetRenderTarget")]
    pub fn target(&self) -> Option<Ref<'_, Texture>> {
        TextureHandle::from_ptr(unsafe { SDL_GetRenderTarget(self.handle.as_ptr()) })
            .map(|h| unsafe { Ref::from_handle(h) })
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

    #[doc(alias = "SDL_RenderReadPixels")]
    pub fn read_target(&self) -> Result<Surface> {
        Surface::from_ptr(unsafe { SDL_RenderReadPixels(self.handle.as_ptr(), std::ptr::null()) })
    }

    #[doc(alias = "SDL_RenderReadPixels")]
    pub fn read_target_area(&self, area: RectI32) -> Result<Surface> {
        Surface::from_ptr(unsafe {
            SDL_RenderReadPixels(self.handle.as_ptr(), (&raw const area).cast())
        })
    }

    #[doc(alias = "SDL_RenderClear")]
    pub fn clear(&self) -> Result {
        to_result(unsafe { SDL_RenderClear(self.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_RenderPresent")]
    pub fn present(&self) -> Result {
        to_result(unsafe { SDL_RenderPresent(self.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_FlushRenderer")]
    pub fn flush(&self) -> Result {
        to_result(unsafe { SDL_FlushRenderer(self.handle.as_ptr()) })
    }

    /// This function is a direct wrapper of SDL's [`SDL_RenderTexture`];
    /// see [`DrawBuilder`] for a neater way to draw to a renderer.
    #[doc(alias = "SDL_RenderTexture")]
    pub fn draw(&self, tex: Ref<Texture>, src: Option<&RectF32>, dst: Option<&RectF32>) -> Result {
        to_result(unsafe {
            SDL_RenderTexture(
                self.handle.as_ptr(),
                tex.handle.as_ptr(),
                opt2ptr(src).cast(),
                opt2ptr(dst).cast(),
            )
        })
    }

    #[doc(alias = "SDL_RenderTextureAffine")]
    pub fn draw_affine(
        &self,
        tex: Ref<Texture>,
        src: Option<&RectF32>,
        origin: Option<&PointF32>,
        right: Option<&PointF32>,
        down: Option<&PointF32>,
    ) -> Result {
        to_result(unsafe {
            SDL_RenderTextureAffine(
                self.handle.as_ptr(),
                tex.handle.as_ptr(),
                opt2ptr(src).cast(),
                opt2ptr(origin).cast(),
                opt2ptr(right).cast(),
                opt2ptr(down).cast(),
            )
        })
    }

    #[doc(alias = "SDL_RenderTextureTiled")]
    pub fn draw_tiled(
        &self,
        tex: Ref<Texture>,
        src: Option<&RectF32>,
        scale: f32,
        dst: Option<&RectF32>,
    ) -> Result {
        to_result(unsafe {
            SDL_RenderTextureTiled(
                self.handle.as_ptr(),
                tex.handle.as_ptr(),
                opt2ptr(src).cast(),
                scale,
                opt2ptr(dst).cast(),
            )
        })
    }

    #[doc(alias = "SDL_RenderTexture9Grid")]
    pub fn draw_9grid(
        &self,
        tex: Ref<Texture>,
        src: Option<&RectF32>,
        (width_left, width_right, width_top, width_bottom): (f32, f32, f32, f32),
        scale: f32,
        dst: Option<&RectF32>,
    ) -> Result {
        to_result(unsafe {
            SDL_RenderTexture9Grid(
                self.handle.as_ptr(),
                tex.handle.as_ptr(),
                opt2ptr(src).cast(),
                width_left,
                width_right,
                width_top,
                width_bottom,
                scale,
                opt2ptr(dst).cast(),
            )
        })
    }

    #[doc(alias = "SDL_RenderLine")]
    pub fn draw_line(&self, start: PointF32, end: PointF32) -> Result {
        to_result(unsafe { SDL_RenderLine(self.handle.as_ptr(), start.x, start.y, end.x, end.y) })
    }

    #[doc(alias = "SDL_RenderLine")]
    pub fn draw_line_with(&self, start: PointF32, end: PointF32, col: RgbaF32) -> Result {
        let old = self.xchg_draw_color_f32(col);
        let ret = self.draw_line(start, end);
        self.set_draw_color_f32(old);

        ret
    }

    #[doc(alias = "SDL_RenderLines")]
    pub fn draw_lines(&self, lines: &[PointF32]) -> Result {
        to_result(unsafe {
            SDL_RenderLines(
                self.handle.as_ptr(),
                lines.as_ptr().cast(),
                lines.len() as i32,
            )
        })
    }

    #[doc(alias = "SDL_RenderLines")]
    pub fn draw_lines_with(&self, lines: &[PointF32], col: RgbaF32) -> Result {
        let old = self.xchg_draw_color_f32(col);
        let ret = self.draw_lines(lines);
        self.set_draw_color_f32(old);

        ret
    }

    #[doc(alias = "SDL_RenderPoint")]
    pub fn draw_point(&self, pos: PointF32) -> Result {
        to_result(unsafe { SDL_RenderPoint(self.handle.as_ptr(), pos.x, pos.y) })
    }

    #[doc(alias = "SDL_RenderPoint")]
    pub fn draw_point_with(&self, pos: PointF32, col: RgbaF32) -> Result {
        let old = self.xchg_draw_color_f32(col);
        let ret = self.draw_point(pos);
        self.set_draw_color_f32(old);

        ret
    }

    #[doc(alias = "SDL_RenderPoints")]
    pub fn draw_points(&self, points: &[PointF32]) -> Result {
        to_result(unsafe {
            SDL_RenderPoints(
                self.handle.as_ptr(),
                points.as_ptr().cast(),
                points.len() as i32,
            )
        })
    }

    #[doc(alias = "SDL_RenderPoints")]
    pub fn draw_points_with(&self, points: &[PointF32], col: RgbaF32) -> Result {
        let old = self.xchg_draw_color_f32(col);
        let ret = self.draw_points(points);
        self.set_draw_color_f32(old);

        ret
    }

    #[doc(alias = "SDL_RenderRect")]
    pub fn draw_rect(&self, rect: RectF32) -> Result {
        to_result(unsafe { SDL_RenderRect(self.handle.as_ptr(), (&raw const rect).cast()) })
    }

    #[doc(alias = "SDL_RenderRect")]
    pub fn draw_rect_with(&self, rect: RectF32, col: RgbaF32) -> Result {
        let old = self.xchg_draw_color_f32(col);
        let ret = self.draw_rect(rect);
        self.set_draw_color_f32(old);

        ret
    }

    #[doc(alias = "SDL_RenderRect")]
    pub fn draw_target_outline(&self) -> Result {
        to_result(unsafe { SDL_RenderRect(self.handle.as_ptr(), std::ptr::null()) })
    }

    #[doc(alias = "SDL_RenderRect")]
    pub fn draw_target_outline_with(&self, col: RgbaF32) -> Result {
        let old = self.xchg_draw_color_f32(col);
        let ret = self.draw_target_outline();
        self.set_draw_color_f32(old);

        ret
    }

    #[doc(alias = "SDL_RenderRects")]
    pub fn draw_rects(&self, rects: &[RectF32]) -> Result {
        to_result(unsafe {
            SDL_RenderRects(
                self.handle.as_ptr(),
                rects.as_ptr().cast(),
                rects.len() as i32,
            )
        })
    }

    #[doc(alias = "SDL_RenderRects")]
    pub fn draw_rects_with(&self, rects: &[RectF32], col: RgbaF32) -> Result {
        let old = self.xchg_draw_color_f32(col);
        let ret = self.draw_rects(rects);
        self.set_draw_color_f32(old);

        ret
    }

    #[doc(alias = "SDL_RenderFillRect")]
    pub fn fill_target(&self) -> Result {
        to_result(unsafe { SDL_RenderFillRect(self.handle.as_ptr(), std::ptr::null()) })
    }

    #[doc(alias = "SDL_RenderFillRect")]
    pub fn fill_target_with(&self, col: RgbaF32) -> Result {
        let old = self.xchg_draw_color_f32(col);
        let ret = self.fill_target();
        self.set_draw_color_f32(old);

        ret
    }

    #[doc(alias = "SDL_RenderFillRect")]
    pub fn fill_rect(&self, rect: RectF32) -> Result {
        to_result(unsafe { SDL_RenderFillRect(self.handle.as_ptr(), (&raw const rect).cast()) })
    }

    #[doc(alias = "SDL_RenderFillRect")]
    pub fn fill_rect_with(&self, rect: RectF32, col: RgbaF32) -> Result {
        let old = self.xchg_draw_color_f32(col);
        let ret = self.fill_rect(rect);
        self.set_draw_color_f32(old);

        ret
    }

    #[doc(alias = "SDL_RenderFillRects")]
    pub fn fill_rects(&self, rects: &[RectF32]) -> Result {
        to_result(unsafe {
            SDL_RenderFillRects(
                self.handle.as_ptr(),
                rects.as_ptr().cast(),
                rects.len() as i32,
            )
        })
    }

    #[doc(alias = "SDL_RenderFillRects")]
    pub fn fill_rects_with(&self, rects: &[RectF32], col: RgbaF32) -> Result {
        let old = self.xchg_draw_color_f32(col);
        let ret = self.fill_rects(rects);
        self.set_draw_color_f32(old);

        ret
    }

    /// For use with [`RendererHandle::xchg_target`]. Otherwise, prefer using
    /// [`RendererHandle::set_target`] or [`RendererHandle::reset_target`].
    ///
    /// # Safety
    /// If the parameter is `Some(tex)`, ensure `tex` lives for as long as it's
    /// used as the target texture.
    #[doc(alias = "SDL_SetRenderTarget")]
    pub fn set_target_opt(&self, tgt: Option<Ref<Texture>>) -> Result {
        to_result(unsafe {
            SDL_SetRenderTarget(
                self.handle.as_ptr(),
                match tgt {
                    Some(h) => h.handle.as_ptr(),
                    None => std::ptr::null_mut(),
                },
            )
        })
    }

    #[doc(alias = "SDL_SetRenderTarget")]
    pub fn set_target(&self, tgt: Ref<Texture>) -> Result {
        self.set_target_opt(Some(tgt))
    }

    #[doc(alias = "SDL_SetRenderTarget")]
    pub fn reset_target(&self) -> Result {
        self.set_target_opt(None)
    }

    pub fn xchg_target(&self, tgt: Ref<Texture>) -> Result<Option<Ref<'_, Texture>>> {
        let old = self.target();
        self.set_target(tgt)?;
        Ok(old)
    }

    /// Quoting documentation for [`SDL_SetRenderVSync`]:
    /// Not every value is supported by every driver, so you should check
    /// the return value to see whether the requested setting is supported.
    ///
    /// Can be used with [`Renderer::VSYNC_ADAPTIVE`] and [`Renderer::VSYNC_DISABLED`].
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

    pub fn xchg_draw_color_u8(&self, col: RgbaU8) -> RgbaU8 {
        let old = self.draw_color_u8();
        self.set_draw_color_u8(col);
        old
    }

    pub fn xchg_draw_color_f32(&self, col: RgbaF32) -> RgbaF32 {
        let old = self.draw_color_f32();
        self.set_draw_color_f32(col);
        old
    }

    pub fn set_render_state(&self, rs: Ref<RenderState>) -> Result {
        to_result(unsafe { SDL_SetGPURenderState(self.as_ptr(), rs.as_ptr()) })
    }

    #[doc(alias = "SDL_SetGPURenderState")]
    pub fn clear_render_state(&self) -> Result {
        to_result(unsafe { SDL_SetGPURenderState(self.as_ptr(), std::ptr::null_mut()) })
    }
}

impl traits::BlendMode for RendererHandle {
    #[doc(alias = "SDL_GetRenderDrawBlendMode")]
    fn blend_mode(&self) -> BlendMode {
        let mut ret = MaybeUninit::uninit();
        unsafe {
            SDL_GetRenderDrawBlendMode(self.handle.as_ptr(), ret.as_mut_ptr());
            ret.assume_init().into()
        }
    }

    #[doc(alias = "SDL_SetRenderDrawBlendMode")]
    fn set_blend_mode(&self, bm: BlendMode) {
        unsafe {
            SDL_SetRenderDrawBlendMode(self.handle.as_ptr(), bm.into());
        }
    }
}

impl Renderer {
    pub const VSYNC_DISABLED: i32 = SDL_RENDERER_VSYNC_DISABLED;
    pub const VSYNC_ADAPTIVE: i32 = SDL_RENDERER_VSYNC_ADAPTIVE;

    /// Bind the builder to an existing property group.
    ///
    /// The renderer creation properties (`SDL_PROP_RENDERER_CREATE_*`)
    /// never collide with the window or GPU device ones, so a single
    /// [`Properties`] can be shared between the three builders.
    pub fn builder(props: Ref<Properties>) -> RendererBuilder {
        RendererBuilder::new(props)
    }

    #[doc(alias = "SDL_CreateRenderer")]
    pub fn new(wnd: Ref<Window>, name: Option<&CStr>) -> Result<Renderer> {
        Self::from_ptr(unsafe {
            SDL_CreateRenderer(
                wnd.handle.as_ptr(),
                name.map_or(std::ptr::null(), CStr::as_ptr),
            )
        })
    }

    #[doc(alias = "SDL_CreateGPURenderer")]
    pub fn new_gpu(device: Ref<Device>, wnd: Ref<Window>) -> Result<Self> {
        Self::from_ptr(unsafe { SDL_CreateGPURenderer(device.as_ptr(), wnd.as_ptr()) })
    }

    #[doc(alias = "SDL_GetNumRenderDrivers")]
    pub fn num_drivers() -> i32 {
        unsafe { SDL_GetNumRenderDrivers() }
    }
}

/// A builder-like struct intended an an alternative
/// to `RendererHandle::draw()`.
pub struct DrawBuilder<'rnd, 'tex, 'rct> {
    renderer: Ref<'rnd, Renderer>,

    texture: Ref<'tex, Texture>,
    src: Option<&'rct RectF32>,
    dst: Option<&'rct RectF32>,
}

impl<'rnd, 'tex, 'rct> DrawBuilder<'rnd, 'tex, 'rct> {
    pub fn new(rnd: Ref<'rnd, Renderer>, tex: Ref<'tex, Texture>) -> Self {
        Self {
            renderer: rnd,
            texture: tex,
            src: None,
            dst: None,
        }
    }

    pub fn from(&mut self, src: &'rct RectF32) -> &mut Self {
        self.src = Some(src);
        self
    }

    pub fn to(&mut self, dst: &'rct RectF32) -> &mut Self {
        self.dst = Some(dst);
        self
    }

    pub fn draw(&self) -> Result {
        self.renderer.draw(self.texture, self.src, self.dst)
    }
}
