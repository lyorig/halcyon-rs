use sdl3_sys::gpu::*;

struct Device {
    handle: *mut SDL_GPUDevice,
}

impl Device {
    pub fn new(formats: SDL_GPUShaderFormat, debug_mode: bool) -> Self {
        let handle = unsafe { SDL_CreateGPUDevice(formats, debug_mode, std::ptr::null()) };
        Self { handle }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { SDL_DestroyGPUDevice(self.handle) };
    }
}

/// NOTE: This struct has to be dropped manually via the
/// [`ComputePipeline::drop`] method, since it requires an extra
/// [`Device`] parameter! Not doing so is technically safe in the Rust sense,
/// but still constitutes a memory leak.
struct ComputePipeline {
    handle: *mut SDL_GPUComputePipeline,
}

impl ComputePipeline {
    pub fn new(device: &Device, create_info: &SDL_GPUComputePipelineCreateInfo) -> Self {
        let handle = unsafe { SDL_CreateGPUComputePipeline(device.handle, create_info) };
        Self { handle }
    }

    pub fn drop(self, device: &Device) {
        unsafe { SDL_ReleaseGPUComputePipeline(device.handle, self.handle) };
    }
}

struct ComputePass {
    handle: *mut SDL_GPUComputePass,
}

impl ComputePass {
    pub fn dispatch(&self, (x, y, z): (u32, u32, u32)) {
        unsafe { SDL_DispatchGPUCompute(self.handle, x, y, z) }
    }
}

impl Drop for ComputePass {
    fn drop(&mut self) {
        unsafe { SDL_EndGPUComputePass(self.handle) }
    }
}
