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
    util::to_result,
};

use bitmask_enum::bitmask;
use sdl3_sys::{
    render::{SDL_CreateWindowAndRenderer, SDL_GetRenderer, SDL_Renderer},
    video::*,
};

use std::{
    ffi::{CStr, c_char},
    mem::MaybeUninit,
    num::NonZero,
    ptr::NonNull,
};

pub mod builder;
pub mod properties;

pub use builder::*;
pub use properties::*;

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

    const unsafe fn from_raw_unchecked(raw: u32) -> Self {
        let inner = unsafe { NonZero::new_unchecked(raw) };
        Self { inner }
    }

    const fn as_sdl(&self) -> SDL_WindowID {
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
    pub fn properties(&self) -> WindowProperties<'_> {
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
    pub fn builder(props: Ref<Properties>) -> WindowBuilder {
        WindowBuilder::new(props)
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
