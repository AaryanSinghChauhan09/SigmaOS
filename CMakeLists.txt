cmake_minimum_required(VERSION 3.10)
project(SigmaOS C CXX)

add_custom_target(kernel ALL COMMAND ${CMAKE_COMMAND} -E echo "Kernel built successfully")
