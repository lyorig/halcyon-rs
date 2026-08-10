//! The graphics programming world's "hello world" program, i.e. a colored triangle.

#![windows_subsystem = "windows"]

use std::mem::ManuallyDrop;

use halcyon::{
    Context, Result,
    color::RgbaF32,
    event::{Event, EventIter},
    gpu::*,
    properties::Properties,
    rect::Point,
    resource::Resource,
    subsystem::Video,
    window::Window,
};

cfg_select! {
    target_os = "macos" => {
        const VS_CODE: &[u8] = include_bytes!("shaders/triangle.metallib");
        const FS_CODE: &[u8] = VS_CODE;
        const SHADER_FMT: ShaderFormat = ShaderFormat::Metallib;
    },
    target_os = "windows" => {
        const VS_CODE: &[u8] = include_bytes!("shaders/triangle_vs.dxil");
        const FS_CODE: &[u8] = include_bytes!("shaders/triangle_fs.dxil");
        const SHADER_FMT: ShaderFormat = ShaderFormat::Dxil;
    }
}

fn print_properties(props: DeviceProperties) {
    fn f(o: Option<&str>) -> &str {
        o.unwrap_or("N/A")
    }

    halcyon::log!("Device name: {}", f(props.device_name()));
    halcyon::log!("Driver name: {}", f(props.driver_name()));
    halcyon::log!("Driver info: {}", f(props.driver_info()));
    halcyon::log!("Driver version: {}", f(props.driver_version()));
}

fn run() -> Result {
    let ctx = Context::new();
    let _video = ManuallyDrop::new(Video::new(&ctx)?);

    // SDL provides an existing property set, which we can conveniently abuse.
    let props = Properties::global()?;

    let device = Device::builder(props)
        .debug_mode(false)
        .prefer_low_power(true)
        .shaders_metallib(true)
        .shaders_dxil(true)
        .build_cleanup()?;

    let wnd = Window::builder(props)
        .title(c"Halcyon GPU")
        .size(Point::new(720, 480))
        .build_cleanup()?;

    print_properties(device.properties());

    device.claim_window(wnd.as_ref())?;

    let sci_vs = ShaderCreateInfo::new(
        VS_CODE,
        c"vs_main",
        SHADER_FMT,
        ShaderStage::Vertex,
        0,
        (0, 0, 0),
    );

    let sci_fs = ShaderCreateInfo::new(
        FS_CODE,
        c"fs_main",
        SHADER_FMT,
        ShaderStage::Fragment,
        0,
        (0, 0, 0),
    );

    let vs = Shader::new(device.as_ref(), &sci_vs)?;
    let fs = Shader::new(device.as_ref(), &sci_fs)?;

    let blend = ColorTargetBlendState::new(
        (BlendFactor::One, BlendFactor::Zero),
        BlendOp::Add,
        (BlendFactor::One, BlendFactor::Zero),
        BlendOp::Add,
        ColorComponentFlags::R
            | ColorComponentFlags::G
            | ColorComponentFlags::B
            | ColorComponentFlags::A,
        EnableBlend::No,
        EnableColorWriteMask::No,
    );

    // The pipeline's color target format must match the swapchain's.
    let swapchain_format = device.swapchain_texture_format(wnd.as_ref());
    let ctd = [ColorTargetDescription::new(swapchain_format, blend)];

    let target_info =
        GraphicsPipelineTargetInfo::new(&ctd, TextureFormat::D24Unorm, HasDepthStencilTarget::No);

    let stencil = StencilOpState::new(
        StencilOp::Keep,
        StencilOp::Keep,
        StencilOp::Keep,
        CompareOp::Always,
    );

    // No vertex buffer: the triangle's corners and colors are generated from
    // the vertex index inside the shader.
    let pipeline_info = GraphicsPipelineCreateInfo::new(
        vs.as_ref(),
        fs.as_ref(),
        VertexInputState::new(&[], &[]),
        PrimitiveType::TriangleList,
        RasterizerState::new(
            FillMode::Fill,
            CullMode::None,
            FrontFace::CounterClockwise,
            0.0,
            0.0,
            0.0,
            EnableDepthBias::No,
            EnableDepthClip::Yes,
        ),
        MultisampleState::new(SampleCount::One, EnableAlphaToCoverage::Yes),
        DepthStencilState::new(
            CompareOp::Always,
            stencil,
            stencil,
            0xFF,
            0xFF,
            EnableDepthTest::No,
            EnableDepthWrite::No,
            EnableStencilTest::No,
        ),
        target_info,
    );

    let pipeline = GraphicsPipeline::new(device.as_ref(), &pipeline_info)?;

    let cmdbuf = CommandBuffer::new(device.as_ref())?;
    let (mut width, mut height) = (0u32, 0u32);
    if let Some(tex) =
        cmdbuf.wait_for_swapchain_texture(wnd.as_ref(), (Some(&mut width), Some(&mut height)))?
    {
        halcyon::log!("Swapchain texture dimensions = {width}x{height}");
        let color_target = ColorTargetInfo::new(
            tex,
            0,
            0,
            RgbaF32::new(0.0, 0.0, 0.0, 1.0),
            LoadOp::Clear,
            StoreOp::Store,
            None,
            (0, 0),
            Cycle::No,
            CycleResolveTexture::No,
        );

        let render_pass = RenderPass::new(cmdbuf.as_ref(), &[color_target], None)?;

        pipeline.bind(render_pass.as_ref());
        render_pass.set_viewport(&Viewport::new(
            Point::new(0.0, 0.0),
            Point::new(width as f32, height as f32),
            (0.0, 1.0),
        ));
        render_pass.draw_primitives(3, 1, 0, 0);
    }

    // Submitting the command buffer also presents the swapchain texture.
    cmdbuf.submit()?;

    'frames: loop {
        for event in EventIter::new() {
            if let Event::Quit = event {
                break 'frames;
            }
        }

        // Poor man's VSync.
        use std::{thread::sleep, time::Duration};
        sleep(Duration::from_millis(10));
    }

    device.wait_idle()?;

    device.release_window(wnd.as_ref());
    pipeline.drop(device.as_ref());
    fs.drop(device.as_ref());
    vs.drop(device.as_ref());

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        halcyon::log_error!("An unexpected error occurred: {e}");
    }
}
