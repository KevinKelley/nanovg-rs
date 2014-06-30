#![crate_id = "nanovg-rs"]
#![crate_type = "lib"]
//#![deny(non_camel_case_types)]
//#![deny(unnecessary_parens)]
//#![deny(non_uppercase_statics)]
//#![deny(unnecessary_qualification)]
//#![warn(missing_doc)] // FIXME: should be denied.
//#![deny(unused_result)]
//#![deny(unnecessary_typecast)]
//#![warn(visible_private_types)] // FIXME: should be denied.

//#![feature(globs)]
//#![feature(macro_rules)]
//#![feature(managed_boxes)]
//#![feature(unsafe_destructor)]
#![doc(html_root_url = "https://github.com/KevinKelley/nanovg-rs")]

extern crate libc;

pub mod nanovg;

