mod utils;

use icarus_verilog_testbench::IVerilogTest;
use std::path::PathBuf;

fn nand(a: &str, b: &str) -> String {
    let test_top = format!(
        "
module NandTest;
   reg a, b;
   wire c;
   Nand nand1 (a, b, c);
   initial begin
      a = {a};
      b = {b};
        #1;
      $display(\"%b\", c);
    end
endmodule
"
    );
    let top_name = "NandTest";
    let dst = utils::create_test_top(test_top, top_name);
    let test = IVerilogTest::builder()
        .top(top_name)
        .paths(vec![PathBuf::from("../verilog/boolean_gate/nand.sv")])
        .path(dst)
        .build();
    let test = test.test(&PathBuf::from(env!("OUT_DIR")).join("nand_test"));
    test.unwrap().trim_end().to_string()
}

#[test]
fn test_nand() {
    assert_eq!(nand("0", "0"), "1");
    assert_eq!(nand("0", "1"), "1");
    assert_eq!(nand("1", "0"), "1");
    assert_eq!(nand("1", "1"), "0");
}

