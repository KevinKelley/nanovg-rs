use std::io::Command;

fn main() {
    Command::new("make").arg("-f").arg("nanovg.mk").status().unwrap();
}
