/*
At this stage this build script is just a shim to get the old build script
working. At some point they'll both need to be refactored into a single
unified build.rs
*/

use std::io::Command;
use std::os;

fn main() {
    // Build nanovg
    Command::new("make").arg("-f").arg("nanovg.mk").status().unwrap();

    let out_dir = os.getenv("OUT_DIR").unwrap();
    println!("cargo:rustc-flags=-L native={} -l static=shim", out_dir);
}
