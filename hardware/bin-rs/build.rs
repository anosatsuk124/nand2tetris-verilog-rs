use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
    str,
};

fn main() {
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

    let top_upper = match env::var("TEST") {
        Ok(t) => {
            let top_upper = &t.to_uppercase();
            println!("cargo:rustc-cfg=top=\"{}\"", &t);
            top_upper.to_string()
        }
        Err(_) => {
            const TOP: &str = "top";
            let top_upper = TOP.to_uppercase();
            println!("cargo:rustc-cfg=top=\"{}\"", &TOP);
            top_upper
        }
    };
    env::set_var("VERILOG_TOP", &top_upper);

    let cmake_dst = cmake::build("../../hardware");

    println!("cargo:rustc-link-search=native={}", cmake_dst.display());
    println!("cargo:rustc-link-lib=static={}", top_upper);

    if top_upper != "TOP" {
        let top_lower = top_upper.to_lowercase();
        let test_file = format!("{}.rs", top_lower);
        let test_file = Path::new("tests").join(test_file);
        cxx_build::bridge(test_file)
            .flag_if_supported("-std=c++14")
            .flag_if_supported("-Wno-unused-parameter")
            .include(&cpp_dir)
            .includes(&verilator_include_dirs)
            .include(&dist_dir)
            .compile("test");
    } else {
        cxx_build::bridge("src/ffi.rs")
            .flag_if_supported("-std=c++14")
            .flag_if_supported("-Wno-unused-parameter")
            .include(&cpp_dir)
            .includes(&verilator_include_dirs)
            .include(&dist_dir)
            .compile("t");
    }

    println!("cargo:rerun-if-changed={}", cpp_dir.display());
    println!("cargo:rerun-if-changed={}", verilog_dir.display());
}
