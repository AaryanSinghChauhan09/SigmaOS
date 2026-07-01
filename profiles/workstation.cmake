# profiles/workstation.cmake — Gentoo-style USE-flag preset for full workstation builds
# Usage: cmake -B build -DCMAKE_TOOLCHAIN_FILE=profiles/workstation.cmake

set(SIGMA_PROFILE "standalone" CACHE STRING "Build profile" FORCE)

set(SIGMA_USE_HYPERVISOR   ON  CACHE BOOL "" FORCE)
set(SIGMA_USE_AI_ENGINE    ON  CACHE BOOL "" FORCE)
set(SIGMA_USE_ZENITH_DE    ON  CACHE BOOL "" FORCE)
set(SIGMA_USE_CLUSTER      OFF CACHE BOOL "" FORCE)
set(SIGMA_USE_BLUETOOTH    ON  CACHE BOOL "" FORCE)
set(SIGMA_USE_WIFI         ON  CACHE BOOL "" FORCE)
set(SIGMA_USE_CRYPTFS      ON  CACHE BOOL "" FORCE)
set(SIGMA_USE_PQ_NET       OFF CACHE BOOL "" FORCE)
set(SIGMA_USE_WASM         ON  CACHE BOOL "" FORCE)
