use std::ffi::CStr;

use halcyon::{
    Context, properties::Properties, rect::Point, renderer::Renderer, resource::Resource,
    texture::Texture, window::Window,
};
use rustest::test;
use sdl3_sys::{
    pixels::SDL_PixelFormat,
    render::{SDL_PROP_TEXTURE_CREATE_WIDTH_NUMBER, SDL_TextureAccess},
};

/// `Texture::builder` with `SDL_CreateTextureWithProperties`.
#[test]
fn texture_builder() {
    let _ctx = Context::new();
    let props = Properties::new().unwrap();

    let wnd = Window::builder(props.as_ref())
        .hidden(true)
        .size(Point::new(128, 128))
        .build()
        .unwrap();

    let rnd = Renderer::builder(props.as_ref())
        .window(wnd.as_ref())
        .build()
        .unwrap();

    let tex = Texture::builder(rnd.as_ref(), props.as_ref())
        .format(SDL_PixelFormat::RGB24)
        .access(SDL_TextureAccess::STATIC)
        .size(Point::new(16, 16))
        .build()
        .unwrap();

    assert_eq!(tex.size(), Point::new(16.0, 16.0));
}

/// `Texture::properties` reflects what was used at creation.
#[test]
fn texture_properties() {
    let _ctx = Context::new();
    let props = Properties::new().unwrap();

    let wnd = Window::builder(props.as_ref())
        .hidden(true)
        .size(Point::new(128, 128))
        .build()
        .unwrap();

    let rnd = Renderer::builder(props.as_ref())
        .window(wnd.as_ref())
        .build()
        .unwrap();

    let tex = Texture::builder(rnd.as_ref(), props.as_ref())
        .format(SDL_PixelFormat::RGB24)
        .access(SDL_TextureAccess::STATIC)
        .size(Point::new(16, 16))
        .build()
        .unwrap();

    let tp = tex.properties();
    assert!(tp.format() == SDL_PixelFormat::RGB24);
    assert!(tp.access() == SDL_TextureAccess::STATIC);
    assert_eq!(tp.width(), 16);
    assert_eq!(tp.height(), 16);
}

/// `build_cleanup` clears the texture creation properties.
#[test]
fn texture_build_cleanup() {
    let _ctx = Context::new();
    let props = Properties::new().unwrap();

    let wnd = Window::builder(props.as_ref())
        .hidden(true)
        .size(Point::new(128, 128))
        .build()
        .unwrap();

    let rnd = Renderer::builder(props.as_ref())
        .window(wnd.as_ref())
        .build()
        .unwrap();

    let tex = Texture::builder(rnd.as_ref(), props.as_ref())
        .size(Point::new(16, 16))
        .build_cleanup()
        .unwrap();

    assert_eq!(tex.size(), Point::new(16.0, 16.0));

    let width_key = unsafe { CStr::from_ptr(SDL_PROP_TEXTURE_CREATE_WIDTH_NUMBER) };
    assert!(!props.as_ref().has(width_key));
}
