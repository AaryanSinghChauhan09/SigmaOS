cmake_minimum_required(VERSION 3.10)
project(c_host_tests C)

set(CMAKE_C_STANDARD 11)

add_executable(host_tests test_host.c)

add_custom_target(test_kernel DEPENDS host_tests)

enable_testing()
add_test(NAME host_tests COMMAND host_tests)
