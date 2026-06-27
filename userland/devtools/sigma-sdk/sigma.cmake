# SPDX-License-Identifier: GPL-2.0-or-later
# sigma.cmake — CMake toolchain file for cross-compiling to SigmaOS
#
# Usage:
#   cmake -DCMAKE_TOOLCHAIN_FILE=/path/to/sigma.cmake -B build
#   cmake --build build
#
# The sysroot contains SigmaOS headers + musl libc.
# Binaries produced are ELF64 compatible with SigmaOS's Linux-compatible ABI.

set(CMAKE_SYSTEM_NAME      SigmaOS)
set(CMAKE_SYSTEM_PROCESSOR x86_64)

# ── Toolchain ─────────────────────────────────────────────────────────────────
# Prefer Clang for CFI + SafeStack support; fall back to GCC cross-compiler
find_program(SIGMA_CLANG  clang)
find_program(SIGMA_GCC    x86_64-sigma-elf-gcc)

if(SIGMA_CLANG)
    set(CMAKE_C_COMPILER   ${SIGMA_CLANG})
    set(CMAKE_CXX_COMPILER ${SIGMA_CLANG}++)
    set(SIGMA_TOOLCHAIN    "clang")
else()
    set(CMAKE_C_COMPILER   ${SIGMA_GCC})
    set(CMAKE_CXX_COMPILER x86_64-sigma-elf-g++)
    set(SIGMA_TOOLCHAIN    "gcc")
endif()

# ── Sysroot ───────────────────────────────────────────────────────────────────
set(SIGMA_SDK_DIR   "${CMAKE_CURRENT_LIST_DIR}" CACHE PATH "SigmaOS SDK root")
set(SIGMA_SYSROOT   "${SIGMA_SDK_DIR}/sysroot"  CACHE PATH "SigmaOS sysroot")
set(CMAKE_SYSROOT   ${SIGMA_SYSROOT})
set(CMAKE_FIND_ROOT_PATH ${SIGMA_SYSROOT})
set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)
set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)

# ── Target flags ─────────────────────────────────────────────────────────────
# Target Linux x86_64 ABI (syscall numbers match Linux — musl just works)
if(SIGMA_TOOLCHAIN STREQUAL "clang")
    set(SIGMA_TARGET_FLAGS
        "--target=x86_64-unknown-linux-musl"
        "--sysroot=${SIGMA_SYSROOT}"
    )
else()
    set(SIGMA_TARGET_FLAGS "--sysroot=${SIGMA_SYSROOT}")
endif()

add_compile_options(${SIGMA_TARGET_FLAGS})
add_link_options(${SIGMA_TARGET_FLAGS})

# ── Include paths ─────────────────────────────────────────────────────────────
include_directories(
    "${SIGMA_SYSROOT}/usr/include"
    "${SIGMA_SDK_DIR}/../klib/include"   # sigma_kernel_types.h etc.
    "${SIGMA_SDK_DIR}/../userland/ipc"   # sigma_bus.h
)

# ── Default hardening (can be overridden) ─────────────────────────────────────
include("${SIGMA_SDK_DIR}/../../cmake/sigma_hardening.cmake" OPTIONAL)

# ── Convenience macros ────────────────────────────────────────────────────────
# sigma_app(name SOURCES ...) — creates a standard SigmaOS application
macro(sigma_app _name)
    cmake_parse_arguments(_APP "" "" "SOURCES;LIBS" ${ARGN})
    add_executable(${_name} ${_APP_SOURCES})
    target_link_libraries(${_name} PRIVATE sigma-bus sigma-fonts ${_APP_LIBS})
    if(COMMAND sigma_apply_hardening)
        sigma_apply_hardening(${_name})
    endif()
    install(TARGETS ${_name} DESTINATION sigma/bin)
endmacro()

# sigma_driver(name SOURCES ...) — creates a SDF driver binary
macro(sigma_driver _name)
    cmake_parse_arguments(_DRV "" "" "SOURCES" ${ARGN})
    add_executable(${_name} ${_DRV_SOURCES})
    target_link_libraries(${_name} PRIVATE sigma-sdf-uapi)
    set_target_properties(${_name} PROPERTIES SUFFIX ".smod")
    install(TARGETS ${_name} DESTINATION sigma/drivers)
endmacro()

message(STATUS "[sigma-sdk] Toolchain: ${SIGMA_TOOLCHAIN}")
message(STATUS "[sigma-sdk] Sysroot:   ${SIGMA_SYSROOT}")
