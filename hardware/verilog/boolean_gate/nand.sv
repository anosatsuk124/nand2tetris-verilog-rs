import "DPI-C" context task println(input num);

module NAND (
    input a,
    input b,
    output reg out
);

always @ (a, b)
    begin
       out = !(a & b);
    end
endmodule
