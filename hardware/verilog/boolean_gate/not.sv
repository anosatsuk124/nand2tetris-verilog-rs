`ifndef INCLUDE_NOT
`define INCLUDE_NOT
`include "nand.sv"

module Not (input a, output out);

wire b;

Nand nand1 (a, a, b);

assign out = b;

endmodule
`endif
