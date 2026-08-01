#[cfg(not(target_os = "macos"))]
compile_error!("Sorry, this example currently only works on macOS.");

use std::mem::ManuallyDrop;

use halcyon::{
    Context, Result,
    color::RgbaF32,
    event::{Event, EventIter},
    gpu::*,
    rect::Point,
    resource::Resource,
    subsystem::Video,
    window::Window,
};

// Metal shaders, compiled at runtime by SDL.
const VS_MSL: &[u8] = br#"
#include <metal_stdlib>
using namespace metal;

struct V2F {
    float4 position [[position]];
    float4 color;
};

vertex V2F vs_main(uint vid [[vertex_id]]) {
    const float2 pos[3] = {
        float2(-0.5, -0.5),
        float2( 0.5, -0.5),
        float2( 0.0,  0.5),
    };
    const float3 col[3] = {
        float3(1.0, 0.0, 0.0),
        float3(0.0, 1.0, 0.0),
        float3(0.0, 0.0, 1.0),
    };
    V2F out;
    out.position = float4(pos[vid], 0.0, 1.0);
    out.color = float4(col[vid], 1.0);
    return out;
}
"#;

const FS_MSL: &[u8] = br#"
#include <metal_stdlib>
using namespace metal;

struct V2F {
    float4 position [[position]];
    float4 color;
};

fragment float4 fs_main(V2F in [[stage_in]]) {
    return in.color;
}
"#;

fn run() -> Result {
    let ctx = Context::new();
    let _video = ManuallyDrop::new(Video::new(&ctx)?);

    let device = Device::new(ShaderFormats::Msl, DeviceDebug::Yes)?;
    println!("GPU driver: {}", device.driver()?);

    let wnd = Window::new(c"Halcyon GPU", Point::new(800, 600), Default::default())?;
    device.claim_window(wnd.as_ref())?;

    // The pipeline's color target format must match the swapchain's.
    let swapchain_format = device.swapchain_texture_format(wnd.as_ref());

    let vs = Shader::new(
        device.as_ref(),
        &ShaderCreateInfo::new(
            VS_MSL,
            c"vs_main",
            ShaderFormat::Msl,
            ShaderStage::Vertex,
            0,
            (0, 0, 0),
        ),
    )?;

    let fs = Shader::new(
        device.as_ref(),
        &ShaderCreateInfo::new(
            FS_MSL,
            c"fs_main",
            ShaderFormat::Msl,
            ShaderStage::Fragment,
            0,
            (0, 0, 0),
        ),
    )?;

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

    let target_info = GraphicsPipelineTargetInfo::new(
        &[ColorTargetDescription::new(swapchain_format, blend)],
        TextureFormat::D24Unorm,
        HasDepthStencilTarget::No,
    );

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

    'frames: loop {
        for event in EventIter::new() {
            if let Event::Quit = event {
                break 'frames;
            }
        }

        let cmdbuf = CommandBuffer::new(device.as_ref())?;
        let (mut width, mut height) = (0u32, 0u32);
        if let Some(tex) = cmdbuf
            .wait_for_swapchain_texture(wnd.as_ref(), (Some(&mut width), Some(&mut height)))?
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
            // `render_pass` is dropped here, ending the render pass.
        }

        // Submitting the command buffer also presents the swapchain texture.
        cmdbuf.submit()?;
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
        eprintln!("An unexpected error occurred: {e}");
    }
}
