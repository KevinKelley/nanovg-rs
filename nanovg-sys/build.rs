extern crate cc;

use std::env;
use std::path::Path;
use std::process::Command;

fn build_library(backend_macro: &str) {
    let target = env::var("TARGET").unwrap();
    let mut config = cc::Build::new();

    // Hide the nanovg warnings. Not really relevant to us.
    // cc::Build::warnings(false); // this does not disable warnings, it can be used only for enabling them
    config.flag_if_supported("-w"); // disable warnings for msvc and gcc
                                    // (msvc accepts /w or -w, gcc and clang only -w)

    config.include("nanovg/src");
    config.file("nanovg/src/nanovg.c");
    config.file("nanovg_shim.c");
    config.define(backend_macro, None);
    if target.contains("linux") {
        println!("cargo:rustc-link-lib=GL");
    } else if target.contains("darwin") {
        println!("cargo:rustc-link-lib=framework=OpenGL");
    } else if target.contains("windows") {
        config.file("glad/glad.c");
        config.include("glad");
    }

    config.compile("nanovg");
}

fn main() {
    let backend_macro = ["GL3", "GL2", "GLES3", "GLES2"]
        .iter()
        .filter(|f| env::var(format!("CARGO_FEATURE_{}", f)).is_ok())
        .map(|f| format!("NANOVG_{}_IMPLEMENTATION", f))
        .next()
        .expect(
            "Unable to determine the backend / implementation. Have you enabled one of the features?",
        );

    // Initialize nanovg submodule if user forgot to clone parent repository with --recursive.
    if !Path::new("nanovg/.git").exists() {
        let _ = Command::new("git").args(&["submodule", "update", "--init"])
                                   .status();
    }

    build_library(&backend_macro);
}
