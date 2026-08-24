cmake_minimum_required(VERSION 3.10)
project(SigmaOS C CXX Assembly)

message(STATUS "SigmaOS Sovereign Operating System CMake Build Adapter")

# Default build target placeholder
add_custom_target(sigmaos_build
    COMMAND cargo build --release
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    COMMENT "Building SigmaOS Kernel & Sovereign Userland Subsystems via Cargo"
)
