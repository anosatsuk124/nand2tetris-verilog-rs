module TOP (
             input a,
             input b,
             output reg out
             );

    NAND n1 (
              .a(a),
              .b(b),
              .out(out)
              );
endmodule
