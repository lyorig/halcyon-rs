use std::ffi::CStr;

use halcyon::{
    Context, properties::Properties, rect::Point, renderer::Renderer, resource::Resource,
    texture::Texture, window::Window,
};
use rustest::{Result, main, test};
use sdl3_sys::{
    pixels::SDL_PixelFormat,
    render::{SDL_PROP_TEXTURE_CREATE_WIDTH_NUMBER, SDL_TextureAccess},
};

/// `Texture::builder` with `SDL_CreateTextureWithProperties`.
#[test]
fn builder() -> Result {
    let _ctx = Context::new();
    let props = Properties::new()?;

    let wnd = Window::builder(props.as_ref())
        .hidden(true)
        .size(Point::new(128, 128))
        .build()?;

    let rnd = Renderer::builder(props.as_ref())
        .window(wnd.as_ref())
        .build()?;

    let tex = Texture::builder(rnd.as_ref(), props.as_ref())
        .format(SDL_PixelFormat::RGB24)
        .access(SDL_TextureAccess::STATIC)
        .size(Point::new(16, 16))
        .build()?;

    assert_eq!(tex.size(), Point::new(16.0, 16.0));

    Ok(())
}

/// `Texture::properties` reflects what was used at creation.
#[test]
fn properties() -> Result {
    let _ctx = Context::new();
    let props = Properties::new()?;

    let wnd = Window::builder(props.as_ref())
        .hidden(true)
        .size(Point::new(128, 128))
        .build()?;

    let rnd = Renderer::builder(props.as_ref())
        .window(wnd.as_ref())
        .build()?;

    let tex = Texture::builder(rnd.as_ref(), props.as_ref())
        .format(SDL_PixelFormat::RGB24)
        .access(SDL_TextureAccess::STATIC)
        .size(Point::new(16, 16))
        .build()?;

    let tp = tex.properties();
    assert!(tp.format() == SDL_PixelFormat::RGB24);
    assert!(tp.access() == SDL_TextureAccess::STATIC);
    assert_eq!(tp.width(), 16);
    assert_eq!(tp.height(), 16);

    Ok(())
}

/// `build_cleanup` clears the texture creation properties.
#[test]
fn build_cleanup() -> Result {
    let _ctx = Context::new();
    let props = Properties::new()?;

    let wnd = Window::builder(props.as_ref())
        .hidden(true)
        .size(Point::new(128, 128))
        .build()?;

    let rnd = Renderer::builder(props.as_ref())
        .window(wnd.as_ref())
        .build()?;

    let tex = Texture::builder(rnd.as_ref(), props.as_ref())
        .size(Point::new(16, 16))
        .build_cleanup()?;

    assert_eq!(tex.size(), Point::new(16.0, 16.0));

    let width_key = unsafe { CStr::from_ptr(SDL_PROP_TEXTURE_CREATE_WIDTH_NUMBER) };
    assert!(!props.as_ref().has(width_key));

    Ok(())
}

#[main]
fn main() {}
