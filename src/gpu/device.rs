//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryGPU)):
//! - [x] SDL_ClaimWindowForGPUDevice
//! - [x] SDL_CreateGPUDevice
//! - [x] SDL_CreateGPUDeviceWithProperties
//! - [x] SDL_DestroyGPUDevice
//! - [x] SDL_GetGPUDeviceDriver
//! - [x] SDL_GetGPUDeviceProperties
//! - [x] SDL_GetGPUSwapchainTextureFormat
//! - [x] SDL_GPUTextureSupportsFormat
//! - [x] SDL_GPUTextureSupportsSampleCount
//! - [x] SDL_ReleaseWindowFromGPUDevice
//! - [x] SDL_SetGPUAllowedFramesInFlight
//! - [x] SDL_SetGPUSwapchainParameters
//! - [x] SDL_WaitForGPUFences
//! - [x] SDL_WaitForGPUIdle
//! - [x] SDL_WaitForGPUSwapchain
//! - [x] SDL_WindowSupportsGPUPresentMode
//! - [x] SDL_WindowSupportsGPUSwapchainComposition
//! - [x] SDL_GetGPUShaderFormats

use std::{
    ffi::{CStr, c_char, c_void},
    ops::Deref,
};

use sdl3_sys::gpu::*;

use crate::{
    Result,
    error::Error,
    gpu::{EnableDebug, WaitAll},
    properties::{Properties, PropertiesHandle},
    resource::Ref,
    resource_new,
    util::{c_ptr_to_str, to_result},
    window::Window,
};

use super::{
    ShaderFormats,
    fence::Fence,
    texture::{SampleCount, TextureFormat, TextureType, TextureUsageFlags},
};

#[repr(i32)]
#[doc(alias = "SDL_GPUPresentMode")]
pub enum PresentMode {
    Vsync = SDL_GPUPresentMode::VSYNC.0,
    Immediate = SDL_GPUPresentMode::IMMEDIATE.0,
    Mailbox = SDL_GPUPresentMode::MAILBOX.0,
}

#[repr(i32)]
#[doc(alias = "SDL_GPUSwapchainComposition")]
pub enum SwapchainComposition {
    Sdr = SDL_GPUSwapchainComposition::SDR.0,
    SdrLinear = SDL_GPUSwapchainComposition::SDR_LINEAR.0,
    HdrExtendedLinear = SDL_GPUSwapchainComposition::HDR_EXTENDED_LINEAR.0,
    Hdr10St2084 = SDL_GPUSwapchainComposition::HDR10_ST2084.0,
}

/// Builder for [`Device`], using
/// [`SDL_CreateGPUDeviceWithProperties`](https://wiki.libsdl.org/SDL3/SDL_CreateGPUDeviceWithProperties).
pub struct DeviceBuilder {
    inner: Properties,
}

