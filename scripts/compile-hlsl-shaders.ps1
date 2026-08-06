<#
    .SYNOPSIS
    Compiles HLSL source files into executable IR.

    .DESCRIPTION
    Compiles HLSL shaders for use with halcyon-rs' GPU example on Windows.
    Requires MSVC build tools. Changing the alias value may be necessary.
    To find where dxc is on your system, open the Native Tools (or Developer) Command Prompt,
    and run "which dxc". Replace the path following "-Value" with the command output.
#>

Set-Alias -Name dxc -Value "C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\dxc.exe"

dxc -T vs_6_0 -E vs_main examples/shaders/src/triangle.hlsl -Fo examples/shaders/triangle_vs.dxil
dxc -T ps_6_0 -E fs_main examples/shaders/src/triangle.hlsl -Fo examples/shaders/triangle_fs.dxil
