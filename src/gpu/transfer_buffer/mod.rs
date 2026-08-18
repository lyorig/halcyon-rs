//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_CreateGPUTransferBuffer
//! - [x] SDL_MapGPUTransferBuffer
//! - [x] SDL_ReleaseGPUTransferBuffer
//! - [x] SDL_UnmapGPUTransferBuffer

use std::{marker::PhantomData, ptr::NonNull};

use sdl3_sys::{gpu::*, properties::SDL_PropertiesID};

use crate::{
    Result, error::Error, gpu::Cycle, impl_enum_transmute, mod_reexport, properties::Properties,
    resource::Ref, resource_new_no_drop,
};

use super::device::Device;

mod_reexport!(builder);

#[doc(alias = "SDL_GPUTransferBufferLocation")]
#[derive(Clone, Copy)]
pub struct TransferBufferLocation<'tb>(
    pub(crate) SDL_GPUTransferBufferLocation,
    PhantomData<&'tb TransferBuffer>,
);

impl<'tb> TransferBufferLocation<'tb> {
    pub fn new(tb: Ref<'tb, TransferBuffer>, offset: u32) -> Self {
        let transfer_buffer = tb.handle.as_ptr();
        let inner = SDL_GPUTransferBufferLocation {
            transfer_buffer,
            offset,
        };

        Self(inner, PhantomData)
    }
}

#[repr(i32)]
#[derive(Clone, Copy)]
#[doc(alias = "SDL_GPUTransferBufferUsage")]
pub enum TransferBufferUsage {
    Upload = SDL_GPUTransferBufferUsage::UPLOAD.0,
    Download = SDL_GPUTransferBufferUsage::DOWNLOAD.0,
}

impl_enum_transmute!(SDL_GPUTransferBufferUsage, TransferBufferUsage);

#[doc(alias = "SDL_GPUTransferBufferCreateInfo")]
#[derive(Clone, Copy)]
pub struct TransferBufferCreateInfo<'p>(
    SDL_GPUTransferBufferCreateInfo,
    PhantomData<Ref<'p, Properties>>,
);
impl<'p> TransferBufferCreateInfo<'p> {
    pub fn builder(props: Ref<'p, Properties>) -> TransferBufferCreateInfoBuilder<'p> {
        TransferBufferCreateInfoBuilder::new(props)
    }

    pub const fn new(usage: TransferBufferUsage, size: u32) -> Self {
        Self(
            SDL_GPUTransferBufferCreateInfo {
                usage: SDL_GPUTransferBufferUsage::new(usage as _),
                size,
                props: SDL_PropertiesID::new(0),
            },
            PhantomData,
        )
    }

    pub(super) fn new_with_props(
        usage: TransferBufferUsage,
        size: u32,
        props: Ref<'p, Properties>,
    ) -> Self {
        let mut info = Self::new(usage, size);
        info.0.props = props.id();
        info
    }
}

resource_new_no_drop!(SDL_GPUTransferBuffer, TransferBuffer);
impl TransferBuffer {
    /// Create a new [`TransferBuffer`].
    /// This doesn't map or write anything.
    ///
    /// # Important
    /// [`TransferBuffer::new_with`] should be preferred, as it both safer and more ergonomic,
    /// unless you have a particular reason to directly use this struct's methods. See its
    /// documentation for more info.
    #[doc(alias = "SDL_CreateGPUTransferBuffer")]
    pub fn new(device: Ref<Device>, create_info: &TransferBufferCreateInfo) -> Result<Self> {
        let handle = unsafe { SDL_CreateGPUTransferBuffer(device.handle.as_ptr(), &create_info.0) };
        Self::from_ptr(handle)
    }

    /// Creates a new [`TransferBuffer`], maps it, calls `write` with the mapped data, then unmaps.
    /// Basically calls the following functions/methods, in order:
    /// 1. [`TransferBuffer::new`]
    /// 2. [`TransferBufferHandle::map`]
    /// 3. `write`
    /// 4. [`TransferBufferHandle::unmap`]
    ///
    /// The buffer must still be dropped manually.
    ///
    /// This should be your preferred way to use this struct.
    /// Although the aforementioned methods this constructor uses are public,
    /// they are mainly provided to cover the API surface, and enable advanced uses.
    pub fn new_with<F: FnOnce(&mut [u8])>(
        device: Ref<Device>,
        create_info: &TransferBufferCreateInfo,
        cycle: Cycle,
        write: F,
    ) -> Result<Self> {
        let tb = Self::new(device, create_info)?;
        let ptr = tb.map(device, cycle)?;
        let slice = unsafe {
            std::slice::from_raw_parts_mut(ptr.as_ptr().cast::<u8>(), create_info.0.size as _)
        };

        write(slice);

        tb.unmap(device);

        Ok(tb)
    }

    #[doc(alias = "SDL_ReleaseGPUTransferBuffer")]
    pub fn drop(self, device: Ref<Device>) {
        unsafe { SDL_ReleaseGPUTransferBuffer(device.handle.as_ptr(), self.handle.as_ptr()) };
    }
}

impl TransferBufferHandle {
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
