#include "top.h"

std::unique_ptr<Vtop> new_top() {
    std::unique_ptr<Vtop> top(new Vtop);
    top->a = 0;
    top->b = 0;
    return top;
}

void eval_top(std::unique_ptr<Vtop> top) {
    top->eval();
}
