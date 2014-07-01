#![crate_type = "lib"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![crate_id = "github.com/KevinKelley/nanovg-rs#nanovg:0.1"]
#![comment = "Binding for NanoVG vector-graphics library"]

#![warn(non_camel_case_types)]
#![deny(unnecessary_parens)]
#![deny(non_uppercase_statics)]
#![deny(unnecessary_qualification)]
//#![warn(missing_doc)] // FIXME: should be denied.
#![deny(unused_result)]
#![deny(unnecessary_typecast)]
#![warn(visible_private_types)] // FIXME: should be denied.
//#![feature(globs)]
//#![feature(macro_rules)]
//#![feature(managed_boxes)]
//#![feature(unsafe_destructor)]
#![doc(html_root_url = "https://github.com/KevinKelley/nanovg-rs")]

extern crate libc;

pub mod nanovg;

