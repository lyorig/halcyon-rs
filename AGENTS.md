# Project description

halcyon-rs is a Rust wrapper for the Simple DirectMedia Layer (SDL) library.
It combines the OS video and audio abstractions of SDL with the compile-time safety of Rust.

# General coding rules

- Zero-cost wrapping is a priority. Safety comes second. Do not add safety abstractions at the expense of performance.
- Write self-documenting code. Comment only when the purpose is hard to deduce.
- Comments must help new contributors understand the codebase.
- Mark functions `const` whenever possible.
- Each wrapper module starts with a commented `[x]` or `[ ]` checklist. This checklist tracks SDL API coverage. It comes from the official SDL wiki. For example, `src/surface.rs` contains a checklist with functions listed in [CategorySurface](https://wiki.libsdl.org/SDL3/CategorySurface/raw). If a module cannot map 1:1 to a SDL wiki category (for example, `src/window.rs` is a subset of [CategoryVideo](https://wiki.libsdl.org/SDL3/CategoryVideo/raw)), then only relevant functions go in the module checklist.

# Architecture

SDL works with raw pointers. halcyon-rs bridges this gap with three tiers.
The types in parentheses below illustrate the names generated
by the `resource!` macro (see below) for an arbitrary type `Foo`.

- **Handle** (`FooHandle`)
  - `NonNull` wrapper
  - Is `Copy`
  - All methods are implemented here
- **Owned object** (`Foo`)
  - Wraps a handle
  - Implements `Drop` and `Deref<Target = Handle>`
- **Reference** (`Ref<'a, Foo>`)
  - Wraps a handle
  - Is `Copy` and `Deref<Target = Handle>`
  - Tied to a lifetime through `PhantomData`
  - Produced by `Resource::as_ref()`
  - Avoids double indirection compared to `&Handle`

Raw handle returns from methods are `unsafe`. Their lifetime cannot be expressed in Rust's type system.

## Wrapping a new SDL object

Use the `resource!` macro to generate the handle and owned struct boilerplate.
It has several flavors, each for a specific situation.

- `resource!(Type, Library, Destructor)`
  - Owned resource with `Drop`
  - Delegated to by the one-argument overload (see below)
  - Only use directly when the type comes from a satellite library (SDL_mixer, SDL_ttf), or has a non-standard destructor name, such as `TTF_CloseFont`
- `resource!(Type)`
  - Assumes the SDL type is called `SDL_Type`, and its destructor `SDL_DestroyType`
  - Convenience overload for most SDL objects, where this holds true
- `resource_no_drop!(Type)`
  - Used when destructors need parameters (for example, `SDL_GPUBuffer` with [`SDL_ReleaseGPUBuffer`](https://wiki.libsdl.org/SDL3/SDL_ReleaseGPUBuffer/raw))
  - Does not implement `Drop`
  - A custom `fn drop(self, ...)` must be implemented manually on the owned object
- `resource_tied!(Type, Library, Destructor, TiedType)`
  - Adds lifetime `'a` tied to another type (for example, `ttf::Font<'a>` tied to `ttf::Context` which has lifetime `'a`)

In all cases, the `Resource` trait is implemented. Among other things, it enables conversion to a reference through the `as_ref` method.

# Error handling

- `halcyon::Result<T>` is `Result<T, Error>`.
- On failure, `Error::current()` reads `SDL_GetError()`.
- `to_result(bool)` wraps the common pattern: check the bool return. Return `Err(Error::current())` on false.
- `SdlString::from_ptr(ptr)` returns `halcyon::Result<SdlString>`. It handles null pointers.

# Naming conventions

- `xchg_*`: set a value and return the old one (exchange pattern).
- `_with` suffix: temporarily set a property, do an action, and restore the original.
- `opt2ptr()` or `opt2ptr_mut()`: convert `Option<&T>` to a nullable C pointer.
- `#[doc(alias = "SDL_FunctionName")]` on every SDL function wrapper. **This is mandatory.**

# Event system

`Event` is `#[repr(C, u32)]`. It mirrors the layout of `SDL_Event`. Conversion uses raw pointer manipulation with a `DISCRIMINANT_OFFSET` to account for the Rust enum discriminant. `EventIter` implements `Iterator + FusedIterator`. It is a zero-sized type. It calls `SDL_PollEvent` on each `next()`.

# Testing

Tests live in `test/` and not in `src/`. SDL needs the main thread. The default Cargo harness is disabled. [rustest](https://docs.rs/rustest/latest/rustest/) replaces it.

## Creating a test

1. Find the correct module for the test. For example, tests for the `Color` struct go in `test/color.rs`.
2. Create a function with the `#[rustest::test]` attribute.

## Running tests

Run `scripts/test.sh`. This wraps `cargo test` with options specific to this crate.
