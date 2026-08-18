use sdl3_sys::blendmode::SDL_BlendMode;

#[repr(u32)]
#[derive(Clone, Copy)]
#[doc(alias = "SDL_BlendMode")]
pub enum BlendMode {
    None = SDL_BlendMode::NONE.0,
    Blend = SDL_BlendMode::BLEND.0,
    BlendPremultiplied = SDL_BlendMode::BLEND_PREMULTIPLIED.0,
    Add = SDL_BlendMode::ADD.0,
    AddPremultiplied = SDL_BlendMode::ADD_PREMULTIPLIED.0,
    Mod = SDL_BlendMode::MOD.0,
    Mul = SDL_BlendMode::MUL.0,
}

impl From<SDL_BlendMode> for BlendMode {
    fn from(value: SDL_BlendMode) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<BlendMode> for SDL_BlendMode {
    fn from(value: BlendMode) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
