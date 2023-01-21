use icarus_verilog_testbench::IVerilogTest;
use std::path::PathBuf;

mod utils;

fn or(a: &str, b: &str) -> String {
    let test_top = format!(
        "
module OrTest;
   reg a, b;
   wire c;
   Or or1(a, b, c);
   initial begin
      a = {a};
      b = {b};
      #1;
      $display(\"%b\", c);
    end
endmodule
"
    );
    let top_name = "OrTest";
    let dst = utils::create_test_top(test_top, top_name);
    let verilog_dir = PathBuf::from(utils::VERILOG_DIR);
    let boolean_dir = verilog_dir.join("boolean_gate");
    let test = IVerilogTest::builder()
        .top(top_name)
        .paths(vec![
            boolean_dir.clone().join("not.sv"),
            boolean_dir.clone().join("nand.sv"),
            boolean_dir.clone().join("and.sv"),
            boolean_dir.clone().join("or.sv"),
        ])
        .path(dst)
        .build();
    let test = test.test(&PathBuf::from(env!("OUT_DIR")).join("nand_test"));
    test.unwrap().trim_end().to_string()
}

#[test]
fn and_test() {
    let test_file = PathBuf::from(utils::TEST_DATA_DIR).join("01/Or.cmp");
    for test in utils::parse_cmp_file(test_file) {
        assert_eq!(
            or(test.get("a").unwrap(), test.get("b").unwrap()),
            test.get("out").unwrap().to_owned()
        );
    }
}
