# Project description

This is halcyon-rs, a Rust wrapper of the Simple DirectMedia Layer (SDL) library.
halcyon-rs aims to combine the various abstractions over OS video/audio facilities of SDL with the compile-time safety and ergonomics of Rust.

# General coding rules

- Zero-cost wrapping is a priority, safety comes second. Do not add safety abstractions at the expense of performance.
- Self-documenting code; comment only when purpose is hard to deduce.
- Comments should help new contributors understand the codebase.
- Mark functions `const` whenever possible.

# Architecture

SDL works with raw pointers. halcyon-rs bridges this with three tiers:

- **Handle** (e.g. `WindowHandle`): a `NonNull` wrapper, `Copy + Clone`. This is where all methods are actually implemented.
- **Owned object** (e.g. `Window`, `Texture`): wraps a handle, implements `Drop` and `Deref<Target = Handle>`.
- **Reference** (`Ref<'a, T>`): wraps (and derefs to) a handle, is `Copy`, tied to a lifetime via `PhantomData`. Produced by `Resource::as_ref()`. This avoids double indirection compared to `&Handle`.

Raw handle returns from methods are `unsafe`, as their lifetime can't be expressed in Rust's type system.

## Wrapping a new SDL object

Use the `resource!` macro, which generates the handle + owned struct boilerplate.
It comes in a few flavors, each suitable for a specific situation:

| Macro                                                 | Use case                                                                                             |
| ----------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| `resource!(Type)`                                     | Owned resource with `Drop` (calls `SDL_Destroy*`)                                                    |
| `resource!(Type, Library, Destructor)`                | Diffent library and/or destructor name (i.e. `TTF_Font` and `TTF_CloseFont`)                         |
| `resource_no_drop!(Type)`                             | No `Drop`, explicit `.drop()` required. Used when destructors require parameters (e.g. GPU objects)  |
| `resource_tied!(Type, Library, Destructor, TiedType)` | Adds lifetime `'a` tied to another type (e.g. `ttf::Font<'a>` tied to `ttf::Context`)                |

## The `Resource` trait (`src/traits.rs`)

Contains shared wrapper functionality (`resource!` implements this automatically),
most importantly enabling conversion to a reference via the `as_ref` method.

## Module map

| Module          | Purpose                                                                           |
| --------------- | --------------------------------------------------------------------------------- |
| `lib.rs`        | Top-level free functions wrapping global SDL functions, type aliases              |
| `traits.rs`     | `Resource`, `BlendMode`, `ColorModU8`, `ColorModF32`                              |
| `error.rs`      | `Error` struct wrapping `SDL_GetError()`                                          |
| `sdl_string.rs` | `SdlString` (owned string freed via `SDL_free()`)                                 |
| `sdl_box.rs`    | `SdlBox<T>`, `SdlBoxArr<T>` (SDL-allocated memory wrappers)                       |
| `properties.rs` | `SDL_PropertiesID` RAII wrapper                                                   |
| `event.rs`      | `Event` enum (repr C, u32), `EventIter` wrapping `SDL_PollEvent`                  |
| `util.rs`       | `opt2ptr()`, `opt2ptr_mut()`, `to_result()`, `c_ptr_to_str()`, `resource!` macros |
| `context.rs`    | `Context` (SDL init/quit guard)                                                   |
| `subsystem.rs`  | `Subsystem<'ctx, N>` (tied to Context lifetime)                                   |
| `window.rs`     | `Window` + `WindowBuilder`                                                        |
| `renderer.rs`   | `Renderer` + `RendererBuilder` + `DrawBuilder`                                    |
| `surface.rs`    | `Surface`                                                                         |
| `texture.rs`    | `Texture`                                                                         |
| `color.rs`      | `Rgb<T>`, `Rgba<T>`, type aliases (u8/f32), `OpacityBounds` trait                 |
| `rect.rs`       | `Point<T>`, `Rect<T>`, type aliases                                               |
| `display.rs`    | `DisplayHandle` (not owned), display queries                                      |
| `keyboard.rs`   | Keyboard state, text input, key/scan code names                                   |
| `clipboard.rs`  | Clipboard get/set                                                                 |
| `msgbox.rs`     | Message box dialogs                                                               |
| `gpu/`          | GPU device, buffers, textures, shaders, pipelines, passes                         |
| `ttf/`          | TTF font rendering (`Font<'a>`, `Text`)                                           |

Each wrapper module begins with a commented `[x]` / `[ ]` checklist tracking SDL API coverage.
This is sourced from the official SDL wiki, i.e. `src/surface.rs` contains a checklist with
functions listed in [CategorySurface](https://wiki.libsdl.org/SDL3/CategorySurface).
If a module cannot be mapped 1:1 to a SDL wiki category, such as `src/window.rs` being a subset
of [CategoryVideo](https://wiki.libsdl.org/SDL3/CategoryVideo), then only relevant functions
should be included in the module checklist.

# Error handling

- `halcyon::Result<T>` = `Result<T, Error>`.
- On failure, `Error::current()` reads `SDL_GetError()`.
- `to_result(bool)` wraps the common pattern: check bool return, `Err(Error::current())` on false.
- `SdlString::from_ptr(ptr)` returns `halcyon::Result<SdlString>`, handling null pointers.

# Naming conventions

- `xchg_*`: set a value and return the old one (exchange pattern).
- `_with` suffix: temporarily set a property, perform action, restore original.
- `opt2ptr()` / `opt2ptr_mut()`: `Option<&T>` → nullable C pointer.
- `#[doc(alias = "SDL_FunctionName")]` on every SDL function wrapper. **Mandatory.**

# Event system

`Event` is `#[repr(C, u32)]`, mirroring `SDL_Event`'s layout. Conversion uses raw pointer manipulation with a `DISCRIMINANT_OFFSET` to account for the Rust enum discriminant. `EventIter` implements `Iterator + FusedIterator`. It is a zero-sized type, only calling `SDL_PollEvent` on each `next()`.

# Testing

Tests live in `test/` (not `src/`), because SDL requires the main thread.
The default Cargo harness is disabled; [rustest](https://docs.rs/rustest/latest/rustest/) replaces it.

## Creating a test

1. Find an appropriate module for the test (i.e. tests related to the `Color` struct are located in `test/color.rs`).
2. Create a function decorated with `#[rustest::test]`.

## Running tests

Run `scripts/test.sh`, which wraps `cargo test` with options specific to this crate.
