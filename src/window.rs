use crate::error;
use crate::properties::Properties;
use crate::subsystem::Video;
use bitmask_enum::bitmask;
use sdl3_sys::video::*;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::mem::MaybeUninit;

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

pub enum WindowPropertiesBool {
    AlwaysOnTop,
    Borderless,
    ExternalGraphicsContext,
    Focusable,
    Fullscreen,
    Hidden,
    HighPixelDensity,
    Maximized,
    PopupMenu,
    Metal,
    Minimized,
    Modal,
    MouseGrabbed,
    OpenGL,
    Resizable,
    Transparent,
    Tooltip,
    Utility,
    Vulkan,
}

impl WindowPropertiesBool {
    const fn as_cstr(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(match self {
                WindowPropertiesBool::AlwaysOnTop => SDL_PROP_WINDOW_CREATE_ALWAYS_ON_TOP_BOOLEAN,
                WindowPropertiesBool::Borderless => SDL_PROP_WINDOW_CREATE_BORDERLESS_BOOLEAN,
                WindowPropertiesBool::ExternalGraphicsContext => {
                    SDL_PROP_WINDOW_CREATE_EXTERNAL_GRAPHICS_CONTEXT_BOOLEAN
                }
                WindowPropertiesBool::Focusable => SDL_PROP_WINDOW_CREATE_FOCUSABLE_BOOLEAN,
                WindowPropertiesBool::Fullscreen => SDL_PROP_WINDOW_CREATE_FULLSCREEN_BOOLEAN,
                WindowPropertiesBool::Hidden => SDL_PROP_WINDOW_CREATE_HIDDEN_BOOLEAN,
                WindowPropertiesBool::HighPixelDensity => {
                    SDL_PROP_WINDOW_CREATE_HIGH_PIXEL_DENSITY_BOOLEAN
                }
                WindowPropertiesBool::Maximized => SDL_PROP_WINDOW_CREATE_MAXIMIZED_BOOLEAN,
                WindowPropertiesBool::PopupMenu => SDL_PROP_WINDOW_CREATE_MENU_BOOLEAN,
                WindowPropertiesBool::Metal => SDL_PROP_WINDOW_CREATE_METAL_BOOLEAN,
                WindowPropertiesBool::Minimized => SDL_PROP_WINDOW_CREATE_MINIMIZED_BOOLEAN,
                WindowPropertiesBool::Modal => SDL_PROP_WINDOW_CREATE_MODAL_BOOLEAN,
                WindowPropertiesBool::MouseGrabbed => SDL_PROP_WINDOW_CREATE_MOUSE_GRABBED_BOOLEAN,
                WindowPropertiesBool::OpenGL => SDL_PROP_WINDOW_CREATE_OPENGL_BOOLEAN,
                WindowPropertiesBool::Resizable => SDL_PROP_WINDOW_CREATE_RESIZABLE_BOOLEAN,
                WindowPropertiesBool::Transparent => SDL_PROP_WINDOW_CREATE_TRANSPARENT_BOOLEAN,
                WindowPropertiesBool::Tooltip => SDL_PROP_WINDOW_CREATE_TOOLTIP_BOOLEAN,
                WindowPropertiesBool::Utility => SDL_PROP_WINDOW_CREATE_UTILITY_BOOLEAN,
                WindowPropertiesBool::Vulkan => SDL_PROP_WINDOW_CREATE_VULKAN_BOOLEAN,
            })
        }
    }
}

pub enum WindowPropertiesI64 {
    Height,
    Width,
    X,
    Y,
}

impl WindowPropertiesI64 {
    const fn as_cstr(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(match self {
                WindowPropertiesI64::Height => SDL_PROP_WINDOW_CREATE_HEIGHT_NUMBER,
                WindowPropertiesI64::Width => SDL_PROP_WINDOW_CREATE_WIDTH_NUMBER,
                WindowPropertiesI64::X => SDL_PROP_WINDOW_CREATE_X_NUMBER,
                WindowPropertiesI64::Y => SDL_PROP_WINDOW_CREATE_Y_NUMBER,
            })
        }
    }
}

pub enum WindowPropertiesCStr {
    Title,
}

impl WindowPropertiesCStr {
    const fn as_cstr(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(match self {
                WindowPropertiesCStr::Title => SDL_PROP_WINDOW_CREATE_TITLE_STRING,
            })
        }
    }
}

pub struct WindowProperties {
    props: Properties,
}

impl WindowProperties {
    pub fn new() -> Self {
        Self {
            props: Properties::new(),
        }
    }

    pub fn bool(&self, prop: WindowPropertiesBool, value: bool) -> &Self {
        self.props.bool(prop.as_cstr(), value).unwrap();

        self
    }

    pub fn i64(&self, prop: WindowPropertiesI64, value: i64) -> &Self {
        self.props.i64(prop.as_cstr(), value).unwrap();

        self
    }

    pub fn cstr(&self, prop: WindowPropertiesCStr, value: &CStr) -> &Self {
        self.props.cstr(prop.as_cstr(), value).unwrap();

        self
    }
}

pub struct Window<'a> {
    marker: PhantomData<&'a Video<'a>>,
    pub(crate) internal: *mut SDL_Window,
}

impl<'a> Window<'a> {
    pub fn new(
        _video: &'a Video,
        title: &CStr,
        width: i32,
        height: i32,
        flags: WindowFlags,
    ) -> Result<Self, CString> {
        Self::ctor(unsafe { SDL_CreateWindow(title.as_ptr(), width, height, flags.into()) })
    }

    pub fn from_properties(_video: &'a Video, props: &WindowProperties) -> Result<Self, CString> {
        Self::ctor(unsafe { SDL_CreateWindowWithProperties(props.props.id()) })
    }

    fn ctor(internal: *mut SDL_Window) -> Result<Self, CString> {
        if internal.is_null() {
            Err(error::get())
        } else {
            Ok(Self {
                marker: PhantomData,
                internal,
            })
        }
    }

    pub fn sync(&self) -> Result<(), CString> {
        crate::util::btur(unsafe { SDL_SyncWindow(self.internal) })
    }

    pub fn size(&self) -> (i32, i32) {
        debug_assert!(!self.internal.is_null());

        let mut ret = (MaybeUninit::uninit(), MaybeUninit::uninit());

        unsafe {
            SDL_GetWindowSize(self.internal, ret.0.as_mut_ptr(), ret.1.as_mut_ptr());

            (ret.0.assume_init(), ret.1.assume_init())
        }
    }
}

impl Drop for Window<'_> {
    fn drop(&mut self) {
        unsafe { SDL_DestroyWindow(self.internal) }
    }
}
