use std::ffi::{CStr, c_char};

use sdl3_sys::gpu::*;

use crate::{Result, gpu::Device, properties::Properties, resource::Ref};

/// Builder for [`Device`], using
/// [`SDL_CreateGPUDeviceWithProperties`](https://wiki.libsdl.org/SDL3/SDL_CreateGPUDeviceWithProperties).
pub struct DeviceBuilder<'a> {
    inner: Ref<'a, Properties>,
}

impl DeviceBuilder<'_> {
    pub(super) fn new(inner: Ref<Properties>) -> DeviceBuilder {
        DeviceBuilder { inner }
    }

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
        _ = self.inner.set_pointer(cstr, std::ptr::from_ref(value) as _);
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
