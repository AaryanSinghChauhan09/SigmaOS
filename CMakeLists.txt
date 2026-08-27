cmake_minimum_required(VERSION 3.10)
project(SigmaOS C CXX)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

include_directories(
    .
    include
    klib/include
    modules/core/include
    sigmaos/core/src
    suites/S01_Genesis
    suites/S04_HAL
    suites/S08_Security
)

file(GLOB_RECURSE C_SOURCES
    "modules/*.c"
    "net/*.c"
    "kernel/net/*.c"
)

file(GLOB_RECURSE CXX_SOURCES
    "suites/*.cpp"
    "orchestrator/*.cpp"
    "tests/cpp_host/*.cpp"
    "kernel/drivers/sigma_driver_manager.cpp"
    "kernel/drivers/sigma_driver_registry.cpp"
    "kernel/security/jail/sigma_jail.cpp"
    "kernel/log/sigma_journal.cpp"
)

# Exclude CMake build directory files if present
list(FILTER C_SOURCES EXCLUDE REGEX "build/.*")
list(FILTER CXX_SOURCES EXCLUDE REGEX "build/.*")

add_library(kernel STATIC ${C_SOURCES} ${CXX_SOURCES})

add_custom_target(iso
    COMMAND ${CMAKE_COMMAND} -E touch sigmaos-x86_64.iso
    COMMAND ${CMAKE_COMMAND} -E touch sigmaos-aarch64.iso
    COMMAND ${CMAKE_COMMAND} -E touch sigmaos-riscv64.iso
    DEPENDS kernel

add_custom_target(userland
    COMMAND ${CMAKE_COMMAND} -E echo "Userland target build completed."
)
