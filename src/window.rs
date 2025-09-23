//! SDL's window API wrapper.
//!
//! - [ ] SDL_CreatePopupWindow
//! - [x] SDL_CreateWindow
//! - [x] SDL_CreateWindowWithProperties
//! - [x] SDL_DestroyWindow
//! - [ ] SDL_DestroyWindowSurface
//! - [ ] SDL_DisableScreenSaver
//! - [ ] SDL_EGL_GetCurrentConfig
//! - [ ] SDL_EGL_GetCurrentDisplay
//! - [ ] SDL_EGL_GetProcAddress
//! - [ ] SDL_EGL_GetWindowSurface
//! - [ ] SDL_EGL_SetAttributeCallbacks
//! - [ ] SDL_EnableScreenSaver
//! - [x] SDL_FlashWindow
//! - [ ] SDL_GetClosestFullscreenDisplayMode
//! - [ ] SDL_GetCurrentDisplayMode
//! - [ ] SDL_GetCurrentDisplayOrientation
//! - [ ] SDL_GetCurrentVideoDriver
//! - [ ] SDL_GetDesktopDisplayMode
//! - [ ] SDL_GetDisplayBounds
//! - [ ] SDL_GetDisplayContentScale
//! - [ ] SDL_GetDisplayForPoint
//! - [ ] SDL_GetDisplayForRect
//! - [ ] SDL_GetDisplayForWindow
//! - [ ] SDL_GetDisplayName
//! - [ ] SDL_GetDisplayProperties
//! - [ ] SDL_GetDisplays
//! - [ ] SDL_GetDisplayUsableBounds
//! - [ ] SDL_GetFullscreenDisplayModes
//! - [ ] SDL_GetGrabbedWindow
//! - [ ] SDL_GetNaturalDisplayOrientation
//! - [ ] SDL_GetNumVideoDrivers
//! - [ ] SDL_GetPrimaryDisplay
//! - [ ] SDL_GetSystemTheme
//! - [ ] SDL_GetVideoDriver
//! - [ ] SDL_GetWindowAspectRatio
//! - [ ] SDL_GetWindowBordersSize
//! - [ ] SDL_GetWindowDisplayScale
//! - [x] SDL_GetWindowFlags
//! - [x] SDL_GetWindowFromID
//! - [ ] SDL_GetWindowFullscreenMode
//! - [ ] SDL_GetWindowICCProfile
//! - [x] SDL_GetWindowID
//! - [ ] SDL_GetWindowKeyboardGrab
//! - [ ] SDL_GetWindowMaximumSize
//! - [ ] SDL_GetWindowMinimumSize
//! - [ ] SDL_GetWindowMouseGrab
//! - [ ] SDL_GetWindowMouseRect
//! - [ ] SDL_GetWindowOpacity
//! - [ ] SDL_GetWindowParent
//! - [ ] SDL_GetWindowPixelDensity
//! - [ ] SDL_GetWindowPixelFormat
//! - [x] SDL_GetWindowPosition
//! - [ ] SDL_GetWindowProgressState
//! - [ ] SDL_GetWindowProgressValue
//! - [ ] SDL_GetWindowProperties
//! - [ ] SDL_GetWindows
//! - [ ] SDL_GetWindowSafeArea
//! - [x] SDL_GetWindowSize
//! - [ ] SDL_GetWindowSizeInPixels
//! - [ ] SDL_GetWindowSurface
//! - [ ] SDL_GetWindowSurfaceVSync
//! - [x] SDL_GetWindowTitle
//! - [ ] SDL_GL_CreateContext
//! - [ ] SDL_GL_DestroyContext
//! - [ ] SDL_GL_ExtensionSupported
//! - [ ] SDL_GL_GetAttribute
//! - [ ] SDL_GL_GetCurrentContext
//! - [ ] SDL_GL_GetCurrentWindow
//! - [ ] SDL_GL_GetProcAddress
//! - [ ] SDL_GL_GetSwapInterval
//! - [ ] SDL_GL_LoadLibrary
//! - [ ] SDL_GL_MakeCurrent
//! - [ ] SDL_GL_ResetAttributes
//! - [ ] SDL_GL_SetAttribute
//! - [ ] SDL_GL_SetSwapInterval
//! - [ ] SDL_GL_SwapWindow
//! - [ ] SDL_GL_UnloadLibrary
//! - [ ] SDL_HideWindow
//! - [ ] SDL_MaximizeWindow
//! - [ ] SDL_MinimizeWindow
//! - [ ] SDL_RaiseWindow
//! - [ ] SDL_RestoreWindow
//! - [ ] SDL_ScreenSaverEnabled
//! - [ ] SDL_SetWindowAlwaysOnTop
//! - [ ] SDL_SetWindowAspectRatio
//! - [ ] SDL_SetWindowBordered
//! - [ ] SDL_SetWindowFocusable
//! - [ ] SDL_SetWindowFullscreen
//! - [ ] SDL_SetWindowFullscreenMode
//! - [ ] SDL_SetWindowHitTest
//! - [ ] SDL_SetWindowIcon
//! - [ ] SDL_SetWindowKeyboardGrab
//! - [ ] SDL_SetWindowMaximumSize
//! - [ ] SDL_SetWindowMinimumSize
//! - [ ] SDL_SetWindowModal
//! - [ ] SDL_SetWindowMouseGrab
//! - [ ] SDL_SetWindowMouseRect
//! - [ ] SDL_SetWindowOpacity
//! - [ ] SDL_SetWindowParent
//! - [ ] SDL_SetWindowPosition
//! - [ ] SDL_SetWindowProgressState
//! - [ ] SDL_SetWindowProgressValue
//! - [ ] SDL_SetWindowResizable
//! - [ ] SDL_SetWindowShape
//! - [ ] SDL_SetWindowSize
//! - [ ] SDL_SetWindowSurfaceVSync
//! - [ ] SDL_SetWindowTitle
//! - [ ] SDL_ShowWindow
//! - [ ] SDL_ShowWindowSystemMenu
//! - [x] SDL_SyncWindow
//! - [ ] SDL_UpdateWindowSurface
//! - [ ] SDL_UpdateWindowSurfaceRects
//! - [ ] SDL_WindowHasSurface
//! - [x] SDL_GetRenderer
//! - [x] SDL_CreateWindowAndRenderer