impl DeviceBuilder {
    /// Enable debug mode properties and validations. Defaults to `true`.
    pub fn debug_mode(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_GPU_DEVICE_CREATE_DEBUGMODE_BOOLEAN, value)
    }

    /// Prefer energy efficiency over maximum GPU performance. Defaults to `false`.
    pub fn prefer_low_power(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_GPU_DEVICE_CREATE_PREFERLOWPOWER_BOOLEAN, value)
    }

    /// Automatically log useful debug information on device creation. Defaults to `true`.
    pub fn verbose(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_GPU_DEVICE_CREATE_VERBOSE_BOOLEAN, value)
    }

    /// The name of the GPU driver to use, if a specific one is desired.
    pub fn name(&mut self, value: &CStr) -> &mut Self {
        self.set_string(SDL_PROP_GPU_DEVICE_CREATE_NAME_STRING, value)
    }

    /// Enable the Vulkan `shaderClipDistance` feature. Defaults to `true`.
    /// Disabling optional features allows the application to run on some
    /// older Android devices.
    pub fn clip_distance(&mut self, value: bool) -> &mut Self {
        self.set_bool(
            SDL_PROP_GPU_DEVICE_CREATE_FEATURE_CLIP_DISTANCE_BOOLEAN,
            value,
        )
    }

    /// Enable the Vulkan `depthClamp` feature. Defaults to `true`.
    /// Disabling optional features allows the application to run on some
    /// older Android devices.
    pub fn depth_clamping(&mut self, value: bool) -> &mut Self {
        self.set_bool(
            SDL_PROP_GPU_DEVICE_CREATE_FEATURE_DEPTH_CLAMPING_BOOLEAN,
            value,
        )
    }

    /// Enable the Vulkan `drawIndirectFirstInstance` feature. Defaults to `true`.
    /// Disabling optional features allows the application to run on some
    /// older Android devices.
    pub fn indirect_draw_first_instance(&mut self, value: bool) -> &mut Self {
        self.set_bool(
            SDL_PROP_GPU_DEVICE_CREATE_FEATURE_INDIRECT_DRAW_FIRST_INSTANCE_BOOLEAN,
            value,
        )
    }

    /// Enable the Vulkan `samplerAnisotropy` feature. Defaults to `true`.
    /// Disabling optional features allows the application to run on some
    /// older Android devices.
    pub fn anisotropy(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_GPU_DEVICE_CREATE_FEATURE_ANISOTROPY_BOOLEAN, value)
    }

    /// The app is able to provide shaders for an NDA platform.
    pub fn shaders_private(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_GPU_DEVICE_CREATE_SHADERS_PRIVATE_BOOLEAN, value)
    }

    /// The app is able to provide SPIR-V shaders, if applicable.
    pub fn shaders_spirv(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_GPU_DEVICE_CREATE_SHADERS_SPIRV_BOOLEAN, value)
    }

    /// The app is able to provide DXBC shaders, if applicable.
    pub fn shaders_dxbc(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_GPU_DEVICE_CREATE_SHADERS_DXBC_BOOLEAN, value)
    }

    /// The app is able to provide DXIL shaders, if applicable.
    pub fn shaders_dxil(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_GPU_DEVICE_CREATE_SHADERS_DXIL_BOOLEAN, value)
    }

    /// The app is able to provide MSL shaders, if applicable.
    pub fn shaders_msl(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_GPU_DEVICE_CREATE_SHADERS_MSL_BOOLEAN, value)
    }

    /// The app is able to provide Metal shader libraries, if applicable.
    pub fn shaders_metallib(&mut self, value: bool) -> &mut Self {
        self.set_bool(SDL_PROP_GPU_DEVICE_CREATE_SHADERS_METALLIB_BOOLEAN, value)
    }

    /// Allow D3D12 Tier 1 resource binding support, if the application uses
    /// 8 or fewer storage resources across all shader stages.
    pub fn d3d12_allow_fewer_resource_slots(&mut self, value: bool) -> &mut Self {
        self.set_bool(
            SDL_PROP_GPU_DEVICE_CREATE_D3D12_ALLOW_FEWER_RESOURCE_SLOTS_BOOLEAN,
            value,
        )
    }

    /// The prefix to use for all D3D12 vertex semantics. Defaults to `"TEXCOORD"`.
    pub fn d3d12_semantic_name(&mut self, value: &CStr) -> &mut Self {
        self.set_string(SDL_PROP_GPU_DEVICE_CREATE_D3D12_SEMANTIC_NAME_STRING, value)
    }

    /// The D3D12 Agility SDK version, which must match the version in the
    /// DLL vendored alongside the app.
    pub fn d3d12_agility_sdk_version(&mut self, value: i64) -> &mut Self {
        self.set_number(
            SDL_PROP_GPU_DEVICE_CREATE_D3D12_AGILITY_SDK_VERSION_NUMBER,
            value,
        )
    }

    /// The path to the D3D12 Agility SDK DLL, relative to the executable
    /// path. Do not put the DLL in the same directory as the exe.
    pub fn d3d12_agility_sdk_path(&mut self, value: &CStr) -> &mut Self {
        self.set_string(
            SDL_PROP_GPU_DEVICE_CREATE_D3D12_AGILITY_SDK_PATH_STRING,
            value,
        )
    }

    /// Require hardware acceleration for the Vulkan device, excluding
    /// software renderers such as Lavapipe. Defaults to `false`.
    pub fn vulkan_require_hardware_acceleration(&mut self, value: bool) -> &mut Self {
        self.set_bool(
            SDL_PROP_GPU_DEVICE_CREATE_VULKAN_REQUIRE_HARDWARE_ACCELERATION_BOOLEAN,
            value,
        )
    }

    /// Configure Vulkan-specific options during device creation. The
    /// referenced struct is read at build time and must outlive `build()`.
    pub fn vulkan_options(&mut self, value: &SDL_GPUVulkanOptions) -> &mut Self {
        let cstr = unsafe { CStr::from_ptr(SDL_PROP_GPU_DEVICE_CREATE_VULKAN_OPTIONS_POINTER) };
        _ = self
            .inner
            .set_pointer(cstr, value as *const SDL_GPUVulkanOptions as *mut c_void);
        self
    }

    /// Allow macOS support for `MTLGPUFamilyMac1` hardware, if the
    /// application does not write to sRGB textures.
    pub fn metal_allow_macfamily1(&mut self, value: bool) -> &mut Self {
        self.set_bool(
            SDL_PROP_GPU_DEVICE_CREATE_METAL_ALLOW_MACFAMILY1_BOOLEAN,
            value,
        )
    }

    /// Build the device.
    #[doc(alias = "SDL_CreateGPUDeviceWithProperties")]
    pub fn build(&self) -> Result<Device> {
        Device::from_ptr(unsafe { SDL_CreateGPUDeviceWithProperties(self.inner.id()) })
    }

    fn set_bool(&mut self, key: *const c_char, value: bool) -> &mut Self {
        _ = self.inner.set_bool(unsafe { CStr::from_ptr(key) }, value);
        self
    }

    fn set_number(&mut self, key: *const c_char, value: i64) -> &mut Self {
        _ = self.inner.set_number(unsafe { CStr::from_ptr(key) }, value);
        self
    }

    fn set_string(&mut self, key: *const c_char, value: &CStr) -> &mut Self {
        _ = self
            .inner
            .set_string(unsafe { CStr::from_ptr(key) }, value.as_ptr());
        self
    }
}

