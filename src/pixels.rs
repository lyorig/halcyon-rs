use sdl3_sys::{blendmode::SDL_BlendMode, surface::SDL_ScaleMode};

use crate::impl_enum_transmute;

/// A set of blend modes used in drawing operations.
///
/// # Remarks
///
/// These predefined blend modes are supported everywhere.
///
/// Additional values may be obtained from `SDL_ComposeCustomBlendMode`.
#[repr(u32)]
#[derive(Clone, Copy)]
#[doc(alias = "SDL_BlendMode")]
pub enum BlendMode {
    /// No blending: `dstRGBA = srcRGBA`.
    None = SDL_BlendMode::NONE.0,
    /// Alpha blending:
    /// `dstRGB = (srcRGB * srcA) + (dstRGB * (1-srcA))`,
    /// `dstA = srcA + (dstA * (1-srcA))`.
    Blend = SDL_BlendMode::BLEND.0,
    /// Pre-multiplied alpha blending:
    /// `dstRGBA = srcRGBA + (dstRGBA * (1-srcA))`.
    BlendPremultiplied = SDL_BlendMode::BLEND_PREMULTIPLIED.0,
    /// Additive blending:
    /// `dstRGB = (srcRGB * srcA) + dstRGB`, `dstA = dstA`.
    Add = SDL_BlendMode::ADD.0,
    /// Pre-multiplied additive blending:
    /// `dstRGB = srcRGB + dstRGB`, `dstA = dstA`.
    AddPremultiplied = SDL_BlendMode::ADD_PREMULTIPLIED.0,
    /// Color modulate: `dstRGB = srcRGB * dstRGB`, `dstA = dstA`.
    Mod = SDL_BlendMode::MOD.0,
    /// Color multiply:
    /// `dstRGB = (srcRGB * dstRGB) + (dstRGB * (1-srcA))`, `dstA = dstA`.
    Mul = SDL_BlendMode::MUL.0,
}

impl_enum_transmute!(SDL_BlendMode, BlendMode);

/// The scaling mode.
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "SDL_ScaleMode")]
pub enum ScaleMode {
    /// Nearest pixel sampling.
    Nearest = SDL_ScaleMode::NEAREST.0,
    /// Linear filtering.
    Linear = SDL_ScaleMode::LINEAR.0,
    /// Nearest pixel sampling with improved scaling for pixel art.
    PixelArt = SDL_ScaleMode::PIXELART.0,
}

impl_enum_transmute!(SDL_ScaleMode, ScaleMode);
