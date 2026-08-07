set -x

xcrun metal -o /tmp/triangle.ir -c examples/shaders/src/triangle.metal
xcrun metal -o examples/shaders/triangle.metallib /tmp/triangle.ir
rm /tmp/triangle.ir
