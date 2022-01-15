cmake_minimum_required(VERSION 3.18 FATAL_ERROR)

project(staggrid_c_api LANGUAGES C)

find_library(STAGGRID_LIB staggrid_c_api
    PATHS "../../target/debug" "../target/release"
    REQUIRED)

add_executable(grid grid.c)
target_link_libraries(grid ${STAGGRID_LIB})

enable_testing()
add_test(NAME grid COMMAND grid)
