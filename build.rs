extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    let output = Command::new("xcrun")
        .args(["--show-sdk-path"])
        .output()
        .unwrap()
        .stdout;
    let sdk = String::from_utf8(output).unwrap();
    let sdk = sdk.trim().to_string();

    let bindings = bindgen::Builder::default()
        .use_core()
        .blocklist_type("embedded_panic_header")
        .blocklist_type("embedded_panic_header__bindgen_ty_1")
        .header("wrapper.h")
        .clang_arg(format!(
            "-I{}/System/Library/Frameworks/Kernel.framework/Versions/Current/Headers/",sdk
        ))
        .generate()
        .unwrap();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
}
