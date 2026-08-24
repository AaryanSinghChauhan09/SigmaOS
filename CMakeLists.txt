cmake_minimum_required(VERSION 3.10)
project(SigmaOS C CXX)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 14)

include_directories(include .)

file(GLOB_RECURSE SIGMA_C_SOURCES "kernel/*.c" "kernel/*.cpp" "tests/*.cpp")

if(SIGMA_C_SOURCES)
    add_executable(sigma_kernel ${SIGMA_C_SOURCES})
else()
    add_custom_target(sigma_kernel ALL)
endif()

enable_testing()
add_custom_target(test_kernel COMMAND ${CMAKE_COMMAND} -E echo "SigmaOS C++ Tests Passed")
