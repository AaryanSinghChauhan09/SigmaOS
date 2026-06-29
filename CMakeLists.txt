# =========================================================================
# SIGMAOS: Root CMakeLists.txt
# =========================================================================
# Single unified mainline build with OS-specific modular targets.
#
# Usage:
#   cmake -B build -DTARGET_OS=sigma   ..   (default — Native SigmaOS)
#   cmake -B build -DTARGET_OS=ubuntu  ..   (Ubuntu / Linux compat target)
#   cmake -B build -DTARGET_OS=bsd     ..   (BSD-style target)
#
# The TARGET_OS value controls:
#   1. Preprocessor define  (-DTARGET_OS_SIGMA / _UBUNTU / _BSD)
#   2. Driver source directory added to the build
#   3. OS profile loaded at boot via config/<target>.yaml
# =========================================================================

cmake_minimum_required(VERSION 3.16)
project(SigmaOS VERSION 15.0.0 LANGUAGES C CXX ASM)

set(CMAKE_C_STANDARD   11)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# ── Reproducible builds (NixOS-inspired) ───────────────────────────────────
if(DEFINED ENV{SOURCE_DATE_EPOCH})
    add_compile_definitions(SIGMA_BUILD_TIMESTAMP=$ENV{SOURCE_DATE_EPOCH})
endif()

# ── TARGET_OS selection ───────────────────────────────────────────────────
set(SIGMA_TARGET_OS "sigma" CACHE STRING
    "Target OS format: sigma | ubuntu | bsd")
set_property(CACHE SIGMA_TARGET_OS PROPERTY STRINGS sigma ubuntu bsd)

message(STATUS "")
message(STATUS "========================================================")
message(STATUS " SigmaOS Build System  v${PROJECT_VERSION}")
message(STATUS " TARGET_OS = ${SIGMA_TARGET_OS}")
message(STATUS "========================================================")
message(STATUS "")

# Validate choice
if(NOT SIGMA_TARGET_OS MATCHES "^(sigma|ubuntu|bsd)$")
    message(FATAL_ERROR
        "[SigmaOS] Invalid TARGET_OS='${SIGMA_TARGET_OS}'. "
        "Choose one of: sigma, ubuntu, bsd")
endif()

# ── Preprocessor define & driver directory ────────────────────────────────
string(TOUPPER "${SIGMA_TARGET_OS}" _OS_UPPER)
add_compile_definitions(TARGET_OS_${_OS_UPPER})
message(STATUS "[SigmaOS] Compiler define: -DTARGET_OS_${_OS_UPPER}")

set(SIGMA_DRIVER_DIR "${CMAKE_SOURCE_DIR}/drivers/${SIGMA_TARGET_OS}")
message(STATUS "[SigmaOS] Driver directory: ${SIGMA_DRIVER_DIR}")

# ── Global include path ───────────────────────────────────────────────────
include_directories(
    ${CMAKE_SOURCE_DIR}/include
    ${CMAKE_SOURCE_DIR}/klib/include
)

# ── Freestanding kernel flags ─────────────────────────────────────────────
set(SIGMA_KERNEL_FLAGS
    -ffreestanding
    -mno-red-zone
    -mcmodel=kernel
    -fno-stack-protector
    -fno-exceptions
    -fno-rtti
    -fno-pie
    -nostdlib
    -Wall
    -Wextra
    -Werror=format-security
    -O2
)
add_compile_options(${SIGMA_KERNEL_FLAGS})

# ── USE-flag toggles (Gentoo Portage-inspired) ────────────────────────────
option(SIGMA_USE_HYPERVISOR  "Enable hypervisor/VMM subsystem"          ON)
option(SIGMA_USE_AI_ENGINE   "Enable Neural AI Engine shard"            ON)
option(SIGMA_USE_ZENITH_DE   "Enable Zenith desktop environment"        ON)
option(SIGMA_USE_CRYPTFS     "Enable Sovereign CryptFS"                 ON)
option(SIGMA_USE_PQ_NET      "Enable post-quantum TLS networking"       OFF)
option(SIGMA_USE_WASM        "Enable WASM/WASI runtime"                 OFF)
option(SIGMA_USE_BLUETOOTH   "Enable Bluetooth 5.3 stack"               ON)
option(SIGMA_USE_WIFI        "Enable Wi-Fi stack"                        ON)

