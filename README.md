
NanoVG Rust wrapper
===================

(Work In Progress!  possibly useable)

This is a Rust-language wrapper for the NanoVG vector-graphis library.

NanoVG is C code, and it supports several back-ends: GL2, GL3, GLES...

NanoVG is not an extremely complete or extensive implementation of vector graphics.
It is however small and hardware-accelerated, which is what I want.

## Screenshot

![yay! screenshot works in rust demo!](/dump.png?raw=true)

Dependencies
============
Hopefully none, for runtime.  This build process will produce a Rust library,
which includes the Rust wrapper for nanovg functions, and which statically links in
those functions.  NanoVG only does the graphics drawing, though; you'll need to be
getting a GL context from somewhere.  The examples use GLFW.

Premake4 is required for building NanoVG itself.

rust-bindgen wass used to create the initial ffi binding.


Building
========

Controlled by the Makefile in this project root directory.

```
make get-deps  #
make deps
make nanovg
make examples
cd bin; ./example_gl3
```

In the demo,
- 'p' switches between pre- and un-premultiplied alpha;
- 's' saves a screenshot;
- and 'space' toggles scale/rotate of the pseudo-window stuff.


License
=======
MIT, for this binding. NanoVG is released under the zlib license.

Links
=====
- https://github.com/memononen/nanovg
- https://github.com/crabtw/rust-bindgen