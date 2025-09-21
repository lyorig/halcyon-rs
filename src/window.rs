use crate::coord::Pixel;
use crate::defs::SdlResult;
use crate::properties::Properties;
use crate::resource;
use crate::subsystem::Video;
use crate::util::to_result;
use bitmask_enum::bitmask;
use sdl3_sys::video::*;
use std::ffi::{CStr, c_void};
use std::mem::MaybeUninit;
use std::num::NonZero;

#[bitmask(u64)]
pub enum WindowFlags {
    Fullscreen = SDL_WINDOW_FULLSCREEN,
    OpenGL = SDL_WINDOW_OPENGL,
    Occluded = SDL_WINDOW_OCCLUDED,
    Hidden = SDL_WINDOW_HIDDEN,
    Borderless = SDL_WINDOW_BORDERLESS,
    Resizable = SDL_WINDOW_RESIZABLE,
    Minimized = SDL_WINDOW_MINIMIZED,
    Maximized = SDL_WINDOW_MAXIMIZED,
    MouseGrabbed = SDL_WINDOW_MOUSE_GRABBED,
    InputFocus = SDL_WINDOW_INPUT_FOCUS,
    MouseFocus = SDL_WINDOW_MOUSE_FOCUS,
    External = SDL_WINDOW_EXTERNAL,
    Modal = SDL_WINDOW_MODAL,
    HighPixelDensity = SDL_WINDOW_HIGH_PIXEL_DENSITY,
    MouseCapture = SDL_WINDOW_MOUSE_CAPTURE,
    AlwaysOnTop = SDL_WINDOW_ALWAYS_ON_TOP,
    Utility = SDL_WINDOW_UTILITY,
    Tooltip = SDL_WINDOW_TOOLTIP,
    PopupMenu = SDL_WINDOW_POPUP_MENU,
    KeyboardGrabbed = SDL_WINDOW_KEYBOARD_GRABBED,
    Vulkan = SDL_WINDOW_VULKAN,
    Metal = SDL_WINDOW_METAL,
    Transparent = SDL_WINDOW_TRANSPARENT,
    NotFocusable = SDL_WINDOW_NOT_FOCUSABLE,
}

pub struct WindowBuilder {
    inner: Properties,
}

impl WindowBuilder {
    pub fn new() -> Self {
        Self {
            inner: Properties::new(),
        }
    }

    pub fn always_on_top(&mut self, value: bool) -> &mut Self {
        let _ = self
            .inner
            .set_bool(SDL_PROP_WINDOW_CREATE_ALWAYS_ON_TOP_BOOLEAN, value);
        self
    }

    pub fn borderless(&mut self, value: bool) -> &mut Self {
        let _ = self
            .inner
            .set_bool(SDL_PROP_WINDOW_CREATE_BORDERLESS_BOOLEAN, value);
        self
    }

    pub fn constrain_popup(&mut self, value: bool) -> &mut Self {
        let _ = self
            .inner
            .set_bool(SDL_PROP_WINDOW_CREATE_CONSTRAIN_POPUP_BOOLEAN, value);
        self
    }

    // TODO: External graphics context

    pub fn focusable(&mut self, value: bool) -> &mut Self {
        let _ = self
            .inner
            .set_bool(SDL_PROP_WINDOW_CREATE_FOCUSABLE_BOOLEAN, value);
        self
    }

    pub fn fullscreen(&mut self, value: bool) -> &mut Self {
        let _ = self
            .inner
            .set_bool(SDL_PROP_WINDOW_CREATE_FULLSCREEN_BOOLEAN, value);
        self
    }

    pub fn height(&mut self, value: i64) -> &mut Self {
        let _ = self
            .inner
            .set_number(SDL_PROP_WINDOW_CREATE_HEIGHT_NUMBER, value);
        self
    }

    pub fn hidden(&mut self, value: bool) -> &mut Self {
        let _ = self
            .inner
            .set_bool(SDL_PROP_WINDOW_CREATE_HIDDEN_BOOLEAN, value);
        self
    }

    pub fn high_pixel_density(&mut self, value: bool) -> &mut Self {
        let _ = self
            .inner
            .set_bool(SDL_PROP_WINDOW_CREATE_HIGH_PIXEL_DENSITY_BOOLEAN, value);
        self
    }

    pub fn maximized(&mut self, value: bool) -> &mut Self {
        let _ = self
            .inner
            .set_bool(SDL_PROP_WINDOW_CREATE_MAXIMIZED_BOOLEAN, value);
        self
    }

    pub fn menu(&mut self, value: bool) -> &mut Self {
        let _ = self
            .inner
            .set_bool(SDL_PROP_WINDOW_CREATE_MENU_BOOLEAN, value);
        self
    }

    // TODO: Metal

    pub fn minimized(&mut self, value: bool) -> &mut Self {
        let _ = self
            .inner
            .set_bool(SDL_PROP_WINDOW_CREATE_MINIMIZED_BOOLEAN, value);
        self
    }

