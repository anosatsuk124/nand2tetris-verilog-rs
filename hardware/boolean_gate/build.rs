use bindgen::CargoCallbacks;
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

    let obj_dir = PathBuf::from("obj_dir")
        .canonicalize()
        .expect("cannot canonicalize path");

    let static_lib = obj_dir.join("Vnand__ALL.a");
    let lib_header = obj_dir.join("Vnand.h");

    println!("cargo:rustc-link-search={}", obj_dir.to_str().unwrap());

    println!("cargo:rustc-link-lib=Vnand__ALL");

    println!("cargo:rerun-if-changed={}", lib_header.to_str().unwrap());

    const BINDINGS: &str = "bindings.rs";
    const WRAPPER: &str = "wrapper.hpp";

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(WRAPPER)
        .clang_arg(format!("-I{}", verilator_include_dir))
        .clang_arg("-std=c++11")
        .clang_args(&["-x", "c++"])
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join(BINDINGS);
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
