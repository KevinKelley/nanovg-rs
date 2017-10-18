extern crate gcc;

use std::env;

fn build_library(backend_macro: &str) {
    let target = env::var("TARGET").unwrap();
    let mut config = gcc::Build::new();
    config.include("nanovg/src");
    // config.include("nanovg/example");
    config.file("nanovg/src/nanovg.c");
    config.file("src/nanovg_shim.c");
    config.define(backend_macro, None);
    if target.contains("linux") {
        println!("cargo:rustc-link-lib=GL");
    } else if target.contains("darwin") {
        println!("cargo:rustc-link-lib=framework=OpenGL");
    }
    config.compile("libnanovg.a")
}

fn main() {
    let backend_macro = ["GL3", "GL2", "GLES3", "GLES2"]
    	.iter()
    	.filter(|f| env::var(format!("CARGO_FEATURE_{}", f)).is_ok())
    	.map(|f| format!("NANOVG_{}_IMPLEMENTATION", f))
    	.next()
    	.expect("Unable to determine the backend / implementation. Have you enabled one of the features?");

    build_library(&backend_macro);
}
