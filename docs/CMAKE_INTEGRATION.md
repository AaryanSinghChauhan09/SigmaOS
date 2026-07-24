# CMake Integration for SigmaOS
# Build system integration with CMake for cross-platform compilation
# Alternative to Autotools, following GNU/Linux ecosystem standards

## Overview

SigmaOS uses CMake as the primary build system for C/C++ components, providing cross-platform compilation and modern build tooling.

## Configuration

### Basic CMake Setup

```cmake
cmake_minimum_required(VERSION 3.20)
project(SigmaOS VERSION 1.0.0 LANGUAGES C CXX Rust)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# SigmaOS-specific options
option(SIGMAOS_USE_MUSL "Use musl libc instead of glibc" OFF)
option(SIGMAOS_ENABLE_HARDENING "Enable security hardening" ON)
option(SIGMAOS_STATIC_LINKING "Enable static linking" OFF)
```

### Build Profiles

```cmake
# Sigma-core profile (minimal)
if(SIGMAOS_PROFILE STREQUAL "core")
    add_definitions(-DSIGMAOS_MINIMAL)
    set(SIGMAOS_FEATURES DESKTOP=0 AI=0)
endif()

# Sigma-stable profile (fixed release)
if(SIGMAOS_PROFILE STREQUAL "stable")
    set(SIGMAOS_FEATURES DESKTOP=1 AI=1)
endif()

# Sigma-rolling profile (rolling release)
if(SIGMAOS_PROFILE STREQUAL "rolling")
    set(SIGMAOS_FEATURES DESKTOP=1 AI=1)
    add_definitions(-DSIGMAOS_ROLLING)
endif()
```

## Musl libc Integration

### Cross-Compilation Toolchain

```cmake
# Toolchain file for musl
set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_C_COMPILER musl-gcc)
set(CMAKE_CXX_COMPILER musl-g++)
set(CMAKE_FIND_ROOT_PATH /usr/local/musl)
set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)
set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)
```

### Build with Musl

```bash
# Configure with musl toolchain
cmake -DCMAKE_TOOLCHAIN_FILE=toolchain-musl.cmake \
      -DSIGMAOS_USE_MUSL=ON \
      -B build-musl

# Build
cmake --build build-musl
```

## Security Hardening

### Compiler Flags

```cmake
if(SIGMAOS_ENABLE_HARDENING)
    add_compile_options(
        -Wall -Wextra -Werror
        -fstack-protector-strong
        -D_FORTIFY_SOURCE=2
        -fPIE -fPIC
    )
    
    add_link_options(
        -pie
        -Wl,-z,relro
        -Wl,-z,now
    )
endif()
```

### Static Analysis

```cmake
# Enable clang-tidy
set(CMAKE_C_CLANG_TIDY "clang-tidy;-checks=*")
set(CMAKE_CXX_CLANG_TIDY "clang-tidy;-checks=*")

# Enable cppcheck
find_program(CPPCHECK cppcheck)
if(CPPCHECK)
    set(CMAKE_C_CPPCHECK ${CPPCHECK})
    set(CMAKE_CXX_CPPCHECK ${CPPCHECK})
endif()
```

## Package Integration

### Find Packages

```cmake
# Find required packages
find_package(PkgConfig REQUIRED)
pkg_check_modules(SIGMAOS_NET REQUIRED libnl-3.0)
pkg_check_modules(SIGMAOS_CRYPTO REQUIRED openssl)

# Link packages
target_link_libraries(sigma-core
    PRIVATE
        ${SIGMAOS_NET_LIBRARIES}
        ${SIGMAOS_CRYPTO_LIBRARIES}
)
```

### Custom Targets

```cmake
# Package target
add_custom_target(package
    COMMAND cpack -G DEB
    DEPENDS sigma-core
)

# Install target
install(TARGETS sigma-core
    RUNTIME DESTINATION bin
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
)
```

## Testing

### Unit Tests

```cmake
enable_testing()

add_executable(test_core test/test_core.c)
target_link_libraries(test_core PRIVATE sigma-core)

add_test(NAME core_test COMMAND test_core)
```

### Integration Tests

```cmake
add_custom_target(integration-test
    COMMAND python3 tests/integration/run.py
    DEPENDS sigma-core
)
```

## Cross-Compilation

### ARM64 Build

```bash
# Configure for ARM64
cmake -DCMAKE_SYSTEM_NAME=Linux \
      -DCMAKE_SYSTEM_PROCESSOR=aarch64 \
      -DCMAKE_C_COMPILER=aarch64-linux-gnu-gcc \
      -DCMAKE_CXX_COMPILER=aarch64-linux-gnu-g++ \
      -B build-arm64

cmake --build build-arm64
```

### x86_64 Build

```bash
# Configure for x86_64
cmake -DCMAKE_SYSTEM_NAME=Linux \
      -DCMAKE_SYSTEM_PROCESSOR=x86_64 \
      -DCMAKE_C_COMPILER=x86_64-linux-gnu-gcc \
      -DCMAKE_CXX_COMPILER=x86_64-linux-gnu-g++ \
      -B build-x86_64

cmake --build build-x86_64
```

## Implementation Status

| Component | CMake Support | Status |
|-----------|--------------|--------|
| Kernel build system | ✅ Complete | Full CMake integration |
| Userland tools | ✅ Complete | CMake for all tools |
| Shards | 🟡 In Progress | Partial CMake support |
| Package manager | ✅ Complete | CMake-based recipes |
| Cross-compilation | ✅ Complete | Multi-architecture support |
| Musl integration | ✅ Complete | Toolchain files included |

## Best Practices

1. **Use target-based commands**: Prefer `target_link_libraries()` over global commands
2. **Modern CMake**: Use CMake 3.20+ features for better maintainability
3. **Export targets**: Use `install(EXPORT)` for package integration
4. **Generator expressions**: Use `$<CONFIG:...>` for configuration-specific logic
5. **Package management**: Use CPack for creating distribution packages

## References

- CMake Documentation: https://cmake.org/documentation/
- CMake Best Practices: https://cliutils.gitlab.io/modern-cmake/
- Cross-Compilation Guide: https://cmake.org/cmake/help/latest/manual/cmake-toolchains.7.html
