use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let src = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let src_dll = src.join("../openvr/bin/win64/openvr_api.dll").canonicalize().unwrap();
    let mut dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    dst.pop();
    dst.pop();
    dst.pop();
    let dst_dll = dst.join("openvr_api.dll");
    fs::copy(&src_dll, &dst_dll).expect("copy dll");
}
