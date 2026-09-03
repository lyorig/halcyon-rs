//! SDL's window API wrapper.
//!
//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryVideo)):
//! - [x] SDL_CreatePopupWindow
//! - [x] SDL_CreateWindow
//! - [x] SDL_CreateWindowWithProperties
//! - [x] SDL_DestroyWindow
//! - [x] SDL_DestroyWindowSurface
//! - [x] SDL_DisableScreenSaver
//! - [ ] SDL_EGL_GetCurrentConfig
//! - [ ] SDL_EGL_GetCurrentDisplay
//! - [ ] SDL_EGL_GetProcAddress
//! - [ ] SDL_EGL_GetWindowSurface
//! - [ ] SDL_EGL_SetAttributeCallbacks
//! - [x] SDL_EnableScreenSaver
//! - [x] SDL_FlashWindow
//! - [x] SDL_GetCurrentVideoDriver
//! - [x] SDL_GetDisplayForWindow
//! - [x] SDL_GetGrabbedWindow
//! - [x] SDL_GetNumVideoDrivers
//! - [x] SDL_GetSystemTheme
//! - [x] SDL_GetVideoDriver
//! - [x] SDL_GetWindowAspectRatio
//! - [x] SDL_GetWindowBordersSize
//! - [x] SDL_GetWindowDisplayScale
//! - [x] SDL_GetWindowFlags
//! - [x] SDL_GetWindowFromID
//! - [x] SDL_GetWindowFullscreenMode
//! - [x] SDL_GetWindowICCProfile
//! - [x] SDL_GetWindowID
//! - [x] SDL_GetWindowKeyboardGrab
//! - [x] SDL_GetWindowMaximumSize
//! - [x] SDL_GetWindowMinimumSize
//! - [x] SDL_GetWindowMouseGrab
//! - [x] SDL_GetWindowMouseRect
//! - [x] SDL_GetWindowOpacity
//! - [x] SDL_GetWindowParent
//! - [x] SDL_GetWindowPixelDensity
//! - [x] SDL_GetWindowPixelFormat
//! - [x] SDL_GetWindowPosition
//! - [x] SDL_GetWindowProgressState
//! - [x] SDL_GetWindowProgressValue
//! - [x] SDL_GetWindowProperties
//! - [x] SDL_GetWindows
//! - [x] SDL_GetWindowSafeArea
//! - [x] SDL_GetWindowSize
//! - [x] SDL_GetWindowSizeInPixels
//! - [x] SDL_GetWindowSurface
//! - [x] SDL_GetWindowSurfaceVSync
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
//! - [x] SDL_MaximizeWindow
//! - [x] SDL_MinimizeWindow
//! - [x] SDL_RaiseWindow
//! - [x] SDL_RestoreWindow
//! - [x] SDL_ScreenSaverEnabled
//! - [x] SDL_SetWindowAlwaysOnTop
//! - [x] SDL_SetWindowAspectRatio
//! - [x] SDL_SetWindowBordered
//! - [x] SDL_SetWindowFocusable
//! - [x] SDL_SetWindowFullscreen
//! - [x] SDL_SetWindowFullscreenMode
//! - [ ] SDL_SetWindowHitTest
//! - [x] SDL_SetWindowIcon
//! - [x] SDL_SetWindowKeyboardGrab
//! - [x] SDL_SetWindowMaximumSize
//! - [x] SDL_SetWindowMinimumSize
//! - [x] SDL_SetWindowModal
//! - [x] SDL_SetWindowMouseGrab
//! - [x] SDL_SetWindowMouseRect
//! - [x] SDL_SetWindowOpacity
//! - [x] SDL_SetWindowParent
//! - [x] SDL_SetWindowPosition
//! - [x] SDL_SetWindowProgressState
//! - [x] SDL_SetWindowProgressValue
//! - [x] SDL_SetWindowResizable
//! - [x] SDL_SetWindowShape
//! - [x] SDL_SetWindowSize
//! - [x] SDL_SetWindowSurfaceVSync
//! - [x] SDL_SetWindowTitle
//! - [x] SDL_ShowWindow
//! - [x] SDL_ShowWindowSystemMenu
//! - [x] SDL_SyncWindow
//! - [x] SDL_UpdateWindowSurface
//! - [x] SDL_UpdateWindowSurfaceRects
//! - [x] SDL_WindowHasSurface
//! - [x] SDL_GetRenderer
//! - [x] SDL_CreateWindowAndRenderer
//!
//! The remaining `[ ]` entries are OpenGL/EGL interop and the hit-testing
//! callback, which each need their own dedicated abstraction.

