# FFI bindings to libuv for Rust

> This crate is still in early state : do not use in production

My goal with this repo is twofold :

- Implementing my own take of a `*-sys` wrapper exposing the [libuv](https://github.com/libuv/libuv) library to Rust programs / libraries
- Tinkering with the build system to get a better insight at the process of porting CMake build scripts to Cargo ones

Once the release process has been fully automated, I intend to publish versions of this package following the same numbering scheme as libuv.
