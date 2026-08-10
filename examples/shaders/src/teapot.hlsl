// HLSL equivalent of the Metal shaders in examples/teapot.rs, for the D3D12
// backend.
//
// Compile via `scripts\windows\compile-hlsl-shaders.ps1`.
//
// Per the SDL wiki (SDL_CreateGPUShader), on D3D12:
// - vertex input semantics are TEXCOORDn, where n is the attribute location
// - vertex uniform buffers are bound at (b[n], space1)

struct Uniforms {
    float4x4 mvp;
    float4x4 model;
};

ConstantBuffer<Uniforms> u : register(b0, space1);

struct V2F {
    float4 position : SV_Position;
    float3 normal   : TEXCOORD0;
};

V2F vs_main(float3 position : TEXCOORD0,
            float3 normal   : TEXCOORD1) {
    V2F o;
    o.position = mul(u.mvp, float4(position, 1.0));
    o.normal = mul((float3x3)u.model, normal);
    return o;
}

float4 fs_main(V2F input) : SV_Target {
    float3 n = normalize(input.normal);
    float3 light_dir = normalize(float3(0.4, 0.8, 0.3));
    float diffuse = max(dot(n, light_dir), 0.0);
    float3 base = float3(0.75, 0.55, 0.35); // untextured clay teapot
    float3 color = base * (0.15 + 0.85 * diffuse);
    return float4(color, 1.0);
}
