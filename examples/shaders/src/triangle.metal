// Compile with the following commands (in the `examples/shaders/` directory):
// xcrun metal -o /tmp/triangle.ir -c src/triangle.metal
// xcrun metal -o triangle.metallib /tmp/triangle.ir

#include <metal_stdlib>
using namespace metal;

struct V2F {
    float4 position [[position]];
    float4 color;
};

fragment float4 fs_main(V2F in [[stage_in]]) {
    return in.color;
}

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
