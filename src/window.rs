//! SDL's window API wrapper.
//!
//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryVideo)):
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
//! - [ ] SDL_GetCurrentVideoDriver
//! - [x] SDL_GetDisplayForWindow
//! - [ ] SDL_GetGrabbedWindow
//! - [ ] SDL_GetNumVideoDrivers
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
//! - [x] SDL_GetWindowProperties
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
//! - [x] SDL_HideWindow
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
//! - [x] SDL_SetWindowPosition
//! - [ ] SDL_SetWindowProgressState
//! - [ ] SDL_SetWindowProgressValue
//! - [ ] SDL_SetWindowResizable
//! - [ ] SDL_SetWindowShape
//! - [x] SDL_SetWindowSize
//! - [ ] SDL_SetWindowSurfaceVSync
//! - [ ] SDL_SetWindowTitle
//! - [x] SDL_ShowWindow
//! - [ ] SDL_ShowWindowSystemMenu
//! - [x] SDL_SyncWindow
//! - [ ] SDL_UpdateWindowSurface
//! - [ ] SDL_UpdateWindowSurfaceRects
//! - [ ] SDL_WindowHasSurface
//! - [x] SDL_GetRenderer
//! - [x] SDL_CreateWindowAndRenderer

use crate::{
    Result,
    display::DisplayHandle,
    error::Error,
    properties::{Properties, PropertiesHandle},
    rect::PointI32,
    renderer::{Renderer, RendererHandle},
    resource::Ref,
    resource_new,
    surface::{Surface, SurfaceHandle},
    util::{c_ptr_to_str, to_result},
};
use bitmask_enum::bitmask;
use sdl3_sys::{
    render::{SDL_CreateWindowAndRenderer, SDL_GetRenderer, SDL_Renderer},
    video::*,
};
use std::{
    ffi::{CStr, c_char, c_void},
    mem::MaybeUninit,
    num::NonZero,
    ops::Deref,
    ptr::NonNull,
};

#[bitmask(u64)]
pub enum WindowFlags {
    Fullscreen = SDL_WINDOW_FULLSCREEN.0,
    OpenGL = SDL_WINDOW_OPENGL.0,
    Occluded = SDL_WINDOW_OCCLUDED.0,
    Hidden = SDL_WINDOW_HIDDEN.0,
    Borderless = SDL_WINDOW_BORDERLESS.0,
    Resizable = SDL_WINDOW_RESIZABLE.0,
    Minimized = SDL_WINDOW_MINIMIZED.0,
    Maximized = SDL_WINDOW_MAXIMIZED.0,
    MouseGrabbed = SDL_WINDOW_MOUSE_GRABBED.0,
    InputFocus = SDL_WINDOW_INPUT_FOCUS.0,
    MouseFocus = SDL_WINDOW_MOUSE_FOCUS.0,
    External = SDL_WINDOW_EXTERNAL.0,
    Modal = SDL_WINDOW_MODAL.0,
    HighPixelDensity = SDL_WINDOW_HIGH_PIXEL_DENSITY.0,
    MouseCapture = SDL_WINDOW_MOUSE_CAPTURE.0,
    AlwaysOnTop = SDL_WINDOW_ALWAYS_ON_TOP.0,
    Utility = SDL_WINDOW_UTILITY.0,
    Tooltip = SDL_WINDOW_TOOLTIP.0,
    PopupMenu = SDL_WINDOW_POPUP_MENU.0,
    KeyboardGrabbed = SDL_WINDOW_KEYBOARD_GRABBED.0,
    Vulkan = SDL_WINDOW_VULKAN.0,
    Metal = SDL_WINDOW_METAL.0,
    Transparent = SDL_WINDOW_TRANSPARENT.0,
    NotFocusable = SDL_WINDOW_NOT_FOCUSABLE.0,
}

pub struct WindowBuilder<'a> {
    inner: Ref<'a, Properties>,
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
        _ = self
            .inner
            .set_pointer(cstr, value.handle.as_ptr() as *mut c_void);
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

