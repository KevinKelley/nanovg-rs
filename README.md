NanoVG Rust wrapper
===================

**Now with Cargo!**

This is a Rust-language wrapper for the NanoVG vector graphics library.

NanoVG is written in C, and it supports several back-ends: GL2, GL3, GLES...

NanoVG is not an extremely complete or extensive implementation of vector graphics.
It is however small and hardware-accelerated, which is what I want.

## Screenshot

![yay! screenshot works in rust demo!](/screenshot.png)

Prerequisites
=============

This build process will produce a Rust library, which includes the Rust wrapper
for nanovg functions, and which statically links in those functions.
NanoVG only does the graphics drawing, though; you'll need to be
getting a GL context from somewhere.  The examples use GLFW.

Examples require GLFW3 to be installed.

Building
========

Build using Cargo:

    git clone --recursive https://github.com/KevinKelley/nanovg-rs
    cd nanovg-rs
    cargo build

To build the demo, then:

    cd examples/demo
    cargo build
    ./target/debug/example

Note that font and image resources won't be found if you run from
inside the target directory.

In the demo,
- 'p' switches between pre- and un-premultiplied alpha;
- 's' saves a screenshot;
- and 'space' toggles scale/rotate of the pseudo-window stuff.

Usage
=====

Add the following to `Cargo.toml`:

```toml
[dependencies.nanovg]
version = "*"
features = ["glX"]
```

where `glX` should be exactly one of `gl2`, `gl3`, `gles2` or `gles3`,
to specify the version of OpenGL for which NanoVG should be built.

Used By
=======

- [Blendish-rs](https://github.com/KevinKelley/blendish-rs) (Blender-themed widgets)

License
=======

The binding is licensed under [the MIT license](LICENSE.txt).
NanoVG is released under the zlib license.

Links
=====

- [Blendish](https://bitbucket.org/duangle/blendish)
- [NanoVG](https://github.com/memononen/nanovg)
- [rust-bindgen](https://github.com/crabtw/rust-bindgen) (thanks!)
- [gl-rs](https://github.com/bjz/gl-rs)
- [glfw-rs](https://github.com/bjz/glfw-rs)
