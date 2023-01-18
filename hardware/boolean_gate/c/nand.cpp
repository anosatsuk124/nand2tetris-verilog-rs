#pragma once
#include "Vnand.h"

Vnand *top = new Vnand;

Vnand* create() {
    return new Vnand;
}