use std::marker::PhantomData;

use sdl3_sys::init::*;

use crate::{context::Context, defs::SdlResult, error};

/// A handle to an SDL subsystem.
/// This doesn't actually de-initialize the
/// underlying subsystem upon being dropped;
/// that's left up to the context being destroyed.
/// For this reason, it cannot outlive the context.
pub struct Subsystem<'ctx, const TYPE: u32> {
    // Subsystems are tied to a context's lifetime.
    marker: PhantomData<&'ctx Context>,
}

impl<'a, const N: u32> Subsystem<'a, N> {
    pub fn new(_: &'a Context) -> SdlResult<Self> {
        let res = unsafe { SDL_Init(N) };
        if res {
            Ok(Self {
                marker: PhantomData,
            })
        } else {
            Err(error::get())
        }
    }
}

pub type Video<'a> = Subsystem<'a, SDL_INIT_VIDEO>;
