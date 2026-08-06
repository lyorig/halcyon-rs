use std::ffi::{CStr, c_char, c_void};

use sdl3_sys::{
    pixels::{SDL_Colorspace, SDL_PixelFormat},
    render::*,
};

use crate::{
    gpu::{Device, DeviceHandle},
    properties::{Properties, PropertiesHandle},
    resource::Ref,
    surface::{Surface, SurfaceHandle},
    util::c_ptr_to_str,
    window::{Window, WindowHandle},
};

/// Read-only properties of a renderer, as documented by
/// [`SDL_GetRendererProperties`](https://wiki.libsdl.org/SDL3/SDL_GetRendererProperties).
///
/// Generic properties are returned bare since the docs guarantee their
/// existence; backend properties are returned as `Option` since they only
/// exist on their respective backends.
#[derive(Clone, Copy)]
pub struct RendererProperties<'a> {
    inner: Ref<'a, Properties>,
}

impl<'a> RendererProperties<'a> {
    pub(super) fn new(inner: Ref<'a, Properties>) -> Self {
        Self { inner }
    }

    fn get_str(&self, key: *const c_char) -> &str {
        let cstr = unsafe { CStr::from_ptr(key) };
        let s = self.inner.string(cstr, std::ptr::null());

        // SAFETY: Only called for properties whose existence the SDL docs guarantee.
        unsafe { c_ptr_to_str(s) }
    }

    fn opt_number(&self, key: *const c_char) -> Option<i64> {
        let cstr = unsafe { CStr::from_ptr(key) };
        self.inner.has(cstr).then(|| self.inner.number(cstr, 0))
    }

    fn opt_ptr(&self, key: *const c_char) -> Option<*mut c_void> {
        let cstr = unsafe { CStr::from_ptr(key) };
        let p = self.inner.pointer(cstr, std::ptr::null_mut());

        (!p.is_null()).then_some(p)
    }

    pub fn name(&self) -> &str {
        self.get_str(SDL_PROP_RENDERER_NAME_STRING)
    }

    pub fn window(&self) -> Option<Ref<'a, Window>> {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_RENDERER_WINDOW_POINTER) };
        let p = self.inner.pointer(cstr, std::ptr::null_mut());

        WindowHandle::from_ptr(p.cast()).map(|h| unsafe { Ref::from_handle(h) })
    }

    pub fn surface(&self) -> Option<Ref<'a, Surface>> {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_RENDERER_SURFACE_POINTER) };
        let p = self.inner.pointer(cstr, std::ptr::null_mut());

        SurfaceHandle::from_ptr(p.cast()).map(|h| unsafe { Ref::from_handle(h) })
    }

    pub fn vsync(&self) -> i64 {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_RENDERER_VSYNC_NUMBER) };
        self.inner.number(cstr, 0)
    }

    pub fn max_texture_size(&self) -> i64 {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_RENDERER_MAX_TEXTURE_SIZE_NUMBER) };
        self.inner.number(cstr, 0)
    }

    pub fn texture_formats(&self) -> &[SDL_PixelFormat] {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_RENDERER_TEXTURE_FORMATS_POINTER) };
        let begin = self
            .inner
            .pointer(cstr, std::ptr::null_mut())
            .cast::<SDL_PixelFormat>();

        let mut len = 0;
        while unsafe { begin.add(len).read() } != SDL_PixelFormat::UNKNOWN {
            len += 1;
        }
        unsafe { std::slice::from_raw_parts(begin, len) }
    }

    pub fn texture_wrapping(&self) -> bool {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_RENDERER_TEXTURE_WRAPPING_BOOLEAN) };
        self.inner.bool(cstr, false)
    }

    pub fn output_colorspace(&self) -> SDL_Colorspace {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_RENDERER_OUTPUT_COLORSPACE_NUMBER) };
        SDL_Colorspace(self.inner.number(cstr, 0) as u32)
    }

    pub fn hdr_enabled(&self) -> bool {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_RENDERER_HDR_ENABLED_BOOLEAN) };
        self.inner.bool(cstr, false)
    }

    pub fn sdr_white_point(&self) -> f32 {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_RENDERER_SDR_WHITE_POINT_FLOAT) };
        self.inner.float(cstr, 0.)
    }

    pub fn hdr_headroom(&self) -> f32 {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_RENDERER_HDR_HEADROOM_FLOAT) };
        self.inner.float(cstr, 0.)
    }

    pub fn d3d9_device(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_RENDERER_D3D9_DEVICE_POINTER)
    }

    pub fn d3d11_device(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_RENDERER_D3D11_DEVICE_POINTER)
    }

    pub fn d3d11_swapchain(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_RENDERER_D3D11_SWAPCHAIN_POINTER)
    }

    pub fn d3d12_device(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_RENDERER_D3D12_DEVICE_POINTER)
    }

    pub fn d3d12_swapchain(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_RENDERER_D3D12_SWAPCHAIN_POINTER)
    }

    pub fn d3d12_command_queue(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_RENDERER_D3D12_COMMAND_QUEUE_POINTER)
    }

    pub fn vulkan_instance(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_RENDERER_VULKAN_INSTANCE_POINTER)
    }

    pub fn vulkan_surface(&self) -> Option<i64> {
        self.opt_number(SDL_PROP_RENDERER_VULKAN_SURFACE_NUMBER)
    }

    pub fn vulkan_physical_device(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_RENDERER_VULKAN_PHYSICAL_DEVICE_POINTER)
    }

    pub fn vulkan_device(&self) -> Option<*mut c_void> {
        self.opt_ptr(SDL_PROP_RENDERER_VULKAN_DEVICE_POINTER)
    }

    pub fn vulkan_graphics_queue_family_index(&self) -> Option<i64> {
        self.opt_number(SDL_PROP_RENDERER_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX_NUMBER)
    }

    pub fn vulkan_present_queue_family_index(&self) -> Option<i64> {
        self.opt_number(SDL_PROP_RENDERER_VULKAN_PRESENT_QUEUE_FAMILY_INDEX_NUMBER)
    }

    pub fn vulkan_swapchain_image_count(&self) -> Option<i64> {
        self.opt_number(SDL_PROP_RENDERER_VULKAN_SWAPCHAIN_IMAGE_COUNT_NUMBER)
    }

    pub fn gpu_device(&self) -> Option<Ref<'a, Device>> {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_RENDERER_GPU_DEVICE_POINTER) };
        let p = self.inner.pointer(cstr, std::ptr::null_mut());

        DeviceHandle::from_ptr(p.cast()).map(|h| unsafe { Ref::from_handle(h) })
    }
}

impl std::ops::Deref for RendererProperties<'_> {
    type Target = PropertiesHandle;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
