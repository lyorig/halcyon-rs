//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_ClaimWindowForGPUDevice
//! - [x] SDL_CreateGPUDevice
//! - [x] SDL_CreateGPUDeviceWithProperties
//! - [x] SDL_DestroyGPUDevice
//! - [x] SDL_GetGPUDeviceDriver
//! - [x] SDL_GetGPUDeviceProperties
//! - [x] SDL_GetGPUSwapchainTextureFormat
//! - [x] SDL_GPUTextureSupportsFormat
//! - [x] SDL_GPUTextureSupportsSampleCount
//! - [x] SDL_ReleaseWindowFromGPUDevice
//! - [x] SDL_SetGPUAllowedFramesInFlight
//! - [x] SDL_SetGPUSwapchainParameters
//! - [x] SDL_WaitForGPUFences
//! - [x] SDL_WaitForGPUIdle
//! - [x] SDL_WaitForGPUSwapchain
//! - [x] SDL_WindowSupportsGPUPresentMode
//! - [x] SDL_WindowSupportsGPUSwapchainComposition
//! - [x] SDL_GetGPUShaderFormats

use std::ffi::CStr;

use sdl3_sys::gpu::*;

use crate::{
    Result,
    error::Error,
    gpu::{EnableDebug, WaitAll},
    impl_enum_transmute, mod_reexport,
    properties::{Properties, PropertiesHandle},
    resource::Ref,
    resource_new,
    util::to_result,
    window::Window,
};

use super::{
    ShaderFormats,
    fence::Fence,
    texture::{SampleCount, TextureFormat, TextureType, TextureUsageFlags},
};

mod_reexport!(builder);
mod_reexport!(properties);

#[repr(i32)]
#[derive(Clone, Copy)]
#[doc(alias = "SDL_GPUPresentMode")]
pub enum PresentMode {
    Vsync = SDL_GPUPresentMode::VSYNC.0,
    Immediate = SDL_GPUPresentMode::IMMEDIATE.0,
    Mailbox = SDL_GPUPresentMode::MAILBOX.0,
}

#[repr(i32)]
#[derive(Clone, Copy)]
#[doc(alias = "SDL_GPUSwapchainComposition")]
pub enum SwapchainComposition {
    Sdr = SDL_GPUSwapchainComposition::SDR.0,
    SdrLinear = SDL_GPUSwapchainComposition::SDR_LINEAR.0,
    HdrExtendedLinear = SDL_GPUSwapchainComposition::HDR_EXTENDED_LINEAR.0,
    Hdr10St2084 = SDL_GPUSwapchainComposition::HDR10_ST2084.0,
}

impl_enum_transmute!(SDL_GPUPresentMode, PresentMode);
impl_enum_transmute!(SDL_GPUSwapchainComposition, SwapchainComposition);

resource_new!(SDL_GPUDevice, Device, SDL_DestroyGPUDevice);
impl Device {
    #[doc(alias = "SDL_CreateGPUDevice")]
    pub fn new(formats: ShaderFormats, debug: EnableDebug) -> Result<Self> {
        let fmts = SDL_GPUShaderFormat::new(formats.bits());
        let handle = unsafe { SDL_CreateGPUDevice(fmts, debug.into(), std::ptr::null()) };
        Self::from_ptr(handle)
    }

    /// Bind the builder to an existing property group.
    ///
    /// The device creation properties (`SDL_PROP_GPU_DEVICE_CREATE_*`)
    /// never collide with the window or renderer ones, so a single
    /// [`Properties`] can be shared between the three builders.
    pub fn builder(props: Ref<Properties>) -> DeviceBuilder {
        DeviceBuilder::new(props)
    }
}

impl DeviceHandle {
    #[doc(alias = "SDL_ClaimWindowForGPUDevice")]
    pub fn claim_window(&self, window: Ref<Window>) -> Result {
        to_result(unsafe {
            SDL_ClaimWindowForGPUDevice(self.handle.as_ptr(), window.handle.as_ptr())
        })
    }

    #[doc(alias = "SDL_ReleaseWindowFromGPUDevice")]
    pub fn release_window(&self, window: Ref<Window>) {
        unsafe { SDL_ReleaseWindowFromGPUDevice(self.handle.as_ptr(), window.handle.as_ptr()) };
    }

    #[doc(alias = "SDL_WindowSupportsGPUPresentMode")]
    pub fn window_supports_gpu_present_mode(&self, window: Ref<Window>, pm: PresentMode) -> bool {
        unsafe {
            SDL_WindowSupportsGPUPresentMode(
                self.handle.as_ptr(),
                window.handle.as_ptr(),
                SDL_GPUPresentMode::new(pm as _),
            )
        }
    }

