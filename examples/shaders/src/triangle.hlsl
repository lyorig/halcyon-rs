// HLSL equivalent of the Metal shaders in examples/gpu.rs, for the D3D12
// backend.
//
// Compile via `scripts\powershell\compile-hlsl-shaders.ps1`.

struct V2F {
    float4 position : SV_Position;
    float4 color    : COLOR;
};

V2F vs_main(uint vid : SV_VertexID) {
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

    const V2F ret = {
        float4(pos[vid], 0.0, 1.0),
        float4(col[vid], 1.0)
    };

    return ret;
}

float4 fs_main(V2F input) : SV_Target {
    return input.color;
}