use crate::{
    Result,
    boxed::Box,
    display::Display,
    error::Error,
    impl_enum_transmute, mod_reexport,
    properties::{Properties, PropertiesHandle},
    rect::{PointI32, RectI32},
    renderer::{Renderer, RendererHandle},
    resource::Ref,
    resource_new,
    surface::Surface,
    util::{c_ptr_to_str, opt2ptr, opt2res_map, to_result},
};

use bitflags::bitflags;
use sdl3_sys::{
    pixels::SDL_PixelFormat,
    render::{SDL_CreateWindowAndRenderer, SDL_GetRenderer, SDL_Renderer},
    video::*,
};

use std::{
    ffi::{CStr, c_char},
    mem::{MaybeUninit, transmute},
    num::NonZero,
    ptr::NonNull,
};

mod_reexport!(builder);
mod_reexport!(properties);

bitflags! {
    #[derive(Clone, Copy)]
    #[doc(alias = "SDL_WindowFlags")]
    pub struct WindowFlags: u64 {
        /// Window is in fullscreen mode.
        const FULLSCREEN = SDL_WINDOW_FULLSCREEN.0;
        /// Window usable with an OpenGL context.
        const OPENGL = SDL_WINDOW_OPENGL.0;
        /// Window is occluded.
        const OCCLUDED = SDL_WINDOW_OCCLUDED.0;
        /// Window is neither mapped onto the desktop nor shown in the
        /// taskbar/dock/window list; calling [`show`](WindowHandle::show) is required
        /// for it to become visible.
        const HIDDEN = SDL_WINDOW_HIDDEN.0;
        /// No window decoration.
        const BORDERLESS = SDL_WINDOW_BORDERLESS.0;
        /// Window can be resized.
        const RESIZABLE = SDL_WINDOW_RESIZABLE.0;
        /// Window is minimized.
        const MINIMIZED = SDL_WINDOW_MINIMIZED.0;
        /// Window is maximized.
        const MAXIMIZED = SDL_WINDOW_MAXIMIZED.0;
        /// Window has grabbed mouse input.
        const MOUSE_GRABBED = SDL_WINDOW_MOUSE_GRABBED.0;
        /// Window has input focus.
        const INPUT_FOCUS = SDL_WINDOW_INPUT_FOCUS.0;
        /// Window has mouse focus.
        const MOUSE_FOCUS = SDL_WINDOW_MOUSE_FOCUS.0;
        /// Window not created by SDL.
        const EXTERNAL = SDL_WINDOW_EXTERNAL.0;
        /// Window is modal.
        const MODAL = SDL_WINDOW_MODAL.0;
        /// Window uses a high pixel density back buffer if possible.
        const HIGH_PIXEL_DENSITY = SDL_WINDOW_HIGH_PIXEL_DENSITY.0;
        /// Window has mouse captured (unrelated to [`MouseGrabbed`](WindowFlags::MouseGrabbed)).
        const MOUSE_CAPTURE = SDL_WINDOW_MOUSE_CAPTURE.0;
        /// Window has relative mode enabled.
        const MOUSE_RELATIVE_MODE = SDL_WINDOW_MOUSE_RELATIVE_MODE.0;
        /// Window should always be above others.
        const ALWAYS_ON_TOP = SDL_WINDOW_ALWAYS_ON_TOP.0;
        /// Window should be treated as a utility window, not showing in the task bar
        /// and window list.
        const UTILITY = SDL_WINDOW_UTILITY.0;
        /// Window should be treated as a tooltip and does not get mouse or keyboard
        /// focus; requires a parent window.
        const TOOLTIP = SDL_WINDOW_TOOLTIP.0;
        /// Window should be treated as a popup menu; requires a parent window.
        const POPUP_MENU = SDL_WINDOW_POPUP_MENU.0;
        /// Window has grabbed keyboard input.
        const KEYBOARD_GRABBED = SDL_WINDOW_KEYBOARD_GRABBED.0;
        /// Window is in fill-document mode (Emscripten only).
        const FILL_DOCUMENT = SDL_WINDOW_FILL_DOCUMENT.0;
        /// Window usable for a Vulkan surface.
        const VULKAN = SDL_WINDOW_VULKAN.0;
        /// Window usable for a Metal view.
        const METAL = SDL_WINDOW_METAL.0;
        /// Window with a transparent buffer.
        const TRANSPARENT = SDL_WINDOW_TRANSPARENT.0;
        /// Window should not be focusable.
        const NOT_FOCUSABLE = SDL_WINDOW_NOT_FOCUSABLE.0;
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SystemTheme {
    Unknown = SDL_SystemTheme::UNKNOWN.0,
    Light = SDL_SystemTheme::LIGHT.0,
    Dark = SDL_SystemTheme::DARK.0,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProgressState {
    None = SDL_ProgressState::NONE.0,
    Indeterminate = SDL_ProgressState::INDETERMINATE.0,
    Normal = SDL_ProgressState::NORMAL.0,
    Paused = SDL_ProgressState::PAUSED.0,
    Error = SDL_ProgressState::ERROR.0,
}

impl_enum_transmute!(SDL_WindowFlags, WindowFlags);
impl_enum_transmute!(SDL_SystemTheme, SystemTheme);
impl_enum_transmute!(SDL_ProgressState, ProgressState);

impl std::fmt::Display for SystemTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

#[derive(Clone, Copy)]
pub struct WindowId {
    inner: NonZero<u32>,
}

impl WindowId {
    fn from_raw(raw: u32) -> Result<Self> {
        opt2res_map(NonZero::new(raw), |inner| Self { inner })
    }

    const unsafe fn from_raw_unchecked(raw: u32) -> Self {
        let inner = unsafe { NonZero::new_unchecked(raw) };
        Self { inner }
    }

    const fn as_sdl(self) -> SDL_WindowID {
        SDL_WindowID(self.inner.get())
    }
}

resource_new!(SDL_Window, Window, SDL_DestroyWindow);

#[doc(alias = "SDL_GetNumVideoDrivers")]
pub fn num_video_drivers() -> i32 {
    unsafe { SDL_GetNumVideoDrivers() }
}

#[doc(alias = "SDL_GetVideoDriver")]
pub fn video_driver(index: i32) -> Option<&'static str> {
    let ptr = unsafe { SDL_GetVideoDriver(index) };

    if ptr.is_null() {
        None
    } else {
        // SAFETY: Video driver names are valid UTF-8 and stored statically.
        Some(unsafe { c_ptr_to_str(ptr) })
    }
}

#[doc(alias = "SDL_GetCurrentVideoDriver")]
pub fn current_video_driver() -> Option<&'static str> {
    let ptr = unsafe { SDL_GetCurrentVideoDriver() };

    if ptr.is_null() {
        None
    } else {
        // SAFETY: Video driver names are valid UTF-8 and stored statically.
        Some(unsafe { c_ptr_to_str(ptr) })
    }
}

#[doc(alias = "SDL_GetSystemTheme")]
pub fn system_theme() -> SystemTheme {
    // SAFETY: `SystemTheme` has the same representation as `SDL_SystemTheme`.
    unsafe { transmute(SDL_GetSystemTheme()) }
}

#[doc(alias = "SDL_GetGrabbedWindow")]
pub fn grabbed_window() -> Option<WindowHandle> {
    WindowHandle::from_ptr(unsafe { SDL_GetGrabbedWindow() })
}

#[doc(alias = "SDL_GetWindows")]
pub fn windows() -> Result<Box<[WindowHandle]>> {
    let mut count = MaybeUninit::uninit();
    let ptr = unsafe { SDL_GetWindows(count.as_mut_ptr()) };

    // SAFETY: On success, SDL allocates `count` window pointers. `WindowHandle`
    // is a `Copy` wrapper around `NonNull<SDL_Window>`, which has the same size
    // and alignment as `*mut SDL_Window`.
    unsafe { Box::from_raw_parts_nullck(ptr.cast(), count.assume_init() as _) }
}

#[doc(alias = "SDL_ScreenSaverEnabled")]
pub fn screen_saver_enabled() -> bool {
    unsafe { SDL_ScreenSaverEnabled() }
}

#[doc(alias = "SDL_EnableScreenSaver")]
pub fn enable_screen_saver() -> Result<()> {
    to_result(unsafe { SDL_EnableScreenSaver() })
}

#[doc(alias = "SDL_DisableScreenSaver")]
pub fn disable_screen_saver() -> Result<()> {
    to_result(unsafe { SDL_DisableScreenSaver() })
}

impl WindowHandle {
    #[doc(alias = "SDL_SyncWindow")]
    pub fn sync(&self) -> Result<()> {
        to_result(unsafe { SDL_SyncWindow(self.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_FlashWindow")]
    pub fn flash(&self, op: SDL_FlashOperation) -> Result<()> {
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

    #[doc(alias = "SDL_GetWindowSizeInPixels")]
    pub fn size_in_pixels(&self) -> PointI32 {
        let mut ret = MaybeUninit::<PointI32>::uninit();
        let ptr = ret.as_mut_ptr();

        unsafe {
            SDL_GetWindowSizeInPixels(self.as_ptr(), &raw mut (*ptr).x, &raw mut (*ptr).y);
            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_GetWindowMinimumSize")]
    pub fn minimum_size(&self) -> PointI32 {
        let mut ret = MaybeUninit::<PointI32>::uninit();
        let ptr = ret.as_mut_ptr();

        unsafe {
            SDL_GetWindowMinimumSize(self.as_ptr(), &raw mut (*ptr).x, &raw mut (*ptr).y);
            ret.assume_init()
        }
    }

    #[doc(alias = "SDL_GetWindowMaximumSize")]
    pub fn maximum_size(&self) -> PointI32 {
        let mut ret = MaybeUninit::<PointI32>::uninit();
        let ptr = ret.as_mut_ptr();

        unsafe {
            SDL_GetWindowMaximumSize(self.as_ptr(), &raw mut (*ptr).x, &raw mut (*ptr).y);
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
        unsafe {
            NonNull::new(SDL_GetWindowTitle(self.handle.as_ptr()).cast_mut()).unwrap_unchecked()
        }
    }

    #[doc(alias = "SDL_GetWindowFlags")]
    pub fn flags(&self) -> WindowFlags {
        unsafe { SDL_GetWindowFlags(self.handle.as_ptr()) }.into()
    }

    #[doc(alias = "SDL_GetRenderer")]
    pub fn renderer(&self) -> Option<RendererHandle> {
        RendererHandle::from_ptr(unsafe { SDL_GetRenderer(self.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_GetDisplayForWindow")]
    pub fn display(&self) -> Result<Display> {
        let raw = unsafe { SDL_GetDisplayForWindow(self.handle.as_ptr()) };
        Display::from_sdl(raw)
    }

    #[doc(alias = "SDL_GetWindowParent")]
    pub fn parent(&self) -> Option<WindowHandle> {
        WindowHandle::from_ptr(unsafe { SDL_GetWindowParent(self.as_ptr()) })
    }

    #[doc(alias = "SDL_GetWindowPixelDensity")]
    pub fn pixel_density(&self) -> Result<f32> {
        let ret = unsafe { SDL_GetWindowPixelDensity(self.as_ptr()) };
        if ret == 0. {
            Err(Error::current())
        } else {
            Ok(ret)
        }
    }

    #[doc(alias = "SDL_GetWindowDisplayScale")]
    pub fn display_scale(&self) -> Result<f32> {
        let ret = unsafe { SDL_GetWindowDisplayScale(self.as_ptr()) };
        if ret == 0. {
            Err(Error::current())
        } else {
            Ok(ret)
        }
    }

    #[doc(alias = "SDL_GetWindowOpacity")]
    pub fn opacity(&self) -> Result<f32> {
        let ret = unsafe { SDL_GetWindowOpacity(self.as_ptr()) };
        if ret < 0. {
            Err(Error::current())
        } else {
            Ok(ret)
        }
    }

    #[doc(alias = "SDL_GetWindowPixelFormat")]
    pub fn pixel_format(&self) -> SDL_PixelFormat {
        unsafe { SDL_GetWindowPixelFormat(self.as_ptr()) }
    }

    /// The returned pointer is managed by SDL and is valid for as long as the
    /// window is fullscreen.
    #[doc(alias = "SDL_GetWindowFullscreenMode")]
    pub fn fullscreen_mode(&self) -> Option<NonNull<SDL_DisplayMode>> {
        NonNull::new(unsafe { SDL_GetWindowFullscreenMode(self.as_ptr()) }.cast_mut())
    }

    #[doc(alias = "SDL_GetWindowICCProfile")]
    pub fn icc_profile(&self) -> Result<Box<[u8]>> {
        let mut size = MaybeUninit::uninit();
        let ptr = unsafe { SDL_GetWindowICCProfile(self.as_ptr(), size.as_mut_ptr()) };

        // SAFETY: On success, SDL allocates `size` bytes.
        unsafe { Box::from_raw_parts_nullck(ptr.cast(), size.assume_init()) }
    }

    #[doc(alias = "SDL_GetWindowAspectRatio")]
    pub fn aspect_ratio(&self) -> Result<(f32, f32)> {
        let mut min = MaybeUninit::uninit();
        let mut max = MaybeUninit::uninit();

        unsafe {
            if SDL_GetWindowAspectRatio(self.as_ptr(), min.as_mut_ptr(), max.as_mut_ptr()) {
                Ok((min.assume_init(), max.assume_init()))
            } else {
                Err(Error::current())
            }
        }
    }

    /// Returns the border sizes in the order `(top, left, bottom, right)`.
    #[doc(alias = "SDL_GetWindowBordersSize")]
    pub fn borders_size(&self) -> Result<(i32, i32, i32, i32)> {
        let mut top = MaybeUninit::uninit();
        let mut left = MaybeUninit::uninit();
        let mut bottom = MaybeUninit::uninit();
        let mut right = MaybeUninit::uninit();

        unsafe {
            if SDL_GetWindowBordersSize(
                self.as_ptr(),
                top.as_mut_ptr(),
                left.as_mut_ptr(),
                bottom.as_mut_ptr(),
                right.as_mut_ptr(),
            ) {
                Ok((
                    top.assume_init(),
                    left.assume_init(),
                    bottom.assume_init(),
                    right.assume_init(),
                ))
            } else {
                Err(Error::current())
            }
        }
    }

    #[doc(alias = "SDL_GetWindowSafeArea")]
    pub fn safe_area(&self) -> Result<RectI32> {
        let mut ret = MaybeUninit::uninit();

        unsafe {
            if SDL_GetWindowSafeArea(self.as_ptr(), ret.as_mut_ptr()) {
                Ok(std::mem::transmute_copy(ret.assume_init_ref()))
            } else {
                Err(Error::current())
            }
        }
    }

    #[doc(alias = "SDL_GetWindowMouseRect")]
    pub fn mouse_rect(&self) -> Option<NonNull<RectI32>> {
        NonNull::new(unsafe { SDL_GetWindowMouseRect(self.as_ptr()).cast::<RectI32>() }.cast_mut())
    }

    #[doc(alias = "SDL_GetWindowSurface")]
    pub fn surface(&self) -> Result<Surface> {
        Surface::from_ptr(unsafe { SDL_GetWindowSurface(self.as_ptr()) })
    }

    #[doc(alias = "SDL_GetWindowSurfaceVSync")]
    pub fn surface_vsync(&self) -> Result<i32> {
        let mut ret = MaybeUninit::uninit();

        unsafe {
            if SDL_GetWindowSurfaceVSync(self.as_ptr(), ret.as_mut_ptr()) {
                Ok(ret.assume_init())
            } else {
                Err(Error::current())
            }
        }
    }

    #[doc(alias = "SDL_GetWindowKeyboardGrab")]
    pub fn keyboard_grabbed(&self) -> bool {
        unsafe { SDL_GetWindowKeyboardGrab(self.as_ptr()) }
    }

    #[doc(alias = "SDL_GetWindowMouseGrab")]
    pub fn mouse_grabbed(&self) -> bool {
        unsafe { SDL_GetWindowMouseGrab(self.as_ptr()) }
    }

    #[doc(alias = "SDL_GetWindowProgressState")]
    pub fn progress_state(&self) -> Result<ProgressState> {
        let ps = unsafe { SDL_GetWindowProgressState(self.as_ptr()) };
        if ps == SDL_ProgressState::INVALID {
            Err(Error::current())
        } else {
            type Src = SDL_ProgressState;
            type Dst = ProgressState;

            Ok(unsafe { transmute::<Src, Dst>(ps) })
        }
    }

    #[doc(alias = "SDL_GetWindowProgressValue")]
    pub fn progress_value(&self) -> f32 {
        unsafe { SDL_GetWindowProgressValue(self.as_ptr()) }
    }

    #[doc(alias = "SDL_WindowHasSurface")]
    pub fn has_surface(&self) -> bool {
        unsafe { SDL_WindowHasSurface(self.as_ptr()) }
    }

    #[doc(alias = "SDL_SetWindowSize")]
    pub fn set_size(&self, size: PointI32) -> Result<()> {
        to_result(unsafe { SDL_SetWindowSize(self.as_ptr(), size.x, size.y) })
    }

    #[doc(alias = "SDL_SetWindowMinimumSize")]
    pub fn set_min_size(&self, size: PointI32) -> Result<()> {
        to_result(unsafe { SDL_SetWindowMinimumSize(self.as_ptr(), size.x, size.y) })
    }

    #[doc(alias = "SDL_SetWindowMaximumSize")]
    pub fn set_max_size(&self, size: PointI32) -> Result<()> {
        to_result(unsafe { SDL_SetWindowMaximumSize(self.as_ptr(), size.x, size.y) })
    }

    #[doc(alias = "SDL_SetWindowPosition")]
    pub fn set_pos(&self, pos: PointI32) -> Result<()> {
        to_result(unsafe { SDL_SetWindowPosition(self.as_ptr(), pos.x, pos.y) })
    }

    #[doc(alias = "SDL_SetWindowTitle")]
    pub fn set_title(&self, title: &CStr) -> Result<()> {
        to_result(unsafe { SDL_SetWindowTitle(self.as_ptr(), title.as_ptr()) })
    }

    #[doc(alias = "SDL_SetWindowIcon")]
    pub fn set_icon(&self, icon: Ref<Surface>) -> Result<()> {
        to_result(unsafe { SDL_SetWindowIcon(self.as_ptr(), icon.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_SetWindowShape")]
    pub fn set_shape(&self, shape: Ref<Surface>) -> Result<()> {
        to_result(unsafe { SDL_SetWindowShape(self.as_ptr(), shape.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_SetWindowAspectRatio")]
    pub fn set_aspect_ratio(&self, min: f32, max: f32) -> Result<()> {
        to_result(unsafe { SDL_SetWindowAspectRatio(self.as_ptr(), min, max) })
    }

    #[doc(alias = "SDL_SetWindowBordered")]
    pub fn set_bordered(&self, bordered: bool) -> Result<()> {
        to_result(unsafe { SDL_SetWindowBordered(self.as_ptr(), bordered) })
    }

    #[doc(alias = "SDL_SetWindowResizable")]
    pub fn set_resizable(&self, value: bool) -> Result<()> {
        to_result(unsafe { SDL_SetWindowResizable(self.handle.as_ptr(), value) })
    }

    #[doc(alias = "SDL_SetWindowAlwaysOnTop")]
    pub fn set_always_on_top(&self, on_top: bool) -> Result<()> {
        to_result(unsafe { SDL_SetWindowAlwaysOnTop(self.as_ptr(), on_top) })
    }

    #[doc(alias = "SDL_SetWindowFullscreen")]
    pub fn set_fullscreen(&self, fullscreen: bool) -> Result<()> {
        to_result(unsafe { SDL_SetWindowFullscreen(self.as_ptr(), fullscreen) })
    }

    #[doc(alias = "SDL_SetWindowFullscreenMode")]
    pub fn set_fullscreen_mode(&self, mode: Option<&SDL_DisplayMode>) -> Result<()> {
        to_result(unsafe { SDL_SetWindowFullscreenMode(self.as_ptr(), opt2ptr(mode)) })
    }

    #[doc(alias = "SDL_SetWindowKeyboardGrab")]
    pub fn set_keyboard_grab(&self, grabbed: bool) -> Result<()> {
        to_result(unsafe { SDL_SetWindowKeyboardGrab(self.as_ptr(), grabbed) })
    }

    #[doc(alias = "SDL_SetWindowMouseGrab")]
    pub fn set_mouse_grab(&self, grabbed: bool) -> Result<()> {
        to_result(unsafe { SDL_SetWindowMouseGrab(self.as_ptr(), grabbed) })
    }

    #[doc(alias = "SDL_SetWindowMouseRect")]
    pub fn set_mouse_rect(&self, rect: Option<&RectI32>) -> Result<()> {
        to_result(unsafe { SDL_SetWindowMouseRect(self.as_ptr(), opt2ptr(rect).cast()) })
    }

    #[doc(alias = "SDL_SetWindowOpacity")]
    pub fn set_opacity(&self, opacity: f32) -> Result<()> {
        to_result(unsafe { SDL_SetWindowOpacity(self.as_ptr(), opacity) })
    }

    #[doc(alias = "SDL_SetWindowParent")]
    pub fn set_parent(&self, parent: Option<Ref<Window>>) -> Result<()> {
        let ptr = parent.map_or(std::ptr::null_mut(), |p| p.handle.as_ptr());
        to_result(unsafe { SDL_SetWindowParent(self.as_ptr(), ptr) })
    }

    #[doc(alias = "SDL_SetWindowModal")]
    pub fn set_modal(&self, modal: bool) -> Result<()> {
        to_result(unsafe { SDL_SetWindowModal(self.as_ptr(), modal) })
    }

    #[doc(alias = "SDL_SetWindowFocusable")]
    pub fn set_focusable(&self, focusable: bool) -> Result<()> {
        to_result(unsafe { SDL_SetWindowFocusable(self.as_ptr(), focusable) })
    }

    #[doc(alias = "SDL_ShowWindowSystemMenu")]
    pub fn show_system_menu(&self, pos: PointI32) -> Result<()> {
        to_result(unsafe { SDL_ShowWindowSystemMenu(self.as_ptr(), pos.x, pos.y) })
    }

    #[doc(alias = "SDL_SetWindowSurfaceVSync")]
    pub fn set_surface_vsync(&self, vsync: i32) -> Result<()> {
        to_result(unsafe { SDL_SetWindowSurfaceVSync(self.as_ptr(), vsync) })
    }

    #[doc(alias = "SDL_UpdateWindowSurface")]
    pub fn update_surface(&self) -> Result<()> {
        to_result(unsafe { SDL_UpdateWindowSurface(self.as_ptr()) })
    }

    #[doc(alias = "SDL_UpdateWindowSurfaceRects")]
    pub fn update_surface_rects(&self, rects: &[RectI32]) -> Result<()> {
        to_result(unsafe {
            SDL_UpdateWindowSurfaceRects(self.as_ptr(), rects.as_ptr().cast(), rects.len() as i32)
        })
    }

    #[doc(alias = "SDL_DestroyWindowSurface")]
    pub fn destroy_surface(&self) -> Result<()> {
        to_result(unsafe { SDL_DestroyWindowSurface(self.as_ptr()) })
    }

    #[doc(alias = "SDL_ShowWindow")]
    pub fn show(&self) -> Result<()> {
        to_result(unsafe { SDL_ShowWindow(self.as_ptr()) })
    }

    #[doc(alias = "SDL_HideWindow")]
    pub fn hide(&self) -> Result<()> {
        to_result(unsafe { SDL_HideWindow(self.as_ptr()) })
    }

    #[doc(alias = "SDL_RaiseWindow")]
    pub fn raise(&self) -> Result<()> {
        to_result(unsafe { SDL_RaiseWindow(self.as_ptr()) })
    }

    #[doc(alias = "SDL_MaximizeWindow")]
    pub fn maximize(&self) -> Result<()> {
        to_result(unsafe { SDL_MaximizeWindow(self.as_ptr()) })
    }

    #[doc(alias = "SDL_MinimizeWindow")]
    pub fn minimize(&self) -> Result<()> {
        to_result(unsafe { SDL_MinimizeWindow(self.as_ptr()) })
    }

    #[doc(alias = "SDL_RestoreWindow")]
    pub fn restore(&self) -> Result<()> {
        to_result(unsafe { SDL_RestoreWindow(self.as_ptr()) })
    }

    #[doc(alias = "SDL_SetWindowProgressState")]
    pub fn set_progress_state(&self, state: ProgressState) -> Result<()> {
        type Src = ProgressState;
        type Dst = SDL_ProgressState;

        to_result(unsafe {
            SDL_SetWindowProgressState(self.as_ptr(), transmute::<Src, Dst>(state))
        })
    }

    #[doc(alias = "SDL_SetWindowProgressValue")]
    pub fn set_progress_value(&self, value: f32) -> Result<()> {
        to_result(unsafe { SDL_SetWindowProgressValue(self.as_ptr(), value) })
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
    pub fn properties(&self) -> WindowProperties<'_> {
        unsafe {
            let id = SDL_GetWindowProperties(self.handle.as_ptr());
            let handle = PropertiesHandle::from_id(id).unwrap_unchecked();
            let r = Ref::from_handle(handle);

            WindowProperties::new(r)
        }
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
    pub fn builder(props: Ref<Properties>) -> WindowBuilder {
        WindowBuilder::new(props)
    }

    #[doc(alias = "SDL_CreateWindow")]
    pub fn new(title: &CStr, size: PointI32, flags: WindowFlags) -> Result<Self> {
        Self::from_ptr(unsafe { SDL_CreateWindow(title.as_ptr(), size.x, size.y, flags.into()) })
    }

    #[doc(alias = "SDL_CreatePopupWindow")]
    pub fn popup(
        parent: Ref<Window>,
        offset: PointI32,
        size: PointI32,
        flags: WindowFlags,
    ) -> Result<Self> {
        Self::from_ptr(unsafe {
            SDL_CreatePopupWindow(
                parent.handle.as_ptr(),
                offset.x,
                offset.y,
                size.x,
                size.y,
                flags.into(),
            )
        })
    }

    #[doc(alias = "SDL_CreateWindowAndRenderer")]
    pub fn with_renderer(
        title: &CStr,
        size: PointI32,
        flags: WindowFlags,
    ) -> Result<(Self, Renderer)> {
        let mut ret = MaybeUninit::<(*mut SDL_Window, *mut SDL_Renderer)>::uninit();
        let ptr = ret.as_mut_ptr();

        unsafe {
            if SDL_CreateWindowAndRenderer(
                title.as_ptr(),
                size.x,
                size.y,
                flags.into(),
                &raw mut (*ptr).0,
                &raw mut (*ptr).1,
            ) {
                // SAFETY: The above function succeeds only when both
                // the window and renderer are initialized.
                let init = ret.assume_init();
                let wnd = Self::from_ptr(init.0).unwrap_unchecked();
                let rnd = Renderer::from_ptr(init.1).unwrap_unchecked();

                Ok((wnd, rnd))
            } else {
                Err(Error::current())
            }
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