/// Read-only properties of a window, as documented by
/// [`SDL_GetWindowProperties`](https://wiki.libsdl.org/SDL3/SDL_GetWindowProperties).
///
/// Generic properties are returned bare since the docs guarantee their
/// existence; backend properties are returned as `Option` since they only
/// exist on their respective backends.
#[derive(Clone, Copy)]
pub struct WindowProperties<'a> {
    inner: Ref<'a, Properties>,
}

impl<'a> WindowProperties<'a> {
    fn new(inner: Ref<'a, Properties>) -> Self {
        Self { inner }
    }

    fn opt_str(&self, key: *const i8) -> Option<&str> {
        let cstr = unsafe { CStr::from_ptr(key) };
        let s = self.inner.string(cstr, std::ptr::null());

        if s.is_null() {
            return None;
        }

        Some(unsafe { c_ptr_to_str(s) })
    }

    fn opt_number(&self, key: *const i8) -> Option<i64> {
        let cstr = unsafe { CStr::from_ptr(key) };
        self.inner.has(cstr).then(|| self.inner.number(cstr, 0))
    }

    fn opt_ptr(&self, key: *const i8) -> Option<*mut c_void> {
        let cstr = unsafe { CStr::from_ptr(key) };
        let p = self.inner.pointer(cstr, std::ptr::null_mut());

        (!p.is_null()).then_some(p)
    }

    pub fn shape(&self) -> Option<Ref<'a, Surface>> {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_WINDOW_SHAPE_POINTER) };
        let p = self.inner.pointer(cstr, std::ptr::null_mut());

        SurfaceHandle::from_ptr(p.cast()).map(|h| unsafe { Ref::from_handle(h) })
    }

    pub fn hdr_enabled(&self) -> bool {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_WINDOW_HDR_ENABLED_BOOLEAN) };
        self.inner.bool(cstr, false)
    }

    pub fn sdr_white_level(&self) -> f32 {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_WINDOW_SDR_WHITE_LEVEL_FLOAT) };
        self.inner.float(cstr, 0.)
    }

    pub fn hdr_headroom(&self) -> f32 {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_WINDOW_HDR_HEADROOM_FLOAT) };
        self.inner.float(cstr, 0.)
    }

    pub fn cocoa_window(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_WINDOW_COCOA_WINDOW_POINTER)
    }

    pub fn cocoa_metal_view_tag(&self) -> Option<i64> {
        self.opt_number(SDL_PROP_WINDOW_COCOA_METAL_VIEW_TAG_NUMBER)
    }

    pub fn win32_hwnd(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_WINDOW_WIN32_HWND_POINTER)
    }

    pub fn win32_hdc(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_WINDOW_WIN32_HDC_POINTER)
    }

    pub fn win32_instance(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_WINDOW_WIN32_INSTANCE_POINTER)
    }

    pub fn x11_display(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_WINDOW_X11_DISPLAY_POINTER)
    }

    pub fn x11_screen(&self) -> Option<i64> {
        self.opt_number(SDL_PROP_WINDOW_X11_SCREEN_NUMBER)
    }

    pub fn x11_window(&self) -> Option<i64> {
        self.opt_number(SDL_PROP_WINDOW_X11_WINDOW_NUMBER)
    }

    pub fn wayland_display(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_WINDOW_WAYLAND_DISPLAY_POINTER)
    }

    pub fn wayland_surface(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_WINDOW_WAYLAND_SURFACE_POINTER)
    }

    pub fn wayland_viewport(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_WINDOW_WAYLAND_VIEWPORT_POINTER)
    }

    pub fn wayland_egl_window(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_WINDOW_WAYLAND_EGL_WINDOW_POINTER)
    }

    pub fn wayland_xdg_surface(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_WINDOW_WAYLAND_XDG_SURFACE_POINTER)
    }

    pub fn wayland_xdg_toplevel(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_WINDOW_WAYLAND_XDG_TOPLEVEL_POINTER)
    }

    pub fn wayland_xdg_toplevel_export_handle(&self) -> Option<&str> {
        self.opt_str(SDL_PROP_WINDOW_WAYLAND_XDG_TOPLEVEL_EXPORT_HANDLE_STRING)
    }

    pub fn wayland_xdg_popup(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_WINDOW_WAYLAND_XDG_POPUP_POINTER)
    }

    pub fn wayland_xdg_positioner(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_WINDOW_WAYLAND_XDG_POSITIONER_POINTER)
    }
}

