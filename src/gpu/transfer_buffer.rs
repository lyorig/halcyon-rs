//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_CreateGPUTransferBuffer
//! - [x] SDL_MapGPUTransferBuffer
//! - [x] SDL_ReleaseGPUTransferBuffer
//! - [x] SDL_UnmapGPUTransferBuffer

use std::ptr::NonNull;

use sdl3_sys::{gpu::*, properties::SDL_PropertiesID};

use crate::{Result, error::Error, gpu::Cycle, resource::Ref, resource_new_no_drop};

use super::device::Device;

#[doc(alias = "SDL_GPUTransferBufferLocation")]
#[derive(Clone, Copy)]
pub struct TransferBufferLocation(pub(crate) SDL_GPUTransferBufferLocation);
impl TransferBufferLocation {
    pub fn new(tb: Ref<TransferBuffer>, offset: u32) -> Self {
        let transfer_buffer = tb.handle.as_ptr();
        let inner = SDL_GPUTransferBufferLocation {
            transfer_buffer,
            offset,
        };
        Self(inner)
    }
}

#[repr(i32)]
#[doc(alias = "SDL_GPUTransferBufferUsage")]
pub enum TransferBufferUsage {
    Upload = SDL_GPUTransferBufferUsage::UPLOAD.0,
    Download = SDL_GPUTransferBufferUsage::DOWNLOAD.0,
}

#[doc(alias = "SDL_GPUTransferBufferCreateInfo")]
#[derive(Clone, Copy)]
pub struct TransferBufferCreateInfo(SDL_GPUTransferBufferCreateInfo);
impl TransferBufferCreateInfo {
    pub const fn new(usage: TransferBufferUsage, size: u32) -> Self {
        Self(SDL_GPUTransferBufferCreateInfo {
            usage: SDL_GPUTransferBufferUsage::new(usage as _),
            size,
            props: SDL_PropertiesID::new(0),
        })
    }
}

resource_new_no_drop!(SDL_GPUTransferBuffer, TransferBuffer);
impl TransferBuffer {
    #[doc(alias = "SDL_CreateGPUTransferBuffer")]
    pub fn new(device: Ref<Device>, create_info: &TransferBufferCreateInfo) -> Result<Self> {
        let handle = unsafe { SDL_CreateGPUTransferBuffer(device.handle.as_ptr(), &create_info.0) };
        Self::from_ptr(handle)
    }

    #[doc(alias = "SDL_ReleaseGPUTransferBuffer")]
    pub fn drop(self, dev: Ref<Device>) {
        unsafe { SDL_ReleaseGPUTransferBuffer(dev.handle.as_ptr(), self.handle.as_ptr()) };
    }

    #[doc(alias = "SDL_MapGPUTransferBuffer")]
    pub fn map(&self, device: Ref<Device>, cycle: Cycle) -> Result<NonNull<u8>> {
        let ptr = unsafe {
            SDL_MapGPUTransferBuffer(device.handle.as_ptr(), self.handle.as_ptr(), cycle.into())
        };
        NonNull::new(ptr.cast()).ok_or_else(Error::current)
    }

    #[doc(alias = "SDL_UnmapGPUTransferBuffer")]
    pub fn unmap(&self, device: Ref<Device>) {
        unsafe { SDL_UnmapGPUTransferBuffer(device.handle.as_ptr(), self.handle.as_ptr()) };
    }
}
