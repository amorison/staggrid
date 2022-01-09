cmake_minimum_required(VERSION 3.18 FATAL_ERROR)

project(call_from_c LANGUAGES C)

find_library(STAGGRID_LIB staggrid
    PATHS "../target/debug"
    REQUIRED)

add_executable(prgm prgm.c)
target_link_libraries(prgm ${STAGGRID_LIB})