    #[doc(alias = "SDL_WindowSupportsGPUSwapchainComposition")]
    pub fn window_supports_gpu_swapchain_composition(
        &self,
        window: Ref<Window>,
        sc: SwapchainComposition,
    ) -> bool {
        unsafe {
            SDL_WindowSupportsGPUSwapchainComposition(
                self.handle.as_ptr(),
                window.handle.as_ptr(),
                SDL_GPUSwapchainComposition::new(sc as _),
            )
        }
    }

    #[doc(alias = "SDL_WaitForGPUIdle")]
    pub fn wait_idle(&self) -> Result {
        to_result(unsafe { SDL_WaitForGPUIdle(self.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_WaitForGPUSwapchain")]
    pub fn wait_swapchain(&self, window: Ref<Window>) -> Result {
        to_result(unsafe { SDL_WaitForGPUSwapchain(self.handle.as_ptr(), window.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_WaitForGPUFences")]
    pub fn wait_fences(&self, wait_all: WaitAll, fences: &[Ref<Fence>]) -> Result {
        to_result(unsafe {
            SDL_WaitForGPUFences(
                self.handle.as_ptr(),
                wait_all.into(),
                fences.as_ptr().cast(),
                fences.len() as _,
            )
        })
    }

    #[doc(alias = "SDL_GetGPUDeviceDriver")]
    pub fn driver(&self) -> Result<&str> {
        let raw = unsafe { SDL_GetGPUDeviceDriver(self.handle.as_ptr()) };
        if raw.is_null() {
            Err(Error::current())
        } else {
            let cstr = unsafe { CStr::from_ptr(raw) };
            Ok(unsafe { str::from_utf8_unchecked(cstr.to_bytes()) })
        }
    }

    #[doc(alias = "SDL_GetGPUDeviceProperties")]
    pub fn properties(&self) -> DeviceProperties<'_> {
        let id = unsafe { SDL_GetGPUDeviceProperties(self.handle.as_ptr()) };
        let handle =
            PropertiesHandle::from_id(id).expect("A valid GPU device should have properties");

        let r = unsafe { Ref::from_handle(handle) };
        DeviceProperties::new(r)
    }

    #[doc(alias = "SDL_GetGPUSwapchainTextureFormat")]
    pub fn swapchain_texture_format(&self, window: Ref<Window>) -> TextureFormat {
        let fmt = unsafe {
            SDL_GetGPUSwapchainTextureFormat(self.handle.as_ptr(), window.handle.as_ptr())
        };
        fmt.into()
    }

    #[doc(alias = "SDL_GPUTextureSupportsFormat")]
    pub fn texture_supports_format(
        &self,
        format: TextureFormat,
        kind: TextureType,
        usage: TextureUsageFlags,
    ) -> bool {
        unsafe {
            SDL_GPUTextureSupportsFormat(
                self.handle.as_ptr(),
                SDL_GPUTextureFormat::new(format as _),
                SDL_GPUTextureType::new(kind as _),
                SDL_GPUTextureUsageFlags::new(usage.bits()),
            )
        }
    }

    #[doc(alias = "SDL_GPUTextureSupportsSampleCount")]
    pub fn texture_supports_sample_count(
        &self,
        format: TextureFormat,
        sample_count: SampleCount,
    ) -> bool {
        unsafe {
            SDL_GPUTextureSupportsSampleCount(
                self.handle.as_ptr(),
                SDL_GPUTextureFormat::new(format as _),
                SDL_GPUSampleCount::new(sample_count as _),
            )
        }
    }

    #[doc(alias = "SDL_SetGPUSwapchainParameters")]
    pub fn set_swapchain_parameters(
        &self,
        window: Ref<Window>,
        composition: SwapchainComposition,
        present_mode: PresentMode,
    ) -> Result {
        to_result(unsafe {
            SDL_SetGPUSwapchainParameters(
                self.handle.as_ptr(),
                window.handle.as_ptr(),
                SDL_GPUSwapchainComposition::new(composition as _),
                SDL_GPUPresentMode::new(present_mode as _),
            )
        })
    }

    #[doc(alias = "SDL_GetGPUShaderFormats")]
    pub fn shader_formats(&self) -> ShaderFormats {
        let fmts = unsafe { SDL_GetGPUShaderFormats(self.handle.as_ptr()) };
        ShaderFormats::from_bits_retain(fmts.0)
    }

    #[doc(alias = "SDL_SetGPUAllowedFramesInFlight")]
    pub fn set_allowed_frames_in_flight(&self, n: u32) -> Result {
        to_result(unsafe { SDL_SetGPUAllowedFramesInFlight(self.handle.as_ptr(), n) })
    }
}
