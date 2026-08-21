//! API checklist ([source](https://wiki.libsdl.org/SDL3/CategoryRender)):
//! - [x] SDL_CreateGPURenderState
//! - [x] SDL_DestroyGPURenderState
//! - [x] SDL_SetGPURenderStateFragmentUniforms

use std::marker::PhantomData;

use sdl3_sys::{properties::SDL_PropertiesID, render::*};

use crate::{Result, gpu::*, renderer::Renderer, resource::Ref, resource_new, util::to_result};

pub struct RenderStateCreateInfo<'frag, 'sbin, 'sbin_t, 'sbin_s, 'stex, 'stex_t, 'sbuf, 'sbuf_b>(
    SDL_GPURenderStateCreateInfo,
    PhantomData<Ref<'frag, Shader>>,
    PhantomData<&'sbin [TextureSamplerBinding<'sbin_t, 'sbin_s>]>,
    PhantomData<&'stex [Ref<'stex_t, Texture>]>,
    PhantomData<&'sbuf [Ref<'sbuf_b, Buffer>]>,
);

impl<'frag, 'sbin, 'sbin_t, 'sbin_s, 'stex, 'stex_t, 'sbuf, 'sbuf_b>
    RenderStateCreateInfo<'frag, 'sbin, 'sbin_t, 'sbin_s, 'stex, 'stex_t, 'sbuf, 'sbuf_b>
{
    pub fn new(
        fragment_shader: Ref<'frag, Shader>,
        sampler_bindings: &'sbin [TextureSamplerBinding<'sbin_t, 'sbin_s>],
        storage_textures: &'stex [Ref<'stex_t, Texture>],
        storage_buffers: &'sbuf [Ref<'sbuf_b, Buffer>],
    ) -> Self {
        Self(
            SDL_GPURenderStateCreateInfo {
                fragment_shader: fragment_shader.as_ptr(),
                num_sampler_bindings: sampler_bindings.len() as _,
                sampler_bindings: sampler_bindings.as_ptr().cast(),
                num_storage_textures: storage_textures.len() as _,
                storage_textures: storage_textures.as_ptr().cast(),
                num_storage_buffers: storage_buffers.len() as _,
                storage_buffers: storage_buffers.as_ptr().cast(),
                // NOTE: No properties are read as of v3.4.14.
                props: SDL_PropertiesID::new(0),
            },
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
        )
    }
}

resource_new!(SDL_GPURenderState, RenderState, SDL_DestroyGPURenderState);

impl RenderStateHandle {
    #[doc(alias = "SDL_SetGPURenderStateFragmentUniforms")]
    pub fn set_fragment_uniforms(&self, slot_index: u32, data: &[u8]) -> Result {
        to_result(unsafe {
            SDL_SetGPURenderStateFragmentUniforms(
                self.as_ptr(),
                slot_index,
                data.as_ptr().cast(),
                data.len() as _,
            )
        })
    }
}

impl RenderState {
    #[doc(alias = "SDL_CreateGPURenderState")]
    pub fn new(rnd: Ref<Renderer>, ci: &RenderStateCreateInfo) -> Result<Self> {
        Self::from_ptr(unsafe { SDL_CreateGPURenderState(rnd.as_ptr(), &ci.0) })
    }
}
