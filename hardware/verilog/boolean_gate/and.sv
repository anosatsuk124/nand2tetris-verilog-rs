`ifndef INCLUDE_AND
`define INCLUDE_AND
`include "nand.sv"
`include "not.sv"

module And(input a, input b, output out);
   wire c;
   wire d;
   Nand nand1(a, b, c);
   Not not1(c, d);
   assign out = d;
endmodule
`endif
