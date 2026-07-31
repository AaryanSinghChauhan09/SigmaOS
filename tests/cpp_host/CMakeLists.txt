cmake_minimum_required(VERSION 3.10)
project(cpp_host_tests)

set(CMAKE_CXX_STANDARD 17)

add_executable(host_tests test_host.cpp)

add_custom_target(test_kernel DEPENDS host_tests)

enable_testing()
add_test(NAME host_tests COMMAND host_tests)
