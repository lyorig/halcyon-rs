use sdl3_sys::blendmode::SDL_BlendMode;

use crate::impl_enum_conversions;

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

impl_enum_conversions!(SDL_BlendMode, BlendMode);
