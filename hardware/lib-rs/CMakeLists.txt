# Define the rust library to build
if (CMAKE_BUILD_TYPE STREQUAL "Debug")
  set(CARGO_CMD cargo build -p lib-rs)
  set(TARGET_DIR "debug")
else ()
  set(CARGO_CMD cargo build --release -p lib-rs)
  set(TARGET_DIR "release")
endif ()

set(CARGO_SOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../lib-rs")

set(LIB_RS "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/liblib_rs.a")

add_custom_target(lib_rs ALL
  COMMENT "Compiling a rust hardware library"
  COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD}
  COMMAND cp ${LIB_RS} $ENV{OUT_DIR}
  WORKING_DIRECTORY ${CARGO_SOURCE_DIR})

add_test(NAME lib_rs_test
  COMMAND cargo test
  WORKING_DIRECTORY ${CARGO_SOURCE_DIR})
