cmake_minimum_required(VERSION 3.14)
project(SigmaOSKernelTests CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Fetch GoogleTest automatically — resolves 'gtest/gtest.h' file not found
include(FetchContent)
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/refs/tags/v1.14.0.zip
)
# Windows: prevent overriding the parent project's compiler/linker settings
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

# Include kernel headers for host-mode testing (stubs replace bare-metal ASM)
include_directories(
    ${CMAKE_SOURCE_DIR}/stubs
    ${CMAKE_SOURCE_DIR}/../../include
)

# Enable Static Analysis
find_program(CLANG_TIDY_EXE NAMES "clang-tidy")
if(CLANG_TIDY_EXE)
    set(CMAKE_CXX_CLANG_TIDY "${CLANG_TIDY_EXE};-checks=-*,readability-*,bugprone-*;--warnings-as-errors=*")
endif()

enable_testing()

add_executable(test_kernel 
    test_kernel.cpp 
    test_pqc.cpp 
    test_sandbox.cpp
    ../../kernel/core/security/SovereignPQC.cpp
    ../../kernel/core/security/SovereignSandbox.cpp
)
target_link_libraries(test_kernel GTest::gtest_main)

include(GoogleTest)
gtest_discover_tests(test_kernel)
