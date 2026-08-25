//! Implementation checklist:
//! - [x] TTF_CreateGPUTextEngine
//! - [ ] TTF_CreateGPUTextEngineWithProperties
//! - [x] TTF_CreateRendererTextEngine
//! - [ ] TTF_CreateRendererTextEngineWithProperties
//! - [x] TTF_CreateSurfaceTextEngine
//! - [x] TTF_DestroyGPUTextEngine
//! - [x] TTF_DestroyRendererTextEngine
//! - [x] TTF_DestroySurfaceTextEngine
//! - [x] TTF_GetGPUTextEngineWinding
//! - [x] TTF_SetGPUTextEngineWinding

use sdl3_ttf_sys::ttf::*;

use crate::{Result, error::Error, gpu::Device, renderer::Renderer, resource::Ref, resource_new};

#[repr(i32)]
#[derive(Clone, Copy)]
#[doc(alias = "TTF_GPUTextEngineWinding")]
pub enum Winding {
    Clockwise = TTF_GPUTextEngineWinding::CLOCKWISE.0,
    CounterClockwise = TTF_GPUTextEngineWinding::COUNTER_CLOCKWISE.0,
}

resource_new!(TTF_TextEngine, GpuEngine, TTF_DestroyGPUTextEngine);

impl GpuEngine {
    #[doc(alias = "TTF_CreateGPUTextEngine")]
    pub fn new(dev: Ref<Device>) -> Result<Self> {
        Self::from_ptr(unsafe { TTF_CreateGPUTextEngine(dev.as_ptr()) })
    }
}

impl GpuEngineHandle {
    #[doc(alias = "TTF_GetGPUTextEngineWinding")]
    pub fn winding(&self) -> Result<Winding> {
        let wind = unsafe { TTF_GetGPUTextEngineWinding(self.as_ptr()) };
        if wind == TTF_GPUTextEngineWinding::INVALID {
            Err(Error::current())
        } else {
            type Src = TTF_GPUTextEngineWinding;
            type Dst = Winding;
            Ok(unsafe { std::mem::transmute::<Src, Dst>(wind) })
        }
    }

    #[doc(alias = "TTF_SetGPUTextEngineWinding")]
    pub fn set_winding(&self, wind: Winding) {
        unsafe {
            TTF_SetGPUTextEngineWinding(self.as_ptr(), std::mem::transmute(wind));
        }
    }
}

resource_new!(TTF_TextEngine, SurfaceEngine, TTF_DestroySurfaceTextEngine);

impl SurfaceEngine {
    #[doc(alias = "TTF_CreateSurfaceTextEngine")]
    pub fn new() -> Result<Self> {
        Self::from_ptr(unsafe { TTF_CreateSurfaceTextEngine() })
    }
}

resource_new!(
    TTF_TextEngine,
    RendererEngine,
    TTF_DestroyRendererTextEngine
);

impl RendererEngine {
    #[doc(alias = "TTF_CreateRendererTextEngine")]
    pub fn new(rnd: Ref<Renderer>) -> Result<Self> {
        Self::from_ptr(unsafe { TTF_CreateRendererTextEngine(rnd.as_ptr()) })
    }
}
