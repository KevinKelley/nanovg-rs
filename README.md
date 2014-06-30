
NanoVG Rust wrapper
===================

This is a Rust-language wrapper for the NanoVG vector-graphis library.

NanoVG is C code, and it supports several back-ends: GL2, GL3, GLES...

NanoVG is not an extremely complete or extensive implementation of vector graphics.
It is however small and hardware-accelerated, which is what I want.  If you need
all the things, look at Cairo-graphics: it's everywhere, and it's good; eventually
it'll even come with graphics card support. 


Dependencies
============
Hopefully none, for runtime.  This build process will produce a nanovg-rs library,
which includes the Rust wrapper for nanovg functions, and which statically links in
those functions.  NanoVG only does the graphics drawing, though; you'll need to be
getting a GL context from somewhere.  The examples use GLFW.

Premake4 is required for building NanoVG itself.

rust-bindgen is used to create the Rust wrapper for it.


Building
========

Controlled by the Makefile in this project root directory.

```
make get-deps
make deps
make nanovg
make examples
./bin/demo
```

License
=======
MIT, for this binding. NanoVG is released under the zlib license.

Links
=====
https://github.com/memononen/nanovg
https://github.com/crabtw/rust-bindgen