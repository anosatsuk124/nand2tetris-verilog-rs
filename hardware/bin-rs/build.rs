use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
    str,
};

fn main() {
    let cmake_dst = cmake::build("../../hardware");
    println!("cargo:rustc-link-search=native={}", cmake_dst.display());
    println!("cargo:rustc-link-lib=static=Vtop");

    let verilator_root_dir = PathBuf::from(
        str::from_utf8(
            &Command::new("verilator")
                .arg("--getenv")
                .arg("VERILATOR_ROOT")
                .output()
                .expect("Failed to execute verilator")
                .stdout,
        )
        .unwrap()
        .trim(),
    );

    let verilator_include_dir = verilator_root_dir.join("include");
    let vitstd_include_dir = verilator_include_dir.join("vltstd");
    let verilator_include_dirs = vec![verilator_include_dir, vitstd_include_dir];
    let cpp_dir = Path::new("../cpp");
    let verilog_dir = Path::new("../verilog");
    let dist_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("verilator");

    cxx_build::bridge("src/ffi.rs")
        .flag_if_supported("-std=c++14")
        .flag_if_supported("-Wno-unused-parameter")
        .include(&cpp_dir)
        .includes(&verilator_include_dirs)
        .include(&dist_dir)
        .compile("top");

    println!("cargo:rerun-if-changed={}", cpp_dir.display());
    println!("cargo:rerun-if-changed={}", verilog_dir.display());
}
