//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_CreateGPUBuffer
//! - [x] SDL_DownloadFromGPUBuffer
//! - [x] SDL_ReleaseGPUBuffer
//! - [x] SDL_SetGPUBufferName
//! - [x] SDL_UploadToGPUBuffer

use std::{ffi::CStr, marker::PhantomData};

use bitmask_enum::bitmask;
use sdl3_sys::{gpu::*, properties::SDL_PropertiesID};

use crate::{Result, gpu::Cycle, resource::Ref, resource_new_no_drop};

use super::{copy_pass::CopyPass, device::Device, transfer_buffer::TransferBufferLocation};

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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub struct BufferRegion<'b>(SDL_GPUBufferRegion, PhantomData<Ref<'b, Buffer>>);
impl BufferRegion<'_> {
    pub fn new<'b>(buffer: Ref<'b, Buffer>, offset: u32, size: u32) -> BufferRegion<'b> {
        let buffer = buffer.handle.as_ptr();
        let inner = SDL_GPUBufferRegion {
            buffer,
            offset,
            size,
        };
        BufferRegion(inner, PhantomData)
    }
}

#[doc(alias = "SDL_GPUBufferBinding")]
#[derive(Clone, Copy)]
pub struct BufferBinding<'b>(
    pub(crate) SDL_GPUBufferBinding,
    PhantomData<Ref<'b, Buffer>>,
);
impl BufferBinding<'_> {
    pub fn new<'b>(buffer: Ref<'b, Buffer>, offset: u32) -> BufferBinding<'b> {
        BufferBinding(
            SDL_GPUBufferBinding {
                buffer: buffer.handle.as_ptr(),
                offset,
            },
            PhantomData,
        )
    }
}

#[doc(alias = "SDL_GPUBufferLocation")]
#[derive(Clone, Copy)]
pub struct BufferLocation<'b>(
    pub(crate) SDL_GPUBufferLocation,
    PhantomData<Ref<'b, Buffer>>,
);
impl BufferLocation<'_> {
    pub fn new<'b>(buffer: Ref<'b, Buffer>, offset: u32) -> BufferLocation<'b> {
        BufferLocation(
            SDL_GPUBufferLocation {
                buffer: buffer.handle.as_ptr(),
                offset,
            },
            PhantomData,
        )
    }
}

#[doc(alias = "SDL_GPUStorageBufferReadWriteBinding")]
#[derive(Clone, Copy)]
pub struct StorageBufferReadWriteBinding<'b>(
    SDL_GPUStorageBufferReadWriteBinding,
    PhantomData<Ref<'b, Buffer>>,
);

impl StorageBufferReadWriteBinding<'_> {
    pub fn new<'b>(buffer: Ref<'b, Buffer>, cycle: Cycle) -> StorageBufferReadWriteBinding<'b> {
        StorageBufferReadWriteBinding(
            SDL_GPUStorageBufferReadWriteBinding {
                buffer: buffer.handle.as_ptr(),
                cycle: cycle.into(),
                ..Default::default()
            },
            PhantomData,
        )
    }
}

resource_new_no_drop!(SDL_GPUBuffer, Buffer);
impl Buffer {
    #[doc(alias = "SDL_CreateGPUBuffer")]
    pub fn new(device: Ref<Device>, create_info: &BufferCreateInfo) -> Result<Self> {
        let handle = unsafe { SDL_CreateGPUBuffer(device.handle.as_ptr(), &create_info.0) };
        Self::from_ptr(handle)
    }

    #[doc(alias = "SDL_ReleaseGPUBuffer")]
    pub fn drop(self, device: Ref<Device>) {
        unsafe { SDL_ReleaseGPUBuffer(device.handle.as_ptr(), self.handle.as_ptr()) };
    }
}

impl BufferHandle {
    #[doc(alias = "SDL_UploadToGPUBuffer")]
    pub fn upload(
        &self,
        copy_pass: Ref<CopyPass>,
        src: &TransferBufferLocation,
        dst: &BufferRegion,
        cycle: Cycle,
    ) {
        unsafe { SDL_UploadToGPUBuffer(copy_pass.handle.as_ptr(), &src.0, &dst.0, cycle.into()) }
    }

    #[doc(alias = "SDL_DownloadFromGPUBuffer")]
    pub fn download(
        &self,
        copy_pass: Ref<CopyPass>,
        src: &BufferRegion,
        dst: &TransferBufferLocation,
    ) {
        unsafe { SDL_DownloadFromGPUBuffer(copy_pass.handle.as_ptr(), &src.0, &dst.0) };
    }

    #[doc(alias = "SDL_SetGPUBufferName")]
    pub fn set_name(&self, device: Ref<Device>, name: &CStr) {
        unsafe {
            SDL_SetGPUBufferName(device.handle.as_ptr(), self.handle.as_ptr(), name.as_ptr())
        };
    }
}
