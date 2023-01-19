find_package(verilator HINTS $ENV{VERILATOR_ROOT})

# Define the rust library to build
if (CMAKE_BUILD_TYPE STREQUAL "Debug")
  set(CARGO_CMD cargo build)
  set(TARGET_DIR "debug")
else ()
  set(CARGO_CMD cargo build --release)
  set(TARGET_DIR "release")
endif ()

set(CARGO_SOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/rust")

set(HARDWARE_A "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libhardware.a")

add_custom_target(hardware ALL
  COMMENT "Compiling a rust hardware library"
  COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD}
  COMMAND cp ${HARDWARE_A} ${CMAKE_CURRENT_BINARY_DIR}
  WORKING_DIRECTORY ${CARGO_SOURCE_DIR})
set_target_properties(hardware PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})

add_test(NAME hardware_test
  COMMAND cargo test
  WORKING_DIRECTORY ${CARGO_SOURCE_DIR})


# Define the verilator library to build


add_executable(Vtop ${CMAKE_CURRENT_SOURCE_DIR}/cpp/top.cpp)
target_link_libraries(Vtop PRIVATE ${CMAKE_CURRENT_BINARY_DIR}/libhardware.a)
verilate(Vtop SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/verilog/top.sv ${CMAKE_CURRENT_SOURCE_DIR}/verilog/boolean_gate/nand.sv TOP_MODULE top)
