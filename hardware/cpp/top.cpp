#include <verilated.h>
#include "Vtop.h"

int main() {
  Vtop* top = new Vtop();
  top->a = 1;
  top->b = 0;
  while (!Verilated::gotFinish()) {
    top->eval();
  }

  return 0;
}
