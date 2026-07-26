use halcyon::{
    Context, Result, gpu::*, rect::Point, subsystem::Video, traits::Resource, window::Window,
};

const BUFFER_CREATE_INFO: BufferCreateInfo =
    BufferCreateInfo::new(BufferUsageFlags::ComputeStorageWrite, 4096);

fn foo() -> Result {
    let ctx = Context::new();
    let _video = Video::new(&ctx);

    let device = GPUDevice::new(ShaderFormats::Msl, true)?;
    let cmdbuf = GPUCommandBuffer::new(device.as_ref())?;
    let _copy = GPUCopyPass::new(cmdbuf.as_ref())?;
    let buffer = GPUBuffer::new(device.as_ref(), &BUFFER_CREATE_INFO)?;

    let wnd1 = Window::new(c"W1", Point::new(20, 20), Default::default())?;
    let wnd2 = Window::new(c"W2", Point::new(20, 20), Default::default())?;

    device.claim_window(wnd1.as_ref())?;
    device.claim_window(wnd2.as_ref())?;

    buffer.drop(device.as_ref());

    Ok(())
}

fn main() {
    if let Err(e) = foo() {
        eprintln!("An unexpected error occurred: {e}");
    }
}
