//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_AcquireGPUCommandBuffer
//! - [x] SDL_AcquireGPUSwapchainTexture
//! - [x] SDL_SubmitGPUCommandBuffer
//! - [x] SDL_SubmitGPUCommandBufferAndAcquireFence
//! - [x] SDL_WaitAndAcquireGPUSwapchainTexture
//! - [x] SDL_CancelGPUCommandBuffer
//! - [x] SDL_BlitGPUTexture
//! - [x] SDL_GenerateMipmapsForGPUTexture
//! - [x] SDL_InsertGPUDebugLabel
//! - [x] SDL_PopGPUDebugGroup
//! - [x] SDL_PushGPUDebugGroup
//! - [x] SDL_PushGPUComputeUniformData
//! - [x] SDL_PushGPUFragmentUniformData
//! - [x] SDL_PushGPUVertexUniformData

use std::{ffi::CStr, marker::PhantomData, mem::MaybeUninit, ptr::NonNull};

use sdl3_sys::{gpu::*, surface::SDL_FlipMode};

use crate::{
    Result,
    color::RgbaF32,
    gpu::Cycle,
    impl_enum_transmute,
    resource::{Ref, Resource},
    resource_new_no_drop,
    util::{opt2ptr_mut, to_result},
    window::Window,
};

use super::{
    device::Device,
    fence::Fence,
    render_pass::LoadOp,
    sampler::Filter,
    texture::{BlitRegion, Texture, TextureHandle},
};

/// Converts a raw swapchain texture pointer into a reference.
/// A null pointer (e.g. too many frames in flight) yields `None`.
fn swapchain_texture<'a>(ptr: *mut SDL_GPUTexture) -> Option<Ref<'a, Texture>> {
    let handle = NonNull::new(ptr)?;
    let inner = TextureHandle { handle };
    Some(unsafe { Ref::from_handle(inner) })
}

#[repr(i32)]
#[derive(Clone, Copy)]
#[doc(alias = "SDL_FlipMode")]
pub enum FlipMode {
    None = SDL_FlipMode::NONE.0,
    Horizontal = SDL_FlipMode::HORIZONTAL.0,
    Vertical = SDL_FlipMode::VERTICAL.0,
    HorizontalAndVertical = SDL_FlipMode::HORIZONTAL_AND_VERTICAL.0,
}

impl_enum_transmute!(SDL_FlipMode, FlipMode);

#[doc(alias = "SDL_GPUBlitInfo")]
#[derive(Clone, Copy)]
pub struct BlitInfo<'s, 'd>(
    SDL_GPUBlitInfo,
    PhantomData<Ref<'s, Texture>>,
    PhantomData<Ref<'d, Texture>>,
);

impl<'s, 'd> BlitInfo<'s, 'd> {
    pub fn new(
        source: BlitRegion<'s>,
        destination: BlitRegion<'d>,
        load_op: LoadOp,
        clear_color: RgbaF32,
        flip_mode: FlipMode,
        filter: Filter,
        cycle: Cycle,
    ) -> Self {
        Self(
            SDL_GPUBlitInfo {
                source: source.0,
                destination: destination.0,
                load_op: SDL_GPULoadOp::new(load_op as _),
                clear_color: clear_color.into(),
                flip_mode: SDL_FlipMode::new(flip_mode as _),
                filter: SDL_GPUFilter::new(filter as _),
                cycle: cycle.into(),
                ..Default::default()
            },
            PhantomData,
            PhantomData,
        )
    }
}

resource_new_no_drop!(SDL_GPUCommandBuffer, CommandBuffer);
impl CommandBuffer {
    #[doc(alias = "SDL_AcquireGPUCommandBuffer")]
    pub fn new(device: Ref<Device>) -> Result<Self> {
        let handle = unsafe { SDL_AcquireGPUCommandBuffer(device.handle.as_ptr()) };
        Self::from_ptr(handle)
    }

    /// Creates a new [`CommandBuffer`], performs some operations on it, then submits it.
    ///
    /// Propagates [`Err`] returned by:
    /// - [`CommandBuffer::new`]
    /// - `op`
    /// - [`CommandBuffer::submit`]
    pub fn run<F: FnOnce(Ref<Self>) -> Result<()>>(device: Ref<Device>, op: F) -> Result<()> {
        let cmdbuf = Self::new(device)?;
        op(cmdbuf.as_ref())?;
        cmdbuf.submit()
    }

    /// Creates a new [`CommandBuffer`], performs some operations on it, then submits it, returning a fence.
    ///
    /// Propagates [`Err`] returned by:
    /// - [`CommandBuffer::new`]
    /// - `op`
    /// - [`CommandBuffer::submit_fence`]
    pub fn run_fence<F: FnOnce(Ref<Self>) -> Result<()>>(
        device: Ref<Device>,
        op: F,
    ) -> Result<Fence> {
        let cmdbuf = Self::new(device)?;
        op(cmdbuf.as_ref())?;
        cmdbuf.submit_fence()
    }

