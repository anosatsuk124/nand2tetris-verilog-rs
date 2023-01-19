use std::{env, path::PathBuf};
fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let dst = cmake::build("../../hardware");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=Vtop");
    println!("cargo:rustc-link-search=native={}", PathBuf::from(env::var("OUT_DIR").unwrap()).display());
    println!("cargo:rustc-link-lib=static=lib_rs");

    // println!("cargo:rustc-link-lib=dylib=stdc++");

    cxx_build::bridge("src/main.rs")
        .flag_if_supported("-std=c++14")
        .include("../cpp")
        .include(PathBuf::from(env::var("OUT_DIR").unwrap()).join("verilator").canonicalize().unwrap())
        .include("/opt/homebrew/Cellar/verilator/5.004/share/verilator/include")
        .include("/opt/homebrew/Cellar/verilator/5.004/share/verilator/include/vltstd")
        .compile("top");
}
