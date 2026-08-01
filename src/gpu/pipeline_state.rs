//! Types describing the state of a graphics pipeline.
//!
//! Everything in this module is a building block of
//! [`GraphicsPipelineCreateInfo`](crate::gpu::graphics_pipeline::GraphicsPipelineCreateInfo).

use bitmask_enum::bitmask;
use sdl3_sys::gpu::*;

use crate::gpu::enums::*;

use super::{
    sampler::CompareOp,
    texture::{SampleCount, TextureFormat},
};

#[repr(i32)]
#[doc(alias = "SDL_GPUVertexElementFormat")]
pub enum VertexElementFormat {
    Invalid = SDL_GPUVertexElementFormat::INVALID.0,
    Int = SDL_GPUVertexElementFormat::INT.0,
    Int2 = SDL_GPUVertexElementFormat::INT2.0,
    Int3 = SDL_GPUVertexElementFormat::INT3.0,
    Int4 = SDL_GPUVertexElementFormat::INT4.0,
    Uint = SDL_GPUVertexElementFormat::UINT.0,
    Uint2 = SDL_GPUVertexElementFormat::UINT2.0,
    Uint3 = SDL_GPUVertexElementFormat::UINT3.0,
    Uint4 = SDL_GPUVertexElementFormat::UINT4.0,
    Float = SDL_GPUVertexElementFormat::FLOAT.0,
    Float2 = SDL_GPUVertexElementFormat::FLOAT2.0,
    Float3 = SDL_GPUVertexElementFormat::FLOAT3.0,
    Float4 = SDL_GPUVertexElementFormat::FLOAT4.0,
    Byte2 = SDL_GPUVertexElementFormat::BYTE2.0,
    Byte4 = SDL_GPUVertexElementFormat::BYTE4.0,
    Ubyte2 = SDL_GPUVertexElementFormat::UBYTE2.0,
    Ubyte4 = SDL_GPUVertexElementFormat::UBYTE4.0,
    Byte2Norm = SDL_GPUVertexElementFormat::BYTE2_NORM.0,
    Byte4Norm = SDL_GPUVertexElementFormat::BYTE4_NORM.0,
    Ubyte2Norm = SDL_GPUVertexElementFormat::UBYTE2_NORM.0,
    Ubyte4Norm = SDL_GPUVertexElementFormat::UBYTE4_NORM.0,
    Short2 = SDL_GPUVertexElementFormat::SHORT2.0,
    Short4 = SDL_GPUVertexElementFormat::SHORT4.0,
    Ushort2 = SDL_GPUVertexElementFormat::USHORT2.0,
    Ushort4 = SDL_GPUVertexElementFormat::USHORT4.0,
    Short2Norm = SDL_GPUVertexElementFormat::SHORT2_NORM.0,
    Short4Norm = SDL_GPUVertexElementFormat::SHORT4_NORM.0,
    Ushort2Norm = SDL_GPUVertexElementFormat::USHORT2_NORM.0,
    Ushort4Norm = SDL_GPUVertexElementFormat::USHORT4_NORM.0,
    Half2 = SDL_GPUVertexElementFormat::HALF2.0,
    Half4 = SDL_GPUVertexElementFormat::HALF4.0,
}

#[repr(i32)]
#[doc(alias = "SDL_GPUVertexInputRate")]
pub enum VertexInputRate {
    Vertex = SDL_GPUVertexInputRate::VERTEX.0,
    Instance = SDL_GPUVertexInputRate::INSTANCE.0,
}

#[repr(i32)]
#[doc(alias = "SDL_GPUFillMode")]
pub enum FillMode {
    Fill = SDL_GPUFillMode::FILL.0,
    Line = SDL_GPUFillMode::LINE.0,
}

#[repr(i32)]
#[doc(alias = "SDL_GPUCullMode")]
pub enum CullMode {
    None = SDL_GPUCullMode::NONE.0,
    Front = SDL_GPUCullMode::FRONT.0,
    Back = SDL_GPUCullMode::BACK.0,
}

