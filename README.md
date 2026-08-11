# halcyon-rs

An sdl3-sys wrapper. Aims for as close to 100% API coverage, while making it neater & safer to use via various Rust mechanisms (see _Enhancements_).
As I'm primarily a C++ developer, this library is probably unsound in various places. These ought to be weeded out over time after reaching full API coverage.

## Important

This is purely an API wrapper. It isn't concerned with how you find, and link with, SDL on your system;
these topics are covered by the [sdl3-sys docs](https://docs.rs/sdl3-sys/latest/sdl3_sys/).

## Concepts

SDL works with raw pointers and ownership rules are mostly described via function documentation. halcyon-rs aims
to disambiguate with _handles_, _owned objects_ and _references_. For an arbitrary type `Foo`:
- The handle `FooHandle` is where the API is actually implemented. Since it isn't tied to anything, it's usually unsafe to use.
- The owned object `Foo` contains a handle and is responsible for `Drop`ping it.
- The reference `Ref<'a, Foo>` contains a handle, is lifetime-bound to an owned object, and doesn't drop anything.

## Enhancements

In an attempt to justify the time spent on this project, here is a list of things that, in my eyes,
make halcyon-rs much neater to use over raw SDL bindings:

- `Drop` impl'd where applicable[^1]
- `Ref` for borrowing opaque handles without extra indirection
- `halcyon::Result<T>` instead of `SDL_GetError()`
- `Box` and `String` for mapping SDL allocations to Rust
- Descriptive bool-enums instead of bool parameters
- Builders wrapping the [Properties API](https://wiki.libsdl.org/SDL3/CategoryProperties)
- Encapsulation of reserved struct parameters

[^1]: Certain SDL_gpu objects are an exception, since their destructors require an extra parameter. Such objects have a "manual" `drop()` method, and have proper documentation of this fact.
