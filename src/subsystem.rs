use std::marker::PhantomData;

use sdl3_sys::init::*;

use crate::{Context, Result, error::Error};

/// A handle to an SDL subsystem.
/// This doesn't actually de-initialize the underlying subsystem upon being dropped;
/// that's left up to the [`Context`] being destroyed. Thus, it cannot outlive it.
///
/// # Usage with [`std::mem::ManuallyDrop`]
/// By default, dropping a subsystem de-initializes it.
/// However, since dropping a [`Context`] de-initializes everything "by force"
/// anyways, you can potentially avoid redundant FFI calls by wrapping this struct
/// in [`std::mem::ManuallyDrop`].
pub struct Subsystem<'ctx, const TYPE: u32> {
    marker: PhantomData<&'ctx Context>,
}

impl<const N: u32> Subsystem<'_, N> {
    const FLAG: SDL_InitFlags = SDL_InitFlags::new(N);

    #[doc(alias = "SDL_InitSubSystem")]
    pub fn new<'ctx>(_: &'ctx Context) -> Result<Subsystem<'ctx, N>> {
        let res = unsafe { SDL_InitSubSystem(Self::FLAG) };
        if res {
            Ok(Subsystem {
                marker: PhantomData,
            })
        } else {
            Err(Error::current())
        }
    }
}

impl<const N: u32> Drop for Subsystem<'_, N> {
    #[doc(alias = "SDL_QuitSubSystem")]
    fn drop(&mut self) {
        unsafe { SDL_QuitSubSystem(Self::FLAG) };
    }
}

pub type Video<'ctx> = Subsystem<'ctx, { SDL_InitFlags::VIDEO.0 }>;
pub type Events<'ctx> = Subsystem<'ctx, { SDL_InitFlags::EVENTS.0 }>;
