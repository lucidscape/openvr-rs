extern crate openvr_bindgen;

use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    openvr_bindgen::generate(Path::new("openvr/headers/openvr_api.json"), &out_dir.join("ffi.rs")).expect("failed to generate bindings");
}
