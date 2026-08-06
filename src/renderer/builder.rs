use std::ffi::CStr;

use sdl3_sys::{pixels::SDL_Colorspace, render::*};

use crate::{
    Result, properties::Properties, renderer::Renderer, resource::Ref, surface::Surface,
    window::Window,
};

pub struct RendererBuilder<'a> {
    inner: Ref<'a, Properties>,
}

impl RendererBuilder<'_> {
    pub(super) fn new(inner: Ref<Properties>) -> RendererBuilder {
        RendererBuilder { inner }
    }

    /// The name of the rendering driver to use, if a specific one is desired.
    #[doc(alias = "SDL_PROP_RENDERER_CREATE_NAME_STRING")]
    pub fn name(&mut self, value: &CStr) -> &mut Self {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_RENDERER_CREATE_NAME_STRING) };
        _ = self.inner.set_string(cstr, value.as_ptr());

        self
    }

    /// The window where rendering is displayed. Required if this isn't a
    /// software renderer using a surface. Mutually exclusive with
    /// [`RendererBuilder::surface`].
    #[doc(alias = "SDL_PROP_RENDERER_CREATE_WINDOW_POINTER")]
    pub fn window(&mut self, value: Ref<Window>) -> &mut Self {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_RENDERER_CREATE_WINDOW_POINTER) };
        _ = self.inner.set_pointer(cstr, value.handle.as_ptr().cast());

        self
    }

    /// The surface where rendering is displayed, if you want a software
    /// renderer without a window.
    #[doc(alias = "SDL_PROP_RENDERER_CREATE_SURFACE_POINTER")]
    pub fn surface(&mut self, value: Ref<Surface>) -> &mut Self {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_RENDERER_CREATE_SURFACE_POINTER) };
        _ = self.inner.set_pointer(cstr, value.handle.as_ptr().cast());

        self
    }

    /// An [`SDL_Colorspace`] value describing the colorspace for output to the
    /// display. Defaults to `SDL_COLORSPACE_SRGB`.
    #[doc(alias = "SDL_PROP_RENDERER_CREATE_OUTPUT_COLORSPACE_NUMBER")]
    pub fn colorspace(&mut self, value: SDL_Colorspace) -> &mut Self {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_RENDERER_CREATE_OUTPUT_COLORSPACE_NUMBER) };
        _ = self.inner.set_number(cstr, value.0.into());

        self
    }

    /// Non-zero if you want present synchronized with the refresh rate. This
    /// property can take any value that is supported by `SDL_SetRenderVSync`
    /// for the renderer.
    #[doc(alias = "SDL_PROP_RENDERER_CREATE_PRESENT_VSYNC_NUMBER")]
    pub fn vsync(&mut self, value: i64) -> &mut Self {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_RENDERER_CREATE_PRESENT_VSYNC_NUMBER) };
        _ = self.inner.set_number(cstr, value);

        self
    }

    /// Build the renderer.
    ///
    /// This doesn't require a subsystem parameter, as the [`Window`]
    /// you're creating this with needs one, proving the subsystem has been
    /// initialized.
    #[doc(alias = "SDL_CreateRendererWithProperties")]
    pub fn build(&self) -> Result<Renderer> {
        Renderer::from_ptr(unsafe { SDL_CreateRendererWithProperties(self.inner.id()) })
    }
}
