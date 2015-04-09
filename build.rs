/*
At this stage this build script is just a shim to get the old build script
working. At some point they'll both need to be refactored into a single
unified build.rs
*/
#![feature(io, os)]

use std::process::Command;
use std::env;

fn main() {
    // Build nanovg
    Command::new("make").arg("-f").arg("nanovg.mk").status().unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-flags=-L native={} -l static=nanovg_shim", out_dir);
}
