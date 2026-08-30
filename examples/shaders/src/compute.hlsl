// Compile via `scripts/windows/compile-hlsl-shaders.ps1`.

RWByteAddressBuffer data : register(u0, space1);

[numthreads(1, 1, 1)]
void cs_main(uint3 tid : SV_DispatchThreadID) {
    if (tid.x == 0) {
        data.Store(0, data.Load(0) + 1);
    }
}
