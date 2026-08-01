// HLSL equivalent of the Metal shaders in examples/gpu.rs, for the D3D12
// backend.
//
// Unlike MSL, SDL's D3D12 backend cannot compile HLSL at runtime: it expects
// precompiled bytecode. For DXIL (ShaderFormat::Dxil), compile each entry
// point with dxc:
//
//   dxc -T vs_6_0 -E vs_main triangle.hlsl -Fo triangle_vs.dxil
//   dxc -T ps_6_0 -E fs_main triangle.hlsl -Fo triangle_fs.dxil
//
// For DXBC (ShaderFormat::Dxbc, SM5.1), use fxc:
//
//   fxc /T vs_5_1 /E vs_main triangle.hlsl /Fo triangle_vs.dxbc
//   fxc /T ps_5_1 /E fs_main triangle.hlsl /Fo triangle_fs.dxbc
//
// (dxc also accepts the -T vs_5_1 / -T ps_5_1 profiles, but recent versions
// promote them to 6.0 internally, so the output is DXIL rather than DXBC.)
// Pass the bytecode to ShaderCreateInfo with the matching entry point name.

struct V2F {
    float4 position : SV_Position;
    float4 color    : COLOR;
};

V2F vs_main(uint vid : SV_VertexID) {
    float2 pos[3] = {
        float2(-0.5, -0.5),
        float2( 0.5, -0.5),
        float2( 0.0,  0.5),
    };
    float3 col[3] = {
        float3(1.0, 0.0, 0.0),
        float3(0.0, 1.0, 0.0),
        float3(0.0, 0.0, 1.0),
    };
    V2F o;
    o.position = float4(pos[vid], 0.0, 1.0);
    o.color = float4(col[vid], 1.0);
    return o;
}

float4 fs_main(V2F input) : SV_Target {
    return input.color;
}
