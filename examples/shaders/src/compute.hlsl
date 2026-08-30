// Compile via `scripts/windows/compile-hlsl-shaders.ps1`.

RWStructuredBuffer<uint> data : register(u0);

[numthreads(1, 1, 1)]
void cs_main(uint3 tid : SV_DispatchThreadID) {
    if (tid.x == 0) {
        data[0] += 1;
    }
}
