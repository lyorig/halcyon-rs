set -x

xcrun metal -o /tmp/triangle.ir -c examples/shaders/src/triangle.metal
xcrun metal -o examples/shaders/triangle.metallib /tmp/triangle.ir
rm /tmp/triangle.ir

xcrun metal -o /tmp/teapot.ir -c examples/shaders/src/teapot.metal
xcrun metal -o examples/shaders/teapot.metallib /tmp/teapot.ir
rm /tmp/teapot.ir