#[derive(Clone, Copy)]
pub struct DeviceProperties<'a> {
    inner: Ref<'a, Properties>,
}

impl<'a> DeviceProperties<'a> {
    fn new(inner: Ref<'a, Properties>) -> Self {
        Self { inner }
    }

    fn get(&self, key: *const i8) -> Option<&str> {
        let cstr = unsafe { CStr::from_ptr(key) };
        let s = self.inner.string(cstr, std::ptr::null());

        if s.is_null() {
            return None;
        }

        Some(unsafe { c_ptr_to_str(s.cast()) })
    }

    pub fn device_name(&self) -> Option<&str> {
        self.get(SDL_PROP_GPU_DEVICE_NAME_STRING)
    }

    pub fn driver_name(&self) -> Option<&str> {
        self.get(SDL_PROP_GPU_DEVICE_DRIVER_NAME_STRING)
    }

    pub fn driver_version(&self) -> Option<&str> {
        self.get(SDL_PROP_GPU_DEVICE_DRIVER_VERSION_STRING)
    }

    pub fn driver_info(&self) -> Option<&str> {
        self.get(SDL_PROP_GPU_DEVICE_DRIVER_INFO_STRING)
    }
}

impl Deref for DeviceProperties<'_> {
    type Target = PropertiesHandle;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

resource_new!(SDL_GPUDevice, Device, SDL_DestroyGPUDevice);
impl Device {
    #[doc(alias = "SDL_CreateGPUDevice")]
    pub fn new(formats: ShaderFormats, debug: EnableDebug) -> Result<Self> {
        let fmts = SDL_GPUShaderFormat::new(formats.bits());
        let handle = unsafe { SDL_CreateGPUDevice(fmts, debug.into(), std::ptr::null()) };
        Self::from_ptr(handle)
    }

    pub fn builder() -> Result<DeviceBuilder> {
        let inner = Properties::new()?;
        Ok(DeviceBuilder { inner })
    }
}

impl DeviceHandle {
    #[doc(alias = "SDL_ClaimWindowForGPUDevice")]
    pub fn claim_window(&self, window: Ref<Window>) -> Result {
        to_result(unsafe {
            SDL_ClaimWindowForGPUDevice(self.handle.as_ptr(), window.handle.as_ptr())
        })
    }

    #[doc(alias = "SDL_ReleaseWindowFromGPUDevice")]
    pub fn release_window(&self, window: Ref<Window>) {
        unsafe { SDL_ReleaseWindowFromGPUDevice(self.handle.as_ptr(), window.handle.as_ptr()) };
    }

    #[doc(alias = "SDL_WindowSupportsGPUPresentMode")]
    pub fn window_supports_gpu_present_mode(&self, window: Ref<Window>, pm: PresentMode) -> bool {
        unsafe {
            SDL_WindowSupportsGPUPresentMode(
                self.handle.as_ptr(),
                window.handle.as_ptr(),
                SDL_GPUPresentMode::new(pm as _),
            )
        }
    }

