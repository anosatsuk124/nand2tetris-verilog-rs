use std::{path::PathBuf};
mod utils;

use icarus_verilog_testbench::IVerilogTest;

fn nand(a: u32, b: u32) -> u32 {
    let test_top = format!("
module NandTest;
   reg a, b;
   wire c;
   Nand nand1 (a, b, c);
   initial begin
      a = {a};
      b = {b};
        #1;
      $display(\"%d\", c);
    end
endmodule
");
    let top_name = "NandTest";
    let dst = utils::create_test_top(test_top, top_name);
    let test = IVerilogTest::builder()
        .top(top_name)
        .paths(vec![PathBuf::from("../verilog/boolean_gate/nand.sv")])
        .path(dst)
        .build();
    let test = test.test(&PathBuf::from(env!("OUT_DIR")).join("nand_test"));
    test.unwrap().trim_end().parse::<u32>().unwrap()
}

#[test]
fn test_nand() {
    assert_eq!(nand(0, 0), 1);
    assert_eq!(nand(0, 1), 1);
    assert_eq!(nand(1, 0), 1);
    assert_eq!(nand(1, 1), 0);
}
