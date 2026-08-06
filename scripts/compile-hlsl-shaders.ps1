dxc -T vs_6_0 -E vs_main examples/shaders/src/triangle.hlsl -Fo examples/shaders/triangle_vs.dxil
dxc -T ps_6_0 -E fs_main examples/shaders/src/triangle.hlsl -Fo examples/shaders/triangle_fs.dxil
