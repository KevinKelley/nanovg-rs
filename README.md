# NanoVG - Rust Wrapper

NanoVG-RS is a wrapper around the [NanoVG vector graphics library](https://github.com/memononen/nanovg) for the Rust programming language.

> NanoVG is small antialiased vector graphics rendering library for OpenGL. It has lean API modeled after HTML5 canvas API. It is aimed to be a practical and fun toolset for building scalable user interfaces and visualizations.

NanoVG-RS provides a fully featured, functional, high-level and Rust-idiomatic API on top of the NanoVG C-API.

# Building

We recommend grabbing the latest release from [crates.io](https://crates.io/crates/nanovg).

Alternatively, you can clone and build the library yourself:

    git clone --recursive https://github.com/KevinKelley/nanovg-rs
    cd nanovg-rs
    cargo build --features "gl3"

This library comes with a couple examples:
- very useful example called `demo-glutin`. If you want to make sure that nanovg is working on your system, clone and build this crate as shown above and run the command `cargo run --example demo-glutin --features="gl3"`. This should produce a window similar to that below.
- a clock example, 'demo-clock', because who doesn't like clocks?  And I needed to get rotation transforms working.  Run it with `cargo run --example demo-clock --features "gl3"`

**Note** that when running the examples, the needed resources might not be found if you run it without a `cargo run --example` command. Thist is just a working-directory path issue.

Usage
=====

Add the following to your `Cargo.toml`:

```toml
[dependencies.nanovg]
version = "Use the latest version from crates.io"
features = ["glX"]
```

`glX` can be exactly one of `gl2`, `gl3`, `gles2` or `gles3`,
to specify the version of OpenGL to use. Use `gl3` or `gl2` for computers and `gles3` or `gles2` for mobile devices.

**TODO: SIMPLE API GUIDE**

# Screenshots

You can see more screenshots [here](/screenshots).

![demo-ui](/screenshots/demo-ui.png)
Output of the `demo-ui` example.

# Interesting Links

- [Blendish](https://bitbucket.org/duangle/blendish)
- [NanoVG](https://github.com/memononen/nanovg)
- [gl-rs](https://github.com/bjz/gl-rs)
- [Glutin](https://github.com/tomaka/glutin)
- [glfw-rs](https://github.com/PistonDevelopers/glfw-rs)

# License and Credits

The binding is licensed under [the MIT license](LICENSE.txt).
NanoVG is released under the zlib license.

Test-font *Mechanic of the Heart* by [Agathe M.Joyce](https://www.dafont.com/agathe-m-joyce.d6546).