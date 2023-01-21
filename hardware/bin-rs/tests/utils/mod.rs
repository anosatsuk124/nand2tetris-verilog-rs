use csv;
use std::{collections::HashMap, fs::File, io::Write, path::PathBuf};

pub const TEST_DATA_DIR: &str = "../../nand2tetris/projects";

pub const VERILOG_DIR: &str = "../verilog";

pub fn create_test_top(top_module: String, top_name: &str) -> PathBuf {
    let dst = PathBuf::from(env!("OUT_DIR")).join(top_name.to_owned() + ".sv");
    let mut f = File::create(&dst).unwrap();
    f.write(top_module.as_bytes()).unwrap();
    dst
}

pub fn parse_cmp_file(file: PathBuf) -> Vec<HashMap<String, String>> {
    let f = File::open(file).unwrap();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'|')
        .trim(csv::Trim::All)
        .from_reader(f);
    reader.deserialize().map(|r| r.unwrap()).collect()
}
