#pragma once
#include <memory>
#include "Vtop.h"

std::unique_ptr<Vtop> new_top();

void eval_top(std::unique_ptr<Vtop> top);
