extern crate gcc;

use std::env;

fn main() {
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
    config.compile("libnanovg.a")
}