    pub fn modal(&mut self, value: bool) -> &mut Self {
        let _ = self
            .inner
            .set_bool(SDL_PROP_WINDOW_CREATE_MODAL_BOOLEAN, value);
        self
    }

    pub fn mouse_grabbed(&mut self, value: bool) -> &mut Self {
        let _ = self
            .inner
            .set_bool(SDL_PROP_WINDOW_CREATE_MOUSE_GRABBED_BOOLEAN, value);
        self
    }

    // TODO: OpenGL

    pub fn parent(&mut self, value: WindowRef) -> &mut Self {
        let _ = self.inner.set_pointer(
            SDL_PROP_WINDOW_CREATE_PARENT_POINTER,
            value.handle.as_ptr() as *mut c_void,
        );
        self
    }

    pub fn resizable(&mut self, value: bool) -> &mut Self {
        let _ = self
            .inner
            .set_bool(SDL_PROP_WINDOW_CREATE_RESIZABLE_BOOLEAN, value);
        self
    }

    pub fn title(&mut self, value: &CStr) -> &mut Self {
        let _ = self
            .inner
            .set_string(SDL_PROP_WINDOW_CREATE_TITLE_STRING, value);
        self
    }

    pub fn transparent(&mut self, value: bool) -> &mut Self {
        let _ = self
            .inner
            .set_bool(SDL_PROP_WINDOW_CREATE_TRANSPARENT_BOOLEAN, value);
        self
    }

    pub fn tooltip(&mut self, value: bool) -> &mut Self {
        let _ = self
            .inner
            .set_bool(SDL_PROP_WINDOW_CREATE_TOOLTIP_BOOLEAN, value);
        self
    }

    pub fn utility(&mut self, value: bool) -> &mut Self {
        let _ = self
            .inner
            .set_bool(SDL_PROP_WINDOW_CREATE_UTILITY_BOOLEAN, value);
        self
    }

    // TODO: Vulkan

    pub fn width(&mut self, value: i64) -> &mut Self {
        let _ = self
            .inner
            .set_number(SDL_PROP_WINDOW_CREATE_WIDTH_NUMBER, value);
        self
    }

    pub fn x(&mut self, value: i64) -> &mut Self {
        let _ = self
            .inner
            .set_number(SDL_PROP_WINDOW_CREATE_X_NUMBER, value);
        self
    }

    pub fn y(&mut self, value: i64) -> &mut Self {
        let _ = self
            .inner
            .set_number(SDL_PROP_WINDOW_CREATE_Y_NUMBER, value);
        self
    }

    /// Utility method that calls `self.width()` and `self.height()`.
    pub fn size(&mut self, (w, h): (Pixel, Pixel)) -> &mut Self {
        self.width(w.into());
        self.height(h.into())
    }

    /// Utility method that calls `self.x()` and `self.y()`.
    pub fn position(&mut self, (x, y): (Pixel, Pixel)) -> &mut Self {
        self.x(x.into());
        self.y(y.into())
    }

    /// Build the window.
    ///
    /// This requires a `Video` subsystem as a parameter to "prove"
    /// you've initialized it. SDL would've probably errored if you hadn't
    /// anyway, but it's a zero-cost way to prevent cross-platform bugs.
    #[doc(alias = "SDL_CreateWindowWithProperties")]
    pub fn build(&self, _subsystem: &Video) -> SdlResult<Window> {
        Window::from_ptr(unsafe { SDL_CreateWindowWithProperties(self.inner.id()) })
    }
}

resource!(Window, WindowRef, SDL_Window, SDL_DestroyWindow);

impl WindowRef {
    #[doc(alias = "SDL_SyncWindow")]
    pub fn sync(&self) -> SdlResult {
        to_result(unsafe { SDL_SyncWindow(self.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_GetWindowSize")]
    pub fn size(&self) -> (i32, i32) {
        let mut ret = (MaybeUninit::uninit(), MaybeUninit::uninit());

        unsafe {
            SDL_GetWindowSize(self.handle.as_ptr(), ret.0.as_mut_ptr(), ret.1.as_mut_ptr());
            (ret.0.assume_init(), ret.1.assume_init())
        }
    }
}

impl Window {
    pub const POS_CENTERED: i32 = SDL_WINDOWPOS_CENTERED;
    pub const POS_UNDEFINED: i32 = SDL_WINDOWPOS_UNDEFINED;

    #[doc(alias = "SDL_CreateWindow")]
    pub fn new(title: &CStr, width: i32, height: i32, flags: WindowFlags) -> SdlResult<Self> {
        Self::from_ptr(unsafe { SDL_CreateWindow(title.as_ptr(), width, height, flags.into()) })
    }

    /// Returns this window's unique ID.
    /// An ID of 0 is invalid, so `NonZero` is returned instead.
    #[doc(alias = "SDL_GetWindowID")]
    pub fn id(&self) -> NonZero<SDL_WindowID> {
        NonZero::new(unsafe { SDL_GetWindowID(self.inner.handle.as_ptr()) })
            .expect("SDL_GetWindowID returned invalid (zero) ID")
    }
}
