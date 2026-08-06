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

    pub fn name(&mut self, value: &CStr) -> &mut Self {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_RENDERER_CREATE_NAME_STRING) };
        _ = self.inner.set_string(cstr, value.as_ptr());

        self
    }

    /// The window where rendering is displayed. Mutually exclusive with
    /// [`RendererBuilder::surface`].
    pub fn window(&mut self, value: Ref<Window>) -> &mut Self {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_RENDERER_CREATE_WINDOW_POINTER) };
        _ = self.inner.set_pointer(cstr, value.handle.as_ptr().cast());

        self
    }

    pub fn surface(&mut self, value: Ref<Surface>) -> &mut Self {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_RENDERER_CREATE_SURFACE_POINTER) };
        _ = self.inner.set_pointer(cstr, value.handle.as_ptr().cast());

        self
    }

    pub fn colorspace(&mut self, value: SDL_Colorspace) -> &mut Self {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_RENDERER_CREATE_OUTPUT_COLORSPACE_NUMBER) };
        _ = self.inner.set_number(cstr, value.0.into());

        self
    }

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
