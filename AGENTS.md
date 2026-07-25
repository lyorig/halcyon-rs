# Project description

This is halcyon-rs, a Rust wrapper of the Simple DirectMedia Layer (SDL) library.
halcyon-rs aims to combine the various abstractions over OS video/audio facilities of SDL with the compile-time safety and ergonomics of Rust.

# Testing

Tests are not present in `src/lib.rs`, because a lot of SDL functionality requires running on the main thread. As such, the default harness (which runs on a separate thread) is disabled, and tests are instead in a standalone `test/` directory. [rustest](https://docs.rs/rustest/latest/rustest/) is used to mimic Cargo's default test behavior.
Run tests via `cargo test --tests -- --test-threads=1`. The rationale for this is:
a) there are currently no doctests,
b) running with multiple threads causes unintended failures, as mentioned before.

# General coding rules

- Follow the principle of self-documenting code; only comment on code whose purpose is hard to deduce from reading.
- The point of comments should be to help new contributors understand the codebase better.
- Mark functions/methods `const` whenever possible.
- Functions/methods wrapping a certain SDL function shall have a corresponding `#[doc(alias = ...)]` attribute.