#[repr(i32)]
#[doc(alias = "SDL_GPUFrontFace")]
pub enum FrontFace {
    CounterClockwise = SDL_GPUFrontFace::COUNTER_CLOCKWISE.0,
    Clockwise = SDL_GPUFrontFace::CLOCKWISE.0,
}

#[repr(i32)]
#[doc(alias = "SDL_GPUBlendFactor")]
pub enum BlendFactor {
    Invalid = SDL_GPUBlendFactor::INVALID.0,
    Zero = SDL_GPUBlendFactor::ZERO.0,
    One = SDL_GPUBlendFactor::ONE.0,
    SrcColor = SDL_GPUBlendFactor::SRC_COLOR.0,
    OneMinusSrcColor = SDL_GPUBlendFactor::ONE_MINUS_SRC_COLOR.0,
    DstColor = SDL_GPUBlendFactor::DST_COLOR.0,
    OneMinusDstColor = SDL_GPUBlendFactor::ONE_MINUS_DST_COLOR.0,
    SrcAlpha = SDL_GPUBlendFactor::SRC_ALPHA.0,
    OneMinusSrcAlpha = SDL_GPUBlendFactor::ONE_MINUS_SRC_ALPHA.0,
    DstAlpha = SDL_GPUBlendFactor::DST_ALPHA.0,
    OneMinusDstAlpha = SDL_GPUBlendFactor::ONE_MINUS_DST_ALPHA.0,
    ConstantColor = SDL_GPUBlendFactor::CONSTANT_COLOR.0,
    OneMinusConstantColor = SDL_GPUBlendFactor::ONE_MINUS_CONSTANT_COLOR.0,
    SrcAlphaSaturate = SDL_GPUBlendFactor::SRC_ALPHA_SATURATE.0,
}

#[repr(i32)]
#[doc(alias = "SDL_GPUBlendOp")]
pub enum BlendOp {
    Invalid = SDL_GPUBlendOp::INVALID.0,
    Add = SDL_GPUBlendOp::ADD.0,
    Subtract = SDL_GPUBlendOp::SUBTRACT.0,
    ReverseSubtract = SDL_GPUBlendOp::REVERSE_SUBTRACT.0,
    Min = SDL_GPUBlendOp::MIN.0,
    Max = SDL_GPUBlendOp::MAX.0,
}

#[repr(i32)]
#[doc(alias = "SDL_GPUStencilOp")]
pub enum StencilOp {
    Invalid = SDL_GPUStencilOp::INVALID.0,
    Keep = SDL_GPUStencilOp::KEEP.0,
    Zero = SDL_GPUStencilOp::ZERO.0,
    Replace = SDL_GPUStencilOp::REPLACE.0,
    IncrementAndClamp = SDL_GPUStencilOp::INCREMENT_AND_CLAMP.0,
    DecrementAndClamp = SDL_GPUStencilOp::DECREMENT_AND_CLAMP.0,
    Invert = SDL_GPUStencilOp::INVERT.0,
    IncrementAndWrap = SDL_GPUStencilOp::INCREMENT_AND_WRAP.0,
    DecrementAndWrap = SDL_GPUStencilOp::DECREMENT_AND_WRAP.0,
}

#[bitmask(u8)]
#[doc(alias = "SDL_GPUColorComponentFlags")]
pub enum ColorComponentFlags {
    R = SDL_GPUColorComponentFlags::R.0,
    G = SDL_GPUColorComponentFlags::G.0,
    B = SDL_GPUColorComponentFlags::B.0,
    A = SDL_GPUColorComponentFlags::A.0,
}

#[doc(alias = "SDL_GPUVertexBufferDescription")]
#[derive(Clone, Copy)]
pub struct VertexBufferDescription(SDL_GPUVertexBufferDescription);
impl VertexBufferDescription {
    pub fn new(
        slot: u32,
        pitch: u32,
        input_rate: VertexInputRate,
        instance_step_rate: u32,
    ) -> Self {
        Self(SDL_GPUVertexBufferDescription {
            slot,
            pitch,
            input_rate: SDL_GPUVertexInputRate::new(input_rate as _),
            instance_step_rate,
        })
    }
}

