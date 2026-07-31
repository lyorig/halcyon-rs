//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_ClaimWindowForGPUDevice
//! - [x] SDL_CreateGPUDevice
//! - [x] SDL_DestroyGPUDevice
//! - [x] SDL_GetGPUDeviceDriver
//! - [x] SDL_ReleaseWindowFromGPUDevice
//! - [x] SDL_SetGPUAllowedFramesInFlight
//! - [x] SDL_WaitForGPUFences
//! - [x] SDL_WaitForGPUIdle
//! - [x] SDL_WaitForGPUSwapchain
//! - [x] SDL_WindowSupportsGPUPresentMode
//! - [x] SDL_WindowSupportsGPUSwapchainComposition

use std::ffi::CStr;

use sdl3_sys::gpu::*;

use crate::{
    Result, boolenum, error::Error, resource, traits::Ref, util::to_result, window::Window,
};

use super::{ShaderFormats, fence::GPUFence};

boolenum!(DeviceDebug);
boolenum!(WaitAll);

resource!(GPUDevice);
impl GPUDevice {
    #[doc(alias = "SDL_CreateGPUDevice")]
    pub fn new(formats: ShaderFormats, debug: DeviceDebug) -> Result<Self> {
        let fmts = SDL_GPUShaderFormat::new(formats.bits());
        let handle = unsafe { SDL_CreateGPUDevice(fmts, debug.into(), std::ptr::null()) };
        Self::from_ptr(handle)
    }
}

impl GPUDeviceHandle {
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
    pub fn window_supports_gpu_present_mode(
        &self,
        window: Ref<Window>,
        pm: SDL_GPUPresentMode,
    ) -> bool {
        unsafe {
            SDL_WindowSupportsGPUPresentMode(self.handle.as_ptr(), window.handle.as_ptr(), pm)
        }
    }

    #[doc(alias = "SDL_WindowSupportsGPUSwapchainComposition")]
    pub fn window_supports_gpu_swapchain_composition(
        &self,
        window: Ref<Window>,
        sc: SDL_GPUSwapchainComposition,
    ) -> bool {
        unsafe {
            SDL_WindowSupportsGPUSwapchainComposition(
                self.handle.as_ptr(),
                window.handle.as_ptr(),
                sc,
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
    pub fn wait_fences(&self, wait_all: WaitAll, fences: &[Ref<GPUFence>]) -> Result {
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

    #[doc(alias = "SDL_SetGPUAllowedFramesInFlight")]
    pub fn set_allowed_frames_in_flight(&self, n: u32) -> Result {
        to_result(unsafe { SDL_SetGPUAllowedFramesInFlight(self.handle.as_ptr(), n) })
    }
}
