# halcyon-rs scripts

Scripts used to simplify & automate development of halcyon-rs.
Currently split into two directories: `unix` and `windows`. The main difference is that Windows
scripts are written in PowerShell, while the Unix ones are simple `.sh` files. Some scripts are
shared (such as `test.{ps1,sh}`), but others are genuinely platform-specific (`compile-{hlsl,metal}-shaders.{ps1,sh}`).

This directory layout is subject to change in the future.