    #[doc(alias = "SDL_WindowSupportsGPUSwapchainComposition")]
    pub fn window_supports_gpu_swapchain_composition(
        &self,
        window: Ref<Window>,
        sc: SwapchainComposition,
    ) -> bool {
        unsafe {
            SDL_WindowSupportsGPUSwapchainComposition(
                self.handle.as_ptr(),
                window.handle.as_ptr(),
                SDL_GPUSwapchainComposition::new(sc as _),
            )
        }
    }

    #[doc(alias = "SDL_WaitForGPUIdle")]
    pub fn wait_idle(&self) -> Result {
        to_result(unsafe { SDL_WaitForGPUIdle(self.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_WaitForGPUSwapchain")]
    pub fn wait_swapchain(&self, window: Ref<Window>) -> Result {
        to_result(unsafe { SDL_WaitForGPUSwapchain(self.handle.as_ptr(), window.handle.as_ptr()) })
    }

    #[doc(alias = "SDL_WaitForGPUFences")]
    pub fn wait_fences(&self, wait_all: WaitAll, fences: &[Ref<Fence>]) -> Result {
        to_result(unsafe {
            SDL_WaitForGPUFences(
                self.handle.as_ptr(),
                wait_all.into(),
                fences.as_ptr().cast(),
                fences.len() as _,
            )
        })
    }

    #[doc(alias = "SDL_GetGPUDeviceDriver")]
    pub fn driver(&self) -> Result<&str> {
        let raw = unsafe { SDL_GetGPUDeviceDriver(self.handle.as_ptr()) };
        if raw.is_null() {
            Err(Error::current())
        } else {
            let cstr = unsafe { CStr::from_ptr(raw) };
            Ok(unsafe { str::from_utf8_unchecked(cstr.to_bytes()) })
        }
    }

    #[doc(alias = "SDL_GetGPUDeviceProperties")]
    pub fn properties(&'_ self) -> DeviceProperties<'_> {
        let id = unsafe { SDL_GetGPUDeviceProperties(self.handle.as_ptr()) };
        let handle =
            PropertiesHandle::from_id(id).expect("A valid GPU device should have properties");

        let r = unsafe { Ref::from_handle(handle) };
        DeviceProperties::new(r)
    }

    #[doc(alias = "SDL_GetGPUSwapchainTextureFormat")]
    pub fn swapchain_texture_format(&self, window: Ref<Window>) -> TextureFormat {
        let fmt = unsafe {
            SDL_GetGPUSwapchainTextureFormat(self.handle.as_ptr(), window.handle.as_ptr())
        };
        fmt.into()
    }

    #[doc(alias = "SDL_GPUTextureSupportsFormat")]
    pub fn texture_supports_format(
        &self,
        format: TextureFormat,
        kind: TextureType,
        usage: TextureUsageFlags,
    ) -> bool {
        unsafe {
            SDL_GPUTextureSupportsFormat(
                self.handle.as_ptr(),
                SDL_GPUTextureFormat::new(format as _),
                SDL_GPUTextureType::new(kind as _),
                SDL_GPUTextureUsageFlags::new(usage.bits()),
            )
        }
    }

    #[doc(alias = "SDL_GPUTextureSupportsSampleCount")]
    pub fn texture_supports_sample_count(
        &self,
        format: TextureFormat,
        sample_count: SampleCount,
    ) -> bool {
        unsafe {
            SDL_GPUTextureSupportsSampleCount(
                self.handle.as_ptr(),
                SDL_GPUTextureFormat::new(format as _),
                SDL_GPUSampleCount::new(sample_count as _),
            )
        }
    }

    #[doc(alias = "SDL_SetGPUSwapchainParameters")]
    pub fn set_swapchain_parameters(
        &self,
        window: Ref<Window>,
        composition: SwapchainComposition,
        present_mode: PresentMode,
    ) -> Result {
        to_result(unsafe {
            SDL_SetGPUSwapchainParameters(
                self.handle.as_ptr(),
                window.handle.as_ptr(),
                SDL_GPUSwapchainComposition::new(composition as _),
                SDL_GPUPresentMode::new(present_mode as _),
            )
        })
    }

    #[doc(alias = "SDL_GetGPUShaderFormats")]
    pub fn shader_formats(&self) -> ShaderFormats {
        let fmts = unsafe { SDL_GetGPUShaderFormats(self.handle.as_ptr()) };
        ShaderFormats::from(fmts.0)
    }

    #[doc(alias = "SDL_SetGPUAllowedFramesInFlight")]
    pub fn set_allowed_frames_in_flight(&self, n: u32) -> Result {
        to_result(unsafe { SDL_SetGPUAllowedFramesInFlight(self.handle.as_ptr(), n) })
    }
}
