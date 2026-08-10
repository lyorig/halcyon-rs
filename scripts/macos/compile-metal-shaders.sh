set -e

# Check that `xcrun` is available on PATH. We don't check for the Metal Toolchain,
# since the utility prints readable errors in case it's missing.
if ! command -v xcrun >/dev/null 2>&1; then
    echo "\`xcrun\` not found. Make sure that you have the Xcode Command Line Tools installed (\`xcode-select --install\`)."
    exit 1
fi


# examples/gpu.rs
xcrun metal -o /tmp/triangle.ir -c examples/shaders/src/triangle.metal
xcrun metal -o examples/shaders/triangle.metallib /tmp/triangle.ir
rm /tmp/triangle.ir

# examples/teapot.rs
xcrun metal -o /tmp/teapot.ir -c examples/shaders/src/teapot.metal
xcrun metal -o examples/shaders/teapot.metallib /tmp/teapot.ir
rm /tmp/teapot.ir