impl Deref for WindowProperties<'_> {
    type Target = PropertiesHandle;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct WindowId {
    inner: NonZero<u32>,
}

impl WindowId {
    fn from_raw(raw: u32) -> Result<Self> {
        match NonZero::new(raw) {
            Some(inner) => Ok(Self { inner }),
            None => Err(Error::current()),
        }
    }

    unsafe fn from_raw_unchecked(raw: u32) -> Self {
        let inner = unsafe { NonZero::new_unchecked(raw) };
        Self { inner }
    }

    fn as_sdl(&self) -> SDL_WindowID {
        SDL_WindowID(self.inner.get())
    }
}

resource_new!(SDL_Window, Window, SDL_DestroyWindow);

impl WindowHandle {
    #[doc(alias = "SDL_SyncWindow")]
    pub fn sync(&self) -> Result {
        to_result(unsafe { SDL_SyncWindow(self.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_FlashWindow")]
    pub fn flash(&self, op: SDL_FlashOperation) -> Result {
        to_result(unsafe { SDL_FlashWindow(self.handle.as_ptr(), op) })
    }

    #[doc(alias = "SDL_GetWindowSize")]
    pub fn size(&self) -> PointI32 {
        let mut ret = MaybeUninit::<PointI32>::uninit();
        let ptr = ret.as_mut_ptr();

        unsafe {
            SDL_GetWindowSize(self.handle.as_ptr(), &raw mut (*ptr).x, &raw mut (*ptr).y);
            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_GetWindowPosition")]
    pub fn position(&self) -> PointI32 {
        let mut ret = MaybeUninit::<PointI32>::uninit();
        let ptr = ret.as_mut_ptr();

        unsafe {
            SDL_GetWindowPosition(self.handle.as_ptr(), &raw mut (*ptr).x, &raw mut (*ptr).y);
            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_GetWindowTitle")]
    pub fn title(&self) -> NonNull<c_char> {
        NonNull::new(unsafe { SDL_GetWindowTitle(self.handle.as_ptr()).cast_mut() })
            .expect("SDL_GetWindowTitle should return a valid pointer")
    }

    #[doc(alias = "SDL_GetWindowFlags")]
    pub fn flags(&self) -> SDL_WindowFlags {
        unsafe { SDL_GetWindowFlags(self.handle.as_ptr()) }
    }

    #[doc(alias = "SDL_GetRenderer")]
    pub fn renderer(&self) -> Option<RendererHandle> {
        RendererHandle::from_ptr(unsafe { SDL_GetRenderer(self.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_GetDisplayForWindow")]
    pub fn display(&self) -> Result<DisplayHandle> {
        let raw = unsafe { SDL_GetDisplayForWindow(self.handle.as_ptr()) };
        DisplayHandle::new(raw)
    }

    #[doc(alias = "SDL_SetWindowSize")]
    pub fn set_size(&self, size: PointI32) -> Result {
        to_result(unsafe { SDL_SetWindowSize(self.handle.as_ptr(), size.x, size.y) })
    }

    #[doc(alias = "SDL_SetWindowPosition")]
    pub fn set_pos(&self, pos: PointI32) -> Result {
        to_result(unsafe { SDL_SetWindowPosition(self.handle.as_ptr(), pos.x, pos.y) })
    }

    #[doc(alias = "SDL_ShowWindow")]
    pub fn show(&self) -> Result {
        to_result(unsafe { SDL_ShowWindow(self.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_HideWindow")]
    pub fn hide(&self) -> Result {
        to_result(unsafe { SDL_HideWindow(self.handle.as_ptr()) })
    }

    /// Read-only properties of this window, as documented by
    /// [`SDL_GetWindowProperties`](https://wiki.libsdl.org/SDL3/SDL_GetWindowProperties).
    ///
    /// Covers the generic properties plus the Cocoa, Win32, X11 and Wayland
    /// backends. Not covered: Android, iOS/UIKit, KMS/DRM, OpenVR, QNX,
    /// Vivante, Emscripten and visionOS, as well as
    /// `SDL_PROP_WINDOW_WAYLAND_WINDOW_ID_STRING`, which sdl3-sys does not
    /// expose.
    #[doc(alias = "SDL_GetWindowProperties")]
    pub fn properties(&'_ self) -> WindowProperties<'_> {
        let id = unsafe { SDL_GetWindowProperties(self.handle.as_ptr()) };
        let handle = PropertiesHandle::from_id(id).expect("A valid window should have properties");

        let r = unsafe { Ref::from_handle(handle) };
        WindowProperties::new(r)
    }
}

impl Window {
    pub const POS_CENTERED: i32 = SDL_WINDOWPOS_CENTERED;
    pub const POS_UNDEFINED: i32 = SDL_WINDOWPOS_UNDEFINED;

    /// Bind the builder to an existing property group.
    ///
    /// A single [`Properties`] can be shared between the window, renderer
    /// and GPU device builders, since their creation properties
    /// (`SDL_PROP_WINDOW_CREATE_*`, `SDL_PROP_RENDERER_CREATE_*`,
    /// `SDL_PROP_GPU_DEVICE_CREATE_*`) never collide with each other.
    /// They do collide with themselves, however: creating a second window
    /// from the same group inherits any leftover window properties, so use
    /// one [`Properties`] per window.
    pub fn builder(props: Ref<'_, Properties>) -> WindowBuilder<'_> {
        WindowBuilder { inner: props }
    }

    #[doc(alias = "SDL_CreateWindow")]
    pub fn new(title: &CStr, size: PointI32, flags: SDL_WindowFlags) -> Result<Self> {
        Self::from_ptr(unsafe { SDL_CreateWindow(title.as_ptr(), size.x, size.y, flags) })
    }

    #[doc(alias = "SDL_CreateWindowAndRenderer")]
    pub fn with_renderer(
        title: &CStr,
        size: PointI32,
        flags: SDL_WindowFlags,
    ) -> (Result<Self>, Result<Renderer>) {
        let mut ret = MaybeUninit::<(*mut SDL_Window, *mut SDL_Renderer)>::uninit();
        let ptr = ret.as_mut_ptr();

        unsafe {
            SDL_CreateWindowAndRenderer(
                title.as_ptr(),
                size.x,
                size.y,
                flags,
                &raw mut (*ptr).0,
                &raw mut (*ptr).1,
            );

            let init = ret.assume_init();
            (Self::from_ptr(init.0), Renderer::from_ptr(init.1))
        }
    }

    #[doc(alias = "SDL_GetWindowFromID")]
    pub fn from_id(id: WindowId) -> Option<WindowHandle> {
        NonNull::new(unsafe { SDL_GetWindowFromID(id.as_sdl()) })
            .map(|handle| WindowHandle { handle })
    }

    /// Returns this window's unique ID.
    #[doc(alias = "SDL_GetWindowID")]
    pub fn id(&self) -> WindowId {
        let id = unsafe { SDL_GetWindowID(self.inner.handle.as_ptr()) }.0;

        // SAFETY: Valid windows should always have an ID.
        unsafe { WindowId::from_raw_unchecked(id) }
    }
}
