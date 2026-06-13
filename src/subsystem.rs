use sdl3_sys::init::*;

use crate::{context::Context, defs::SdlResult, error::Error};

/// A handle to an SDL subsystem.
/// This doesn't actually de-initialize the
/// underlying subsystem upon being dropped;
/// that's left up to the context being destroyed.
/// For this reason, it cannot outlive the context.
pub struct Subsystem<const TYPE: u32> {}

impl<const N: u32> Subsystem<N> {
    #[doc(alias = "SDL_Init")]
    pub fn new(_: &Context) -> SdlResult<Self> {
        let res = unsafe { SDL_Init(SDL_InitFlags(N)) };
        if res {
            Ok(Self {})
        } else {
            Err(Error::current())
        }
    }
}

pub type Video = Subsystem<{ SDL_INIT_VIDEO.0 }>;
