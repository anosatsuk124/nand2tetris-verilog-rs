use std::env;
use std::path::PathBuf;

fn main() {
    if !std::process::Command::new("verilator")
        .arg("-cc")
        .arg("verilog/nand.v")
        .output()
        .expect("could not spawn `verilator`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not compile with verilator");
    }

    let verilator_include_dir = "/opt/homebrew/Cellar/verilator/5.004/share/verilator/include";

    let mut b = autocxx_build::Builder::new("src/main.rs", &["obj_dir", verilator_include_dir])
        .build()
        .unwrap();
    // This assumes all your C++ bindings are in main.rs
    b.flag_if_supported("-std=c++14").compile("verilog"); // arbitrary library name, pick anything
    println!("cargo:rerun-if-changed=src/main.rs");
}
