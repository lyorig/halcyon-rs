use std::mem::ManuallyDrop;

use halcyon::{
    Context, Result, gpu::*, rect::Point, resource::Resource, subsystem::Video, window::Window,
};

const BUFFER_CREATE_INFO: BufferCreateInfo =
    BufferCreateInfo::new(BufferUsageFlags::ComputeStorageWrite, 4096);

const TCI: TextureCreateInfo = TextureCreateInfo::new(
    TextureType::_2d,
    TextureFormat::R8g8b8a8Uint,
    TextureUsageFlags::ColorTarget,
    Point::new(16, 16),
    1,
    1,
    SampleCount::One,
);

fn run() -> Result {
    let ctx = Context::new();
    let _video = ManuallyDrop::new(Video::new(&ctx)?);

    let device = Device::new(ShaderFormats::Msl, DeviceDebug::Yes)?;
    let cmdbuf = CommandBuffer::new(device.as_ref())?;
    let _copy = CopyPass::new(cmdbuf.as_ref())?;
    let buffer = Buffer::new(device.as_ref(), &BUFFER_CREATE_INFO)?;

    let wnd1 = Window::new(c"W1", Point::new(20, 20), Default::default())?;
    let wnd2 = Window::new(c"W2", Point::new(20, 20), Default::default())?;

    device.claim_window(wnd1.as_ref())?;
    device.claim_window(wnd2.as_ref())?;

    let tex = Texture::new(device.as_ref(), &TCI)?;
    tex.drop(device.as_ref());

    buffer.drop(device.as_ref());

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("An unexpected error occurred: {e}");
    }
}
