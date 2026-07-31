//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_CreateGPUBuffer
//! - [x] SDL_DownloadFromGPUBuffer
//! - [x] SDL_ReleaseGPUBuffer
//! - [x] SDL_SetGPUBufferName
//! - [x] SDL_UploadToGPUBuffer

use std::ffi::CStr;

use bitmask_enum::bitmask;
use sdl3_sys::{gpu::*, properties::SDL_PropertiesID};

use crate::{Result, resource_no_drop, traits::Ref};

use super::{copy_pass::GPUCopyPass, device::GPUDevice, transfer_buffer::TransferBufferLocation};

#[bitmask(u32)]
#[doc(alias = "SDL_GPUBufferUsageFlags")]
pub enum BufferUsageFlags {
    Vertex = SDL_GPUBufferUsageFlags::VERTEX.0,
    Index = SDL_GPUBufferUsageFlags::INDEX.0,
    Indirect = SDL_GPUBufferUsageFlags::INDIRECT.0,
    GraphicsStorageRead = SDL_GPUBufferUsageFlags::GRAPHICS_STORAGE_READ.0,
    ComputeStorageRead = SDL_GPUBufferUsageFlags::COMPUTE_STORAGE_READ.0,
    ComputeStorageWrite = SDL_GPUBufferUsageFlags::COMPUTE_STORAGE_WRITE.0,
}

#[doc(alias = "SDL_GPUBufferCreateInfo")]
pub struct BufferCreateInfo(SDL_GPUBufferCreateInfo);
impl BufferCreateInfo {
    pub const fn new(usage: BufferUsageFlags, size: u32) -> Self {
        let usage = SDL_GPUBufferUsageFlags::new(usage.bits());
        let inner = SDL_GPUBufferCreateInfo {
            usage,
            size,
            props: SDL_PropertiesID::new(0),
        };
        Self(inner)
    }
}

#[doc(alias = "SDL_GPUBufferRegion")]
pub struct BufferRegion(SDL_GPUBufferRegion);
impl BufferRegion {
    pub fn new(buffer: Ref<GPUBuffer>, offset: u32, size: u32) -> Self {
        let buffer = buffer.handle.as_ptr();
        let inner = SDL_GPUBufferRegion {
            buffer,
            offset,
            size,
        };
        Self(inner)
    }
}

resource_no_drop!(GPUBuffer);
impl GPUBuffer {
    #[doc(alias = "SDL_CreateGPUBuffer")]
    pub fn new(device: Ref<GPUDevice>, create_info: &BufferCreateInfo) -> Result<Self> {
        let handle = unsafe { SDL_CreateGPUBuffer(device.handle.as_ptr(), &create_info.0) };
        Self::from_ptr(handle)
    }

    #[doc(alias = "SDL_ReleaseGPUBuffer")]
    pub fn drop(self, device: Ref<GPUDevice>) {
        unsafe { SDL_ReleaseGPUBuffer(device.handle.as_ptr(), self.handle.as_ptr()) };
    }
}

impl GPUBufferHandle {
    #[doc(alias = "SDL_UploadToGPUBuffer")]
    pub fn upload(
        &self,
        copy_pass: Ref<GPUCopyPass>,
        src: &TransferBufferLocation,
        dst: &BufferRegion,
        cycle: bool,
    ) {
        unsafe { SDL_UploadToGPUBuffer(copy_pass.handle.as_ptr(), &src.0, &dst.0, cycle) }
    }

    #[doc(alias = "SDL_DownloadFromGPUBuffer")]
    pub fn download(
        &self,
        copy_pass: Ref<GPUCopyPass>,
        src: &BufferRegion,
        dst: &TransferBufferLocation,
    ) {
        unsafe { SDL_DownloadFromGPUBuffer(copy_pass.handle.as_ptr(), &src.0, &dst.0) };
    }

    #[doc(alias = "SDL_SetGPUBufferName")]
    pub fn set_name(&self, device: Ref<GPUDevice>, name: &CStr) {
        unsafe {
            SDL_SetGPUBufferName(device.handle.as_ptr(), self.handle.as_ptr(), name.as_ptr())
        };
    }
}
