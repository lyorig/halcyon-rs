use halcyon::{context::Context, defs::SdlResult, gpu::*, subsystem::Video, traits::Resource};
use sdl3_sys::{
    gpu::{SDL_GPUBufferCreateInfo, SDL_GPUBufferUsageFlags, SDL_GPUShaderFormat},
    properties::SDL_PropertiesID,
};

const BUFFER_CREATE_INFO: SDL_GPUBufferCreateInfo = SDL_GPUBufferCreateInfo {
    usage: SDL_GPUBufferUsageFlags::COMPUTE_STORAGE_WRITE,
    size: 4096,
    props: SDL_PropertiesID::new(0),
};

fn foo() -> SdlResult {
    let ctx = unsafe { Context::new() };
    let _video = Video::new(&ctx);

    let device = GPUDevice::new(SDL_GPUShaderFormat::METALLIB, true)?;
    let cmdbuf = GPUCommandBuffer::new(device.as_ref())?;
    let _copy = GPUCopyPass::new(cmdbuf.as_ref())?;
    let buffer = GPUBuffer::new(device.as_ref(), &BUFFER_CREATE_INFO)?;

    buffer.drop(device.as_ref());

    Ok(())
}

fn main() {
    if let Err(e) = foo() {
        eprintln!("An unexpected error occurred: {e}");
    }
}
