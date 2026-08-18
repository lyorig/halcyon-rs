//! Implementation checklist ([source](https://wiki.libsdl.org/SDL3/CategoryCPUInfo)):
//! - [x] SDL_GetCPUCacheLineSize
//! - [x] SDL_GetNumLogicalCPUCores
//! - [x] SDL_GetSIMDAlignment
//! - [x] SDL_GetSystemPageSize
//! - [x] SDL_GetSystemRAM
//! - [x] SDL_HasAltiVec
//! - [x] SDL_HasARMSIMD
//! - [x] SDL_HasAVX
//! - [x] SDL_HasAVX2
//! - [x] SDL_HasAVX512F
//! - [x] SDL_HasLASX
//! - [x] SDL_HasLSX
//! - [x] SDL_HasMMX
//! - [x] SDL_HasNEON
//! - [x] SDL_HasSSE
//! - [x] SDL_HasSSE2
//! - [x] SDL_HasSSE3
//! - [x] SDL_HasSSE41
//! - [x] SDL_HasSSE42
//! - [ ] SDL_HasSVE2 (will be available in SDL 3.6.0)

use std::num::NonZero;

use sdl3_sys::cpuinfo::*;

#[doc(alias = "SDL_GetCPUCacheLineSize")]
pub fn cache_line_size() -> i32 {
    unsafe { SDL_GetCPUCacheLineSize() }
}

#[doc(alias = "SDL_GetNumLogicalCPUCores")]
pub fn num_logical_cpu_cores() -> i32 {
    unsafe { SDL_GetNumLogicalCPUCores() }
}

#[doc(alias = "SDL_GetSIMDAlignment")]
pub fn simd_alignment() -> usize {
    unsafe { SDL_GetSIMDAlignment() }
}

/// Returns zero if the page size cannot be determined,
/// so the returned value is wrapped in [`Option<NonZero<i32>>`].
#[doc(alias = "SDL_GetSystemPageSize")]
pub fn system_page_size() -> Option<NonZero<i32>> {
    NonZero::new(unsafe { SDL_GetSystemPageSize() })
}

#[doc(alias = "SDL_GetSystemRAM")]
pub fn system_ram_mib() -> i32 {
    unsafe { SDL_GetSystemRAM() }
}

#[doc(alias = "SDL_HasAltiVec")]
pub fn has_altivec() -> bool {
    unsafe { SDL_HasAltiVec() }
}

#[doc(alias = "SDL_HasARMSIMD")]
pub fn has_arm_simd() -> bool {
    unsafe { SDL_HasARMSIMD() }
}

#[doc(alias = "SDL_HasAVX")]
pub fn has_avx() -> bool {
    unsafe { SDL_HasAVX() }
}

#[doc(alias = "SDL_HasAVX2")]
pub fn has_avx2() -> bool {
    unsafe { SDL_HasAVX2() }
}

#[doc(alias = "SDL_HasAVX512F")]
pub fn has_avx512f() -> bool {
    unsafe { SDL_HasAVX512F() }
}

#[doc(alias = "SDL_HasLASX")]
pub fn has_lasx() -> bool {
    unsafe { SDL_HasLASX() }
}

#[doc(alias = "SDL_HasLSX")]
pub fn has_lsx() -> bool {
    unsafe { SDL_HasLSX() }
}

#[doc(alias = "SDL_HasMMX")]
pub fn has_mmx() -> bool {
    unsafe { SDL_HasMMX() }
}

#[doc(alias = "SDL_HasNEON")]
pub fn has_neon() -> bool {
    unsafe { SDL_HasNEON() }
}

#[doc(alias = "SDL_HasSSE")]
pub fn has_sse() -> bool {
    unsafe { SDL_HasSSE() }
}

#[doc(alias = "SDL_HasSSE2")]
pub fn has_sse2() -> bool {
    unsafe { SDL_HasSSE2() }
}

#[doc(alias = "SDL_HasSSE3")]
pub fn has_sse3() -> bool {
    unsafe { SDL_HasSSE3() }
}

#[doc(alias = "SDL_HasSSE41")]
pub fn has_sse4_1() -> bool {
    unsafe { SDL_HasSSE41() }
}

#[doc(alias = "SDL_HasSSE42")]
pub fn has_sse4_2() -> bool {
    unsafe { SDL_HasSSE42() }
}