#[doc(alias = "SDL_GPUVertexAttribute")]
#[derive(Clone, Copy)]
pub struct VertexAttribute(SDL_GPUVertexAttribute);
impl VertexAttribute {
    pub fn new(location: u32, buffer_slot: u32, format: VertexElementFormat, offset: u32) -> Self {
        Self(SDL_GPUVertexAttribute {
            location,
            buffer_slot,
            format: SDL_GPUVertexElementFormat::new(format as _),
            offset,
        })
    }
}

#[doc(alias = "SDL_GPUVertexInputState")]
#[derive(Clone, Copy)]
pub struct VertexInputState(pub(crate) SDL_GPUVertexInputState);
impl VertexInputState {
    pub fn new(descriptions: &[VertexBufferDescription], attributes: &[VertexAttribute]) -> Self {
        Self(SDL_GPUVertexInputState {
            vertex_buffer_descriptions: descriptions.as_ptr().cast(),
            num_vertex_buffers: descriptions.len() as _,
            vertex_attributes: attributes.as_ptr().cast(),
            num_vertex_attributes: attributes.len() as _,
        })
    }
}

#[doc(alias = "SDL_GPURasterizerState")]
#[derive(Clone, Copy)]
pub struct RasterizerState(pub(crate) SDL_GPURasterizerState);
impl RasterizerState {
    // The padding fields are only needed for alignment; sdl3-sys marks them
    // deprecated and recommends `..Default::default()`, which is not implemented
    // for this struct.
    #[allow(deprecated, clippy::too_many_arguments)]
    pub fn new(
        fill_mode: FillMode,
        cull_mode: CullMode,
        front_face: FrontFace,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
        db: EnableDepthBias,
        dc: EnableDepthClip,
    ) -> Self {
        Self(SDL_GPURasterizerState {
            fill_mode: SDL_GPUFillMode::new(fill_mode as _),
            cull_mode: SDL_GPUCullMode::new(cull_mode as _),
            front_face: SDL_GPUFrontFace::new(front_face as _),
            depth_bias_constant_factor,
            depth_bias_clamp,
            depth_bias_slope_factor,
            enable_depth_bias: db.into(),
            enable_depth_clip: dc.into(),
            padding1: 0,
            padding2: 0,
        })
    }
}

#[doc(alias = "SDL_GPUMultisampleState")]
#[derive(Clone, Copy)]
pub struct MultisampleState(pub(crate) SDL_GPUMultisampleState);
impl MultisampleState {
    pub fn new(
        sample_count: SampleCount,
        sample_mask: u32,
        em: EnableMask,
        eatc: EnableAlphaToCoverage,
    ) -> Self {
        Self(SDL_GPUMultisampleState {
            sample_count: SDL_GPUSampleCount::new(sample_count as _),
            sample_mask,
            enable_mask: em.into(),
            enable_alpha_to_coverage: eatc.into(),
            ..Default::default()
        })
    }
}

#[doc(alias = "SDL_GPUStencilOpState")]
#[derive(Clone, Copy)]
pub struct StencilOpState(SDL_GPUStencilOpState);
impl StencilOpState {
    pub fn new(
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    ) -> Self {
        Self(SDL_GPUStencilOpState {
            fail_op: SDL_GPUStencilOp::new(fail_op as _),
            pass_op: SDL_GPUStencilOp::new(pass_op as _),
            depth_fail_op: SDL_GPUStencilOp::new(depth_fail_op as _),
            compare_op: SDL_GPUCompareOp::new(compare_op as _),
        })
    }
}

