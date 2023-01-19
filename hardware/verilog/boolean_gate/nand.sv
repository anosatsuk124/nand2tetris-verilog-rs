import "DPI-C" context task println(input num);

module Nand (
    input a,
    input b,
    output reg out
);

always @ (a, b)
    begin
        if (a == b) 
            begin
                out = 0;
            end
        else
            begin
                out = 1;
            end
       println(out);
    end
endmodule
