use std::marker::PhantomData;

use sdl3_sys::init::*;

use crate::{context::Context, defs::SdlResult, error::Error};

/// A handle to an SDL subsystem.
/// This doesn't actually de-initialize the underlying subsystem upon being dropped;
/// that's left up to the [`Context`] being destroyed. Thus, it cannot outlive it.
pub struct Subsystem<'ctx, const TYPE: u32> {
    marker: PhantomData<&'ctx Context>,
}

impl<const N: u32> Subsystem<'_, N> {
    #[doc(alias = "SDL_Init")]
    pub fn new(_: &Context) -> SdlResult<Self> {
        let res = unsafe { SDL_Init(SDL_InitFlags::new(N)) };
        if res {
            Ok(Self {
                marker: PhantomData,
            })
        } else {
            Err(Error::current())
        }
    }
}

pub type Video<'ctx> = Subsystem<'ctx, { SDL_InitFlags::VIDEO.0 }>;
pub type Events<'ctx> = Subsystem<'ctx, { SDL_InitFlags::EVENTS.0 }>;
