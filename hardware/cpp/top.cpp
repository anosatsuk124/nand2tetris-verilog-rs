#include <memory>
#include "Vtop.h"

std::unique_ptr<Vtop> new_top(int a, int b) {
    std::unique_ptr<Vtop> top(new Vtop);
    top->a = a;
    top->b = b;
    return top;
}

void eval_top(const std::unique_ptr<Vtop>& top) {
    top->eval();
}

int get_out(const std::unique_ptr<Vtop>& top) {
    return top->out;
}
