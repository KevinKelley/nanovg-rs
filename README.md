
NanoVG Rust wrapper
===================

Now with Cargo!


This is a Rust-language wrapper for the NanoVG vector-graphis library.

NanoVG is C code, and it supports several back-ends: GL2, GL3, GLES...

NanoVG is not an extremely complete or extensive implementation of vector graphics.
It is however small and hardware-accelerated, which is what I want.

## Screenshot

![yay! screenshot works in rust demo!](/dump.png?raw=true)

Dependencies
============
This build process will produce a Rust library, which includes the Rust wrapper
for nanovg functions, and which statically links in those functions.
NanoVG only does the graphics drawing, though; you'll need to be
getting a GL context from somewhere.  The examples use GLFW.

Premake4 is required for building NanoVG itself.

rust-bindgen was used to create the initial ffi binding.

gl-rs, by bjz, is used to grab the GL framebuffer for snapshots.

glfw-rs, also bjz's, is used to create the window and expose events.


Building
========

The Makefile is probably out of date now.  Build using cargo:

```
  git clone https://github.com/KevinKelley/nanovg-rs
  cd nanovg-rs
  cargo build
```

To build the demo, then:

```
  cd examples/demo
  cargo build
  ./target/example
```

(note that font and image resources won't be found if you run from
inside the target directory)

In the demo,
- 'p' switches between pre- and un-premultiplied alpha;
- 's' saves a screenshot;
- and 'space' toggles scale/rotate of the pseudo-window stuff.


Used By
=======
- [Blendish-rs](https://github.com/KevinKelley/blendish-rs) (Blender-themed widgets)

License
=======
MIT, for this binding. NanoVG is released under the zlib license.

Links
=====
- [Blendish](https://bitbucket.org/duangle/blendish)
- [NanoVG](https://github.com/memononen/nanovg)
- [rust-bindgen](https://github.com/crabtw/rust-bindgen) (thanks!)
- [gl-rs](https://github.com/bjz/gl-rs)
- [glfw-rs](https://github.com/bjz/glfw-rs)
