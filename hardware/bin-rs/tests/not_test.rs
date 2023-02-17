use icarus_verilog_testbench::IVerilogTest;
use std::path::PathBuf;

mod utils;

fn not(a: &str) -> String {
    let test_top = format!(
        "
module NotTest;
   reg a;
   wire b;
   Not not1(a, b);
   initial begin
      a = {a};
      #1;
      $display(\"%b\", b);
    end
endmodule
"
    );
    let top_name = "NotTest";
    let dst = utils::create_test_top(test_top, top_name);
    let verilog_dir = PathBuf::from(utils::VERILOG_DIR);
    let boolean_dir = verilog_dir.join("boolean_gate");
    let test = IVerilogTest::builder()
        .top(top_name)
        .include(boolean_dir.clone().to_str().unwrap())
        .paths(vec![boolean_dir.clone().join("not.sv")])
        .path(dst)
        .build();
    let test = test.test(&PathBuf::from(env!("OUT_DIR")).join("nand_test"));
    test.unwrap().trim_end().to_string()
}

#[test]
fn not_test() {
    let test_file = PathBuf::from(utils::TEST_DATA_DIR).join("01/Not.cmp");
    for test in utils::parse_cmp_file(test_file) {
        assert_eq!(
            not(test.get("in").unwrap()),
            test.get("out").unwrap().to_owned()
        );
    }
}
