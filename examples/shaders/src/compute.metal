// Compile with the commands in `scripts/macos/compile-metal-shaders.sh`.

#include <metal_stdlib>

using namespace metal;

kernel void cs_main(device uint *data [[buffer(0)]],
                    uint3 tid [[thread_position_in_grid]]) {
    if (tid.x == 0) {
        data[0] += 1;
    }
}
