cmake_minimum_required(VERSION 3.10)
project(hardware)

# Define the verilator library to build
find_package(verilator HINTS $ENV{VERILATOR_ROOT})

set(TARGET Vtop)

add_library(${TARGET} ${CMAKE_CURRENT_SOURCE_DIR}/cpp/top.cpp)

verilate(${TARGET} SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/verilog/top.sv ${CMAKE_CURRENT_SOURCE_DIR}/verilog/boolean_gate/nand.sv TOP_MODULE top DIRECTORY "$ENV{OUT_DIR}/verilator")

install (TARGETS ${TARGET} DESTINATION .)
