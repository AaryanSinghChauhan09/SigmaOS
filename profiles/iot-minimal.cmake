# profiles/iot-minimal.cmake — Gentoo-style USE-flag preset for minimal IoT builds
# Usage: cmake -B build -DCMAKE_TOOLCHAIN_FILE=profiles/iot-minimal.cmake

set(SIGMA_PROFILE        "iot-arm64"  CACHE STRING "Build profile" FORCE)

# Cross-compiler for ARM64
set(CMAKE_SYSTEM_NAME    Linux)
set(CMAKE_SYSTEM_PROCESSOR aarch64)
set(CMAKE_C_COMPILER     aarch64-linux-gnu-gcc   CACHE STRING "" FORCE)
set(CMAKE_CXX_COMPILER   aarch64-linux-gnu-g++   CACHE STRING "" FORCE)

# USE flags — disable heavy subsystems for IoT sensors/edge nodes
set(SIGMA_USE_HYPERVISOR   OFF CACHE BOOL "" FORCE)
set(SIGMA_USE_AI_ENGINE    OFF CACHE BOOL "" FORCE)
set(SIGMA_USE_ZENITH_DE    OFF CACHE BOOL "" FORCE)
set(SIGMA_USE_CLUSTER      OFF CACHE BOOL "" FORCE)
set(SIGMA_USE_WASM         OFF CACHE BOOL "" FORCE)
set(SIGMA_USE_BLUETOOTH    OFF CACHE BOOL "" FORCE)

# IoT nodes need PQ crypto for secure OTA updates
set(SIGMA_USE_PQ_NET       ON  CACHE BOOL "" FORCE)
set(SIGMA_USE_CRYPTFS      ON  CACHE BOOL "" FORCE)
set(SIGMA_USE_WIFI         ON  CACHE BOOL "" FORCE)
