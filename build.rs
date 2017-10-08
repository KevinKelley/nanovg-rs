extern crate gcc;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let mut config = gcc::Config::new();
    config.include("nanovg/src");
    config.include("nanovg/example");
    config.file("nanovg/src/nanovg.c");
    config.file("src/nanovg_shim.c");
    for feature in &["GL2", "GL3", "GLES2", "GLES3"] {
        if env::var(format!("CARGO_FEATURE_{}", feature)).is_ok() {
            config.define(&format!("NANOVG_{}_IMPLEMENTATION", feature), None);
        }
    }
    if target.contains("linux") {
        println!("cargo:rustc-link-lib=GL");
    }
    else if target.contains("darwin") {
        println!("cargo:rustc-link-lib=framework=OpenGL");
    }
    config.compile("libnanovg.a")
}
