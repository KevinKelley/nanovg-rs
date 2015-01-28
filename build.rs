/*
At this stage this build script is just a shim to get the old build script
working. At some point they'll both need to be refactored into a single
unified build.rs
*/

use std::io::Command;

fn main() {
    // Build nanovg
    Command::new("make").arg("-f").arg("nanovg.mk").status().unwrap();
}
