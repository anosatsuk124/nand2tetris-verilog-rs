cmake_minimum_required(VERSION 3.10)
project(hardware)

# Define the verilator library to build
find_package(verilator HINTS $ENV{VERILATOR_ROOT})

set(TARGET "$ENV{VERILOG_TOP}")

add_library(${TARGET} ${CMAKE_CURRENT_SOURCE_DIR}/cpp/$ENV{VERILOG_TOP}.cpp)

verilate(${TARGET} SOURCES r{CMAKE_CURRENT_SOURCE_DIR}/verilog/boolean_gate/NAND.sv ${CMAKE_CURRENT_SOURCE_DIR}/verilog/TOP.sv  TOP_MODULE "$ENV{VERILOG_TOP}" DIRECTORY "$ENV{OUT_DIR}/verilator")

install (TARGETS ${TARGET} DESTINATION .)
