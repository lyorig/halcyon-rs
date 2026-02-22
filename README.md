# halcyon-rs

This is an attempt at porting my [Halcyon](https://github.com/lyorig/halcyon) C++ SDL3 wrapper to Rust.
As I'm primarily a C++ developer, this library is currently **not safe**. My primary vision is getting as close to a zero-cost wrapper as possible, even if that means offloading safety guarantees to the library consumer.

## Concepts
SDL works with raw pointers and ownership rules are mostly described via function documentation, halcyon-rs aims
to disambiguate with *handles*, *owned objects* and *reference objects*. The latter two objects both consist of only a handle;
the only differences are that owned objects implement `Drop`, and references are tied to a lifetime of an owned object.

> [!NOTE]
> The reason for custom reference objects instead of using Rust's built-in references is simple: this would cause unnecessary
> double indirection, since SDL objects are only exposed via pointers (with the exception of `SDL_Surface`, for some reason),
> so it's more efficient to pass them around just like you would in C, yet ensuring lifetime safety via Rust's mechanics.
> If you see a method returning a handle, it'll most likely be marked `unsafe`, since I couldn't find a way to explain its lifetime
> via the language itself.