#[doc(alias = "SDL_GPUDepthStencilState")]
#[derive(Clone, Copy)]
pub struct DepthStencilState(pub(crate) SDL_GPUDepthStencilState);
impl DepthStencilState {
    // The padding fields are only needed for alignment; sdl3-sys marks them
    // deprecated and recommends `..Default::default()`, which is not implemented
    // for this struct.
    #[allow(deprecated, clippy::too_many_arguments)]
    pub fn new(
        compare_op: CompareOp,
        back_stencil_state: StencilOpState,
        front_stencil_state: StencilOpState,
        compare_mask: u8,
        write_mask: u8,
        edt: EnableDepthTest,
        edw: EnableDepthWrite,
        est: EnableStencilTest,
    ) -> Self {
        Self(SDL_GPUDepthStencilState {
            compare_op: SDL_GPUCompareOp::new(compare_op as _),
            back_stencil_state: back_stencil_state.0,
            front_stencil_state: front_stencil_state.0,
            compare_mask,
            write_mask,
            enable_depth_test: edt.into(),
            enable_depth_write: edw.into(),
            enable_stencil_test: est.into(),
            padding1: 0,
            padding2: 0,
            padding3: 0,
        })
    }
}

#[doc(alias = "SDL_GPUColorTargetBlendState")]
#[derive(Clone, Copy)]
pub struct ColorTargetBlendState(SDL_GPUColorTargetBlendState);
impl ColorTargetBlendState {
    // The padding fields are only needed for alignment; sdl3-sys marks them
    // deprecated and recommends `..Default::default()`, which is not implemented
    // for this struct.
    #[allow(deprecated, clippy::too_many_arguments)]
    pub fn new(
        (src_color_blendfactor, dst_color_blendfactor): (BlendFactor, BlendFactor),
        color_blend_op: BlendOp,
        (src_alpha_blendfactor, dst_alpha_blendfactor): (BlendFactor, BlendFactor),
        alpha_blend_op: BlendOp,
        color_write_mask: ColorComponentFlags,
        eb: EnableBlend,
        ecwm: EnableColorWriteMask,
    ) -> Self {
        Self(SDL_GPUColorTargetBlendState {
            src_color_blendfactor: SDL_GPUBlendFactor::new(src_color_blendfactor as _),
            dst_color_blendfactor: SDL_GPUBlendFactor::new(dst_color_blendfactor as _),
            color_blend_op: SDL_GPUBlendOp::new(color_blend_op as _),
            src_alpha_blendfactor: SDL_GPUBlendFactor::new(src_alpha_blendfactor as _),
            dst_alpha_blendfactor: SDL_GPUBlendFactor::new(dst_alpha_blendfactor as _),
            alpha_blend_op: SDL_GPUBlendOp::new(alpha_blend_op as _),
            color_write_mask: SDL_GPUColorComponentFlags::new(color_write_mask.bits()),
            enable_blend: eb.into(),
            enable_color_write_mask: ecwm.into(),
            padding1: 0,
            padding2: 0,
        })
    }
}

#[doc(alias = "SDL_GPUColorTargetDescription")]
#[derive(Clone, Copy)]
pub struct ColorTargetDescription(SDL_GPUColorTargetDescription);
impl ColorTargetDescription {
    pub fn new(format: TextureFormat, blend_state: ColorTargetBlendState) -> Self {
        Self(SDL_GPUColorTargetDescription {
            format: SDL_GPUTextureFormat::new(format as _),
            blend_state: blend_state.0,
        })
    }
}

#[doc(alias = "SDL_GPUGraphicsPipelineTargetInfo")]
#[derive(Clone, Copy)]
pub struct GraphicsPipelineTargetInfo(pub(crate) SDL_GPUGraphicsPipelineTargetInfo);
impl GraphicsPipelineTargetInfo {
    // FIXME: Passing `descriptions` as a temporary causes a UAF.
    pub fn new(
        descriptions: &[ColorTargetDescription],
        depth_stencil_format: TextureFormat,
        hdst: HasDepthStencilTarget,
    ) -> Self {
        Self(SDL_GPUGraphicsPipelineTargetInfo {
            color_target_descriptions: descriptions.as_ptr().cast(),
            num_color_targets: descriptions.len() as _,
            depth_stencil_format: SDL_GPUTextureFormat::new(depth_stencil_format as _),
            has_depth_stencil_target: hdst.into(),
            ..Default::default()
        })
    }
}
