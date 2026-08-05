use std::ffi::{CStr, c_char};

use sdl3_sys::video::*;

use crate::{Result, properties::Properties, rect::PointI32, resource::Ref, window::Window};

pub struct WindowBuilder<'a> {
    pub(super) inner: Ref<'a, Properties>,
}

impl WindowBuilder<'_> {
    pub fn always_on_top(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_WINDOW_CREATE_ALWAYS_ON_TOP_BOOLEAN, value)
    }

    pub fn borderless(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_WINDOW_CREATE_BORDERLESS_BOOLEAN, value)
    }

    pub fn constrain_popup(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_WINDOW_CREATE_CONSTRAIN_POPUP_BOOLEAN, value)
    }

    pub fn ext_gfx_context(&mut self, value: bool) -> &mut Self {
        self.set_bool(
            SDL_PROP_WINDOW_CREATE_EXTERNAL_GRAPHICS_CONTEXT_BOOLEAN,
            value,
        )
    }

    pub fn focusable(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_WINDOW_CREATE_FOCUSABLE_BOOLEAN, value)
    }

    pub fn fullscreen(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_WINDOW_CREATE_FULLSCREEN_BOOLEAN, value)
    }

    pub fn height(&mut self, value: i64) -> &mut Self {
        self.set_number(SDL_PROP_WINDOW_CREATE_HEIGHT_NUMBER, value)
    }

    pub fn hidden(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_WINDOW_CREATE_HIDDEN_BOOLEAN, value)
    }

    pub fn high_pixel_density(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_WINDOW_CREATE_HIGH_PIXEL_DENSITY_BOOLEAN, value)
    }

    pub fn maximized(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_WINDOW_CREATE_MAXIMIZED_BOOLEAN, value)
    }

    pub fn menu(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_WINDOW_CREATE_MENU_BOOLEAN, value)
    }

    pub fn metal(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_WINDOW_CREATE_METAL_BOOLEAN, value)
    }

    pub fn minimized(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_WINDOW_CREATE_MINIMIZED_BOOLEAN, value)
    }

    pub fn modal(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_WINDOW_CREATE_MODAL_BOOLEAN, value)
    }

    pub fn mouse_grabbed(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_WINDOW_CREATE_MOUSE_GRABBED_BOOLEAN, value)
    }

    pub fn opengl(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_WINDOW_CREATE_OPENGL_BOOLEAN, value)
    }

    pub fn parent(&mut self, value: Ref<Window>) -> &mut Self {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_WINDOW_CREATE_PARENT_POINTER) };
        _ = self.inner.set_pointer(cstr, value.handle.as_ptr().cast());
        self
    }

    pub fn resizable(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_WINDOW_CREATE_RESIZABLE_BOOLEAN, value)
    }

    pub fn title(&mut self, value: &CStr) -> &mut Self {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_WINDOW_CREATE_TITLE_STRING) };
        _ = self.inner.set_string(cstr, value.as_ptr());
        self
    }

    pub fn transparent(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_WINDOW_CREATE_TRANSPARENT_BOOLEAN, value)
    }

    pub fn tooltip(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_WINDOW_CREATE_TOOLTIP_BOOLEAN, value)
    }

    pub fn utility(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_WINDOW_CREATE_UTILITY_BOOLEAN, value)
    }

    pub fn vulkan(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_WINDOW_CREATE_VULKAN_BOOLEAN, value)
    }

    pub fn width(&mut self, value: i64) -> &mut Self {
        self.set_number(SDL_PROP_WINDOW_CREATE_WIDTH_NUMBER, value)
    }

    pub fn x(&mut self, value: i64) -> &mut Self {
        self.set_number(SDL_PROP_WINDOW_CREATE_X_NUMBER, value)
    }

    pub fn y(&mut self, value: i64) -> &mut Self {
        self.set_number(SDL_PROP_WINDOW_CREATE_Y_NUMBER, value)
    }

    /// Utility method that calls `self.width()` and `self.height()`.
    pub fn size(&mut self, size: PointI32) -> &mut Self {
        self.width(size.x.into());
        self.height(size.y.into())
    }

    /// Utility method that calls `self.x()` and `self.y()`.
    pub fn position(&mut self, pos: PointI32) -> &mut Self {
        self.x(pos.x.into());
        self.y(pos.y.into())
    }

    /// Build the window.
    #[doc(alias = "SDL_CreateWindowWithProperties")]
    pub fn build(&self) -> Result<Window> {
        Window::from_ptr(unsafe { SDL_CreateWindowWithProperties(self.inner.id()) })
    }

    fn set_bool(&mut self, key: *const c_char, value: bool) -> &mut Self {
        _ = self.inner.set_bool(unsafe { CStr::from_ptr(key) }, value);
        self
    }

    fn set_number(&mut self, key: *const c_char, value: i64) -> &mut Self {
        _ = self.inner.set_number(unsafe { CStr::from_ptr(key) }, value);
        self
    }
}
