// Compile with the following commands (in the `examples/shaders/` directory):
// xcrun metal -o /tmp/teapot.ir -c src/teapot.metal
// xcrun metal -o teapot.metallib /tmp/teapot.ir
//
// Per the SDL wiki (SDL_CreateGPUShader), on the Metal backend uniform
// buffers occupy [[buffer(0)]] onwards, while vertex buffers are bound at
// [[buffer(14)]]+; vertex input therefore uses [[stage_in]], which maps
// automatically to the pipeline's vertex attributes.

#include <metal_stdlib>
using namespace metal;

struct VertexIn {
    float3 position [[attribute(0)]];
    float3 normal   [[attribute(1)]];
};

struct V2F {
    float4 position [[position]];
    float3 normal;
};

struct Uniforms {
    float4x4 mvp;
    float4x4 model;
};

vertex V2F vs_main(VertexIn in [[stage_in]],
                   constant Uniforms& u [[buffer(0)]]) {
    V2F out;
    out.position = u.mvp * float4(in.position, 1.0);
    out.normal = (u.model * float4(in.normal, 0.0)).xyz;
    return out;
}

fragment float4 fs_main(V2F in [[stage_in]]) {
    float3 n = normalize(in.normal);
    float3 light_dir = normalize(float3(0.4, 0.8, 0.3));
    float diffuse = max(dot(n, light_dir), 0.0);
    float3 base = float3(0.75, 0.55, 0.35); // untextured clay teapot
    float3 color = base * (0.15 + 0.85 * diffuse);
    return float4(color, 1.0);
}
