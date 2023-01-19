#pragma once
#include <memory>
#include "Vtop.h"

std::unique_ptr<Vtop> new_top(int a, int b);

void eval_top(const std::unique_ptr<Vtop>& top);

int get_out(const std::unique_ptr<Vtop>& top);
