use halcyon::{context::Context, defs::SdlResult, gpu::*, subsystem::Video, traits::Resource};

const BUFFER_CREATE_INFO: BufferCreateInfo =
    BufferCreateInfo::new(BufferUsageFlags::ComputeStorageWrite, 4096);

fn foo() -> SdlResult {
    let ctx = unsafe { Context::new() };
    let _video = Video::new(&ctx);

    let device = GPUDevice::new(ShaderFormats::Msl, true)?;
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
