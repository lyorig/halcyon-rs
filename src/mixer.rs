use sdl3_mixer_sys::mixer::{
    MIX_CreateMixerDevice, MIX_DestroyMixer, MIX_Init, MIX_Mixer, MIX_Quit,
};
use sdl3_sys::audio::{SDL_AudioDeviceID, SDL_AudioSpec};

use crate::{defs::SdlResult, error::Error, resource, util::opt2ptr};

pub struct Context;

impl Context {
    fn new() -> SdlResult<Self> {
        if unsafe { MIX_Init() } {
            Ok(Self {})
        } else {
            Err(Error::current())
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { MIX_Quit() };
    }
}

resource!(Mixer, MIX);

impl Mixer {
    fn new(id: SDL_AudioDeviceID, spec: Option<&SDL_AudioSpec>) -> SdlResult<Self> {
        let ptr = unsafe { MIX_CreateMixerDevice(id, opt2ptr(spec)) };
        Self::from_ptr(ptr)
    }
}
