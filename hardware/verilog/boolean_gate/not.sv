module Not (input a, output out);

wire b;

Nand nand1 (a, a, b);

assign out = b;

endmodule
