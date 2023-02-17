`ifndef INCLUDE_NAND
`define INCLUDE_NAND
module Nand (
    input  a,
    input  b,
    output out
);

   assign out = !(a & b);
endmodule

`endif 