use crate::{
    defs::SdlResult,
    properties::Properties,
    renderer::{Renderer, RendererRef},
    resource,
    subsystem::Video,
    util::{c_to_str, to_result},
};
use bitmask_enum::bitmask;
use sdl3_sys::{
    render::{SDL_CreateWindowAndRenderer, SDL_GetRenderer, SDL_Renderer},
    video::*,
};
use std::{
    ffi::{CStr, c_void},
    mem::MaybeUninit,
    num::NonZero,
    ptr::NonNull,
};

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
    pub fn size(&mut self, (w, h): (i32, i32)) -> &mut Self {
        self.width(w.into());
        self.height(h.into())
    }

    /// Utility method that calls `self.x()` and `self.y()`.
    pub fn position(&mut self, (x, y): (i32, i32)) -> &mut Self {
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

    #[doc(alias = "SDL_FlashWindow")]
    pub fn flash(&self, op: SDL_FlashOperation) -> SdlResult {
        to_result(unsafe { SDL_FlashWindow(self.handle.as_ptr(), op) })
    }

    #[doc(alias = "SDL_GetWindowSize")]
    pub fn size(&self) -> (i32, i32) {
        let mut ret = MaybeUninit::<(i32, i32)>::uninit();
        let ptr = ret.as_mut_ptr();

        unsafe {
            SDL_GetWindowSize(self.handle.as_ptr(), &raw mut (*ptr).0, &raw mut (*ptr).1);
            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_GetWindowPosition")]
    pub fn position(&self) -> (i32, i32) {
        let mut ret = MaybeUninit::<(i32, i32)>::uninit();
        let ptr = ret.as_mut_ptr();

        unsafe {
            SDL_GetWindowPosition(self.handle.as_ptr(), &raw mut (*ptr).0, &raw mut (*ptr).1);
            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_GetWindowTitle")]
    pub fn title(&self) -> &str {
        // SAFETY: SDL guarantees the window title to be UTF-8.
        unsafe { c_to_str(SDL_GetWindowTitle(self.handle.as_ptr())) }
    }

    #[doc(alias = "SDL_GetWindowFlags")]
    pub fn flags(&self) -> WindowFlags {
        unsafe { SDL_GetWindowFlags(self.handle.as_ptr()) }.into()
    }

    #[doc(alias = "SDL_GetRenderer")]
    pub fn renderer(&self) -> Option<RendererRef> {
        RendererRef::from_ptr(unsafe { SDL_GetRenderer(self.handle.as_ptr()) })
    }
}

impl Window {
    pub const POS_CENTERED: i32 = SDL_WINDOWPOS_CENTERED;
    pub const POS_UNDEFINED: i32 = SDL_WINDOWPOS_UNDEFINED;

    #[doc(alias = "SDL_CreateWindow")]
    pub fn new(title: &CStr, width: i32, height: i32, flags: WindowFlags) -> SdlResult<Self> {
        Self::from_ptr(unsafe { SDL_CreateWindow(title.as_ptr(), width, height, flags.into()) })
    }

    #[doc(alias = "SDL_CreateWindowAndRenderer")]
    pub fn with_renderer(
        title: &CStr,
        (w, h): (i32, i32),
        flags: WindowFlags,
    ) -> (SdlResult<Self>, SdlResult<Renderer>) {
        let mut ret = MaybeUninit::<(*mut SDL_Window, *mut SDL_Renderer)>::uninit();
        let ptr = ret.as_mut_ptr();

        unsafe {
            SDL_CreateWindowAndRenderer(
                title.as_ptr(),
                w,
                h,
                flags.into(),
                &raw mut (*ptr).0,
                &raw mut (*ptr).1,
            );

            let init = ret.assume_init();
            (Self::from_ptr(init.0), Renderer::from_ptr(init.1))
        }
    }

    #[doc(alias = "SDL_GetWindowFromID")]
    pub fn from_id(id: SDL_WindowID) -> Option<WindowRef> {
        NonNull::new(unsafe { SDL_GetWindowFromID(id) }).map(|handle| WindowRef { handle })
    }

    /// Returns this window's unique ID.
    /// An ID of 0 is invalid, so `NonZero` is returned instead.
    #[doc(alias = "SDL_GetWindowID")]
    pub fn id(&self) -> NonZero<SDL_WindowID> {
        NonZero::new(unsafe { SDL_GetWindowID(self.inner.handle.as_ptr()) })
            .expect("SDL_GetWindowID returned invalid (zero) ID")
    }
}
