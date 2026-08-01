# halcyon-rs

An SDL3[^1] wrapper. Aims for as close to 100% API coverage, while making it neater & safer to use via various Rust mechanisms (i.e. lifetimes).
As I'm primarily a C++ developer, this library is probably unsound in various places. These ought to be weeded out over time after reaching full API coverage.

## Concepts
SDL works with raw pointers and ownership rules are mostly described via function documentation, halcyon-rs aims
to disambiguate with *handles*, *owned objects* and *references*. For an arbitrary type `Foo`:
- The handle `FooHandle` is where the API is actually implemented.
- The owned object `Foo` contains a handle and is responsible for `Drop`ping it.
- The reference `Ref<'a, Foo>` contains a handle, is lifetime-bound to an owned object, and doesn't drop anything.

[^1]: Technically [sdl3-sys](https://docs.rs/sdl3-sys/latest/sdl3_sys/), since it's nice to use existing bindings.
