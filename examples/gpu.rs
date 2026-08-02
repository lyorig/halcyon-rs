// #![windows_subsystem = "windows"]

use std::{ffi::CStr, mem::ManuallyDrop};

use halcyon::{
    Context, Result,
    color::RgbaF32,
    event::{Event, EventIter},
    gpu::*,
    properties::Properties,
    rect::Point,
    resource::{Ref, Resource},
    subsystem::Video,
    window::Window,
};

cfg_select! {
    target_os = "macos" => {
        const VS_MSL: &[u8] = include_bytes!("shaders/vs.msl");
        const FS_MSL: &[u8] = include_bytes!("shaders/fs.msl");
        const SHADER_FMT: ShaderFormat = ShaderFormat::Msl;
    },
    target_os = "windows" => {
        const VS_MSL: &[u8] = include_bytes!("shaders/triangle_vs.dxil");
        const FS_MSL: &[u8] = include_bytes!("shaders/triangle_fs.dxil");
        const SHADER_FMT: ShaderFormat = ShaderFormat::Dxil;
    }
}

fn prop_enum(r: Ref<'_, Properties>, n: &CStr) {
    use sdl3_sys::log::SDL_Log;
    let value = r.string(n, c"");
    unsafe {
        SDL_Log(
            c"Property \"%s\" = \"%s\"".as_ptr(),
            n.as_ptr(),
            value.as_ptr(),
        )
    };
}

fn run() -> Result {
    let ctx = Context::new();
    let _video = ManuallyDrop::new(Video::new(&ctx)?);

    let device = Device::new(SHADER_FMT.as_mask(), EnableDebug::No)?;
    _ = device.properties().enumerate(prop_enum);

    let wnd = Window::new(c"Halcyon GPU", Point::new(800, 600), Default::default())?;
    device.claim_window(wnd.as_ref())?;

    _ = wnd.properties().enumerate(prop_enum);

    let sci_vs = ShaderCreateInfo::new(
        VS_MSL,
        c"vs_main",
        SHADER_FMT,
        ShaderStage::Vertex,
        0,
        (0, 0, 0),
    );

    let sci_fs = ShaderCreateInfo::new(
        FS_MSL,
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
        MultisampleState::new(
            SampleCount::One,
            0,
            EnableMask::No,
            EnableAlphaToCoverage::No,
        ),
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
        // println!() isn't enough, since #![windows_subsystem = "windows"]
        // prevents your usual methods of console output from working.
        // SDL's logging API seemingly works, though.
        use sdl3_sys::log::SDL_Log;

        let err = e.into_cstring();
        unsafe { SDL_Log(c"An unexpected error occurred: %s".as_ptr(), err.as_ptr()) };
    }
}