    #[doc(alias = "SDL_SubmitGPUCommandBuffer")]
    pub fn submit(self) -> Result<()> {
        to_result(unsafe { SDL_SubmitGPUCommandBuffer(self.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_SubmitGPUCommandBufferAndAcquireFence")]
    pub fn submit_fence(self) -> Result<Fence> {
        let fence = unsafe { SDL_SubmitGPUCommandBufferAndAcquireFence(self.handle.as_ptr()) };
        Fence::from_ptr(fence)
    }

    #[doc(alias = "SDL_CancelGPUCommandBuffer")]
    pub fn cancel(self) -> Result<()> {
        to_result(unsafe { SDL_CancelGPUCommandBuffer(self.handle.as_ptr()) })
    }
}

impl CommandBufferHandle {
    #[doc(alias = "SDL_AcquireGPUSwapchainTexture")]
    pub fn acquire_swapchain_texture(
        &self,
        wnd: Ref<Window>,
        (tex_x, tex_y): (Option<&mut u32>, Option<&mut u32>),
    ) -> Result<Option<Ref<'_, Texture>>> {
        let mut tex = MaybeUninit::uninit();
        let res = unsafe {
            SDL_AcquireGPUSwapchainTexture(
                self.handle.as_ptr(),
                wnd.handle.as_ptr(),
                tex.as_mut_ptr(),
                opt2ptr_mut(tex_x),
                opt2ptr_mut(tex_y),
            )
        };

        to_result(res).map(|()| swapchain_texture(unsafe { tex.assume_init() }))
    }

    #[doc(alias = "SDL_WaitAndAcquireGPUSwapchainTexture")]
    pub fn wait_for_swapchain_texture(
        &self,
        wnd: Ref<Window>,
        (tex_x, tex_y): (Option<&mut u32>, Option<&mut u32>),
    ) -> Result<Option<Ref<'_, Texture>>> {
        let mut tex = MaybeUninit::uninit();
        let res = unsafe {
            SDL_WaitAndAcquireGPUSwapchainTexture(
                self.handle.as_ptr(),
                wnd.handle.as_ptr(),
                tex.as_mut_ptr(),
                opt2ptr_mut(tex_x),
                opt2ptr_mut(tex_y),
            )
        };

        to_result(res).map(|()| swapchain_texture(unsafe { tex.assume_init() }))
    }

    #[doc(alias = "SDL_GenerateMipmapsForGPUTexture")]
    pub fn generate_mipmaps(&self, texture: Ref<Texture>) {
        unsafe { SDL_GenerateMipmapsForGPUTexture(self.handle.as_ptr(), texture.handle.as_ptr()) }
    }

    #[doc(alias = "SDL_InsertGPUDebugLabel")]
    pub fn insert_debug_label(&self, text: &CStr) {
        unsafe { SDL_InsertGPUDebugLabel(self.handle.as_ptr(), text.as_ptr()) }
    }

    #[doc(alias = "SDL_PushGPUDebugGroup")]
    pub fn push_debug_group(&self, name: &CStr) {
        unsafe { SDL_PushGPUDebugGroup(self.handle.as_ptr(), name.as_ptr()) }
    }

    #[doc(alias = "SDL_PopGPUDebugGroup")]
    pub fn pop_debug_group(&self) {
        unsafe { SDL_PopGPUDebugGroup(self.handle.as_ptr()) }
    }

    #[doc(alias = "SDL_PushGPUVertexUniformData")]
    pub fn push_vertex_uniform_data(&self, slot_index: u32, data: &[u8]) {
        unsafe {
            SDL_PushGPUVertexUniformData(
                self.handle.as_ptr(),
                slot_index,
                data.as_ptr().cast(),
                data.len() as _,
            );
        }
    }

    #[doc(alias = "SDL_PushGPUFragmentUniformData")]
    pub fn push_fragment_uniform_data(&self, slot_index: u32, data: &[u8]) {
        unsafe {
            SDL_PushGPUFragmentUniformData(
                self.handle.as_ptr(),
                slot_index,
                data.as_ptr().cast(),
                data.len() as _,
            );
        }
    }

    #[doc(alias = "SDL_PushGPUComputeUniformData")]
    pub fn push_compute_uniform_data(&self, slot_index: u32, data: &[u8]) {
        unsafe {
            SDL_PushGPUComputeUniformData(
                self.handle.as_ptr(),
                slot_index,
                data.as_ptr().cast(),
                data.len() as _,
            );
        }
    }

    #[doc(alias = "SDL_BlitGPUTexture")]
    pub fn blit(&self, info: &BlitInfo) {
        unsafe { SDL_BlitGPUTexture(self.handle.as_ptr(), &raw const info.0) }
    }
}
