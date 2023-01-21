use std::{fs::File, io::Write, path::PathBuf};

pub fn create_test_top(top_module: String, top_name: &str) -> PathBuf {
    let dst = PathBuf::from(env!("OUT_DIR")).join(top_name.to_owned() + ".sv");
    let mut f = File::create(&dst).unwrap();
    f.write(top_module.as_bytes()).unwrap();
    dst
}
