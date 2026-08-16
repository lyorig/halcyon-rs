set -e

# Check that `dxc` is available on PATH.
if ! command -v dxc >/dev/null 2>&1; then
    echo "\`dxc\` not found. Exiting."
    exit 1
fi


# `examples/gpu.rs`
dxc -spirv -T vs_6_0 -E vs_main examples/shaders/src/triangle.hlsl -Fo examples/shaders/triangle_vs.spv
dxc -spirv -T ps_6_0 -E fs_main examples/shaders/src/triangle.hlsl -Fo examples/shaders/triangle_fs.spv

# `examples/teapot.rs`
dxc -spirv -T vs_6_0 -E vs_main examples/shaders/src/teapot.hlsl -Fo examples/shaders/teapot_vs.spv
dxc -spirv -T ps_6_0 -E fs_main examples/shaders/src/teapot.hlsl -Fo examples/shaders/teapot_fs.spv