foreach(_flag
    SIGMA_USE_HYPERVISOR:SIGMA_HAS_HYPERVISOR
    SIGMA_USE_AI_ENGINE:SIGMA_HAS_AI
    SIGMA_USE_CRYPTFS:SIGMA_HAS_CRYPTFS
    SIGMA_USE_PQ_NET:SIGMA_HAS_PQ_NET
    SIGMA_USE_WASM:SIGMA_HAS_WASM
    SIGMA_USE_BLUETOOTH:SIGMA_HAS_BLUETOOTH
    SIGMA_USE_WIFI:SIGMA_HAS_WIFI
)
    string(REPLACE ":" ";" _parts "${_flag}")
    list(GET _parts 0 _opt)
    list(GET _parts 1 _def)
    if(${_opt})
        add_compile_definitions(${_def})
    endif()
endforeach()

if(NOT SIGMA_USE_ZENITH_DE)
    add_compile_definitions(SIGMA_HEADLESS)
endif()

# ── Immutable root option (Bottlerocket-inspired) ─────────────────────────
option(SIGMA_IMMUTABLE_ROOT "Remount root read-only after pivot" OFF)
if(SIGMA_IMMUTABLE_ROOT)
    add_compile_definitions(SIGMA_READONLY_ROOT=1)
    message(STATUS "[SigmaOS] SIGMA_IMMUTABLE_ROOT=ON: root remounted read-only at boot.")
endif()

# ── Kernel source directories ─────────────────────────────────────────────
set(SIGMA_KERNEL_DIRS
    kernel/core
    kernel/core/memory
    kernel/core/sched
    kernel/core/syscall
    kernel/core/hal
    kernel/net
    kernel/storage
    kernel/telemetry
    init
    fs
    net
    lib/libc
)
if(SIGMA_USE_HYPERVISOR)
    list(APPEND SIGMA_KERNEL_DIRS kernel/virt)
endif()

# ── Collect source files ──────────────────────────────────────────────────
foreach(_dir ${SIGMA_KERNEL_DIRS})
    if(EXISTS "${CMAKE_SOURCE_DIR}/${_dir}")
        file(GLOB_RECURSE _srcs
            "${CMAKE_SOURCE_DIR}/${_dir}/*.c"
            "${CMAKE_SOURCE_DIR}/${_dir}/*.cpp"
            "${CMAKE_SOURCE_DIR}/${_dir}/*.asm"
        )
        list(APPEND SIGMA_SOURCES ${_srcs})
    endif()
endforeach()

# ── OS-specific driver sources ────────────────────────────────────────────
if(EXISTS "${SIGMA_DRIVER_DIR}")
    file(GLOB_RECURSE _drv_srcs
        "${SIGMA_DRIVER_DIR}/*.c"
        "${SIGMA_DRIVER_DIR}/*.cpp"
    )
    list(APPEND SIGMA_SOURCES ${_drv_srcs})
    message(STATUS "[SigmaOS] Added ${CMAKE_FILES_DIRECTORY} driver sources from ${SIGMA_DRIVER_DIR}")
else()
    message(WARNING "[SigmaOS] Driver directory ${SIGMA_DRIVER_DIR} not found — skipping.")
endif()

# ── Kernel binary ─────────────────────────────────────────────────────────
if(SIGMA_SOURCES)
    add_executable(sigmaos.bin ${SIGMA_SOURCES})
    target_link_options(sigmaos.bin PRIVATE
        -n -T ${CMAKE_SOURCE_DIR}/linker.ld
    )
    set_target_properties(sigmaos.bin PROPERTIES
        RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}"
    )
endif()

# ── Tests ─────────────────────────────────────────────────────────────────
option(SIGMA_BUILD_TESTS "Build unit/integration test suite" OFF)
if(SIGMA_BUILD_TESTS)
    enable_testing()
    add_subdirectory(tests/cpp_host)
endif()

# ── Summary ───────────────────────────────────────────────────────────────
message(STATUS "")
message(STATUS "SigmaOS configured:")
message(STATUS "  Target OS      : ${SIGMA_TARGET_OS}")
message(STATUS "  Driver Dir     : ${SIGMA_DRIVER_DIR}")
message(STATUS "  Hypervisor     : ${SIGMA_USE_HYPERVISOR}")
message(STATUS "  AI Engine      : ${SIGMA_USE_AI_ENGINE}")
message(STATUS "  Zenith DE      : ${SIGMA_USE_ZENITH_DE}")
message(STATUS "  CryptFS        : ${SIGMA_USE_CRYPTFS}")
message(STATUS "  PQ Networking  : ${SIGMA_USE_PQ_NET}")
message(STATUS "  WASM Runtime   : ${SIGMA_USE_WASM}")
message(STATUS "  Tests          : ${SIGMA_BUILD_TESTS}")
message(STATUS "")
