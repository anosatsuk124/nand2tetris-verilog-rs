#include "VTOP.h"

std::unique_ptr<VTOP> new_nand(int a, int b) {
  std::unique_ptr<VTOP> top(new VTOP);
  top->a = a;
  top->b = b;
  return top;
}

void eval(const std::unique_ptr<VTOP>& top) {
  top->eval();
}

int get_out(const std::unique_ptr<VTOP>& top) {
  return top->out;
}
