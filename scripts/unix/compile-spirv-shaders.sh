set -e

if [[ $1 = "-h" || $1 = "--help" ]]; then
    cat << EOF
Usage: $(basename "$0") [-h]

Compiles SPIR-V sources for use with GPU examples on platforms supporting Vulkan.
After running this script, \`cargo build --example\` should succeed on Linux.

Options:
    -h, --help  Display this message and exit.
EOF
    exit
fi

# Check that `dxc` is available on PATH.
if ! command -v dxc >/dev/null 2>&1; then
    echo "\`dxc\` not found. Exiting."
    exit 1
fi


# `examples/gpu.rs`
dxc -spirv -O3 -T vs_6_0 -E vs_main examples/shaders/src/triangle.hlsl -Fo examples/shaders/triangle_vs.spv
dxc -spirv -O3 -T ps_6_0 -E fs_main examples/shaders/src/triangle.hlsl -Fo examples/shaders/triangle_fs.spv

# `examples/teapot.rs`
dxc -spirv -O3 -T vs_6_0 -E vs_main examples/shaders/src/teapot.hlsl -Fo examples/shaders/teapot_vs.spv
dxc -spirv -O3 -T ps_6_0 -E fs_main examples/shaders/src/teapot.hlsl -Fo examples/shaders/teapot_fs.spv
