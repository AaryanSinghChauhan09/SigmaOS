# SPDX-License-Identifier: GPL-2.0-or-later
# cmake/sigma_mimalloc.cmake — mimalloc as SigmaOS default userland allocator
#
# Inspired by Chimera Linux musl + mimalloc integration (template.py pattern).
# mimalloc at MI_SECURE=2 provides:
#   - Per-thread free list sharding (prevents cross-thread heap spraying)
#   - Guard pages between allocations (detects buffer overflows)
#   - Randomized base pointer (unpredictable heap layout)
#   - CSPRNG-seeded free list ordering

option(SIGMA_USE_MIMALLOC
    "Use mimalloc as the SigmaOS userland allocator (recommended)"
    ON)

if(SIGMA_USE_MIMALLOC)
    # Locate mimalloc sources (vendored or system)
    if(EXISTS "${CMAKE_SOURCE_DIR}/third_party/mimalloc/src/alloc.c")
        set(MIMALLOC_SRC_DIR "${CMAKE_SOURCE_DIR}/third_party/mimalloc/src")
    else()
        find_path(MIMALLOC_SRC_DIR alloc.c
            PATHS /usr/src/mimalloc/src
                  /opt/mimalloc/src
            DOC   "mimalloc source directory")
    endif()

    if(NOT MIMALLOC_SRC_DIR)
        message(WARNING
            "[sigma_mimalloc] mimalloc sources not found. "
            "Clone into third_party/mimalloc/ or install mimalloc-devel. "
            "Falling back to system malloc (less secure).")
        set(SIGMA_USE_MIMALLOC OFF)
    else()
        # Build mimalloc as a static object library — Chimera EXTRA_OBJ approach
        add_library(sigma_mimalloc OBJECT
            "${MIMALLOC_SRC_DIR}/alloc.c"
            "${MIMALLOC_SRC_DIR}/alloc-aligned.c"
            "${MIMALLOC_SRC_DIR}/alloc-posix.c"
            "${MIMALLOC_SRC_DIR}/heap.c"
            "${MIMALLOC_SRC_DIR}/page.c"
            "${MIMALLOC_SRC_DIR}/segment.c"
            "${MIMALLOC_SRC_DIR}/segment-map.c"
            "${MIMALLOC_SRC_DIR}/arena.c"
            "${MIMALLOC_SRC_DIR}/random.c"     # CSPRNG for free-list randomisation
            "${MIMALLOC_SRC_DIR}/stats.c"
            "${MIMALLOC_SRC_DIR}/os.c"
            "${MIMALLOC_SRC_DIR}/init.c"
        )

        target_compile_options(sigma_mimalloc PRIVATE
            -DMI_SECURE=2              # guard pages + free-list randomisation
            -DMI_PADDING=1             # canary padding between allocations
            -DMI_TRACK_VALGRIND=0      # no Valgrind overhead in production
            -fvisibility=hidden        # Chimera "vis" flag — no exported symbols
            -O3
        )

        target_include_directories(sigma_mimalloc PUBLIC
            "${CMAKE_SOURCE_DIR}/third_party/mimalloc/include"
        )

        message(STATUS "[SigmaOS] mimalloc allocator enabled (MI_SECURE=2)")
        message(STATUS "  Source: ${MIMALLOC_SRC_DIR}")
        message(STATUS "  Security: guard pages + free-list randomisation")
    endif()
endif()

# ── Link mimalloc into all SigmaOS userland targets ────────────────────────
macro(sigma_target_use_mimalloc target)
    if(SIGMA_USE_MIMALLOC AND TARGET sigma_mimalloc)
        target_link_libraries(${target} PRIVATE sigma_mimalloc)
        target_compile_definitions(${target} PRIVATE SIGMA_MIMALLOC_ENABLED=1)
    endif()
endmacro()

# Auto-apply to known userland targets
foreach(_target IN ITEMS
    sigma-apid
    sigma-healthd
    sigma-trustd
    sigma-ds
    sigma-watchdog
    sigma-metrics
    sigma-telemetry
    sigma-cloudsync
    sigma-power
    sigma-netd
    sigma-busd
    sigma-vault
    sigma-pkg-cli
)
    if(TARGET ${_target})
        sigma_target_use_mimalloc(${_target})
    endif()
endforeach()
