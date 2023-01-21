module top (
             input a,
             input b,
             output reg out
             );

    Nand n1 (
              .a(a),
              .b(b),
              .out(out)
              );
endmodule